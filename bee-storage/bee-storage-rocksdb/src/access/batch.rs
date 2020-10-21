// Copyright 2020 IOTA Stiftung
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with
// the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on
// an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use bee_crypto::ternary::Hash;
use bee_protocol::{tangle::MessageMetadata, MilestoneIndex};
use bee_storage::{
    access::{ApplyBatch, Batch, BatchBuilder},
    persistable::Persistable,
};

use crate::{access::OpError, storage::*};

pub struct StorageBatch<'a> {
    storage: &'a Storage,
    batch: WriteBatch,
    key_buf: Vec<u8>,
    value_buf: Vec<u8>,
}

#[async_trait::async_trait]
impl<'a> ApplyBatch for StorageBatch<'a> {
    type E = OpError;
    async fn apply(self, durability: bool) -> Result<(), Self::E> {
        let mut write_options = WriteOptions::default();
        write_options.set_sync(false);
        write_options.disable_wal(!durability);
        self.storage.inner.write_opt(self.batch, &write_options)?;
        Ok(())
    }
}

impl<'a> Batch<'a> for Storage {
    type BatchBuilder = StorageBatch<'a>;
    fn create_batch(&'a self) -> Self::BatchBuilder {
        StorageBatch {
            storage: self,
            batch: WriteBatch::default(),
            key_buf: Vec::new(),
            value_buf: Vec::new(),
        }
    }
}

impl<'a> BatchBuilder<'a, Storage, Hash, MessageMetadata> for StorageBatch<'a> {
    type Error = OpError;
    fn try_insert(mut self, hash: &Hash, transaction_metadata: &MessageMetadata) -> Result<Self, (Self, Self::Error)> {
        let hash_to_metadata = self.storage.inner.cf_handle(TRANSACTION_HASH_TO_METADATA).unwrap();
        self.key_buf.clear();
        self.value_buf.clear();
        hash.write_to(&mut self.key_buf);
        transaction_metadata.write_to(&mut self.value_buf);
        self.batch
            .put_cf(&hash_to_metadata, self.key_buf.as_slice(), self.value_buf.as_slice());
        Ok(self)
    }
    fn try_delete(mut self, hash: &Hash) -> Result<Self, (Self, Self::Error)> {
        let hash_to_metadata = self.storage.inner.cf_handle(TRANSACTION_HASH_TO_METADATA).unwrap();
        self.key_buf.clear();
        hash.write_to(&mut self.key_buf);
        self.batch.delete_cf(&hash_to_metadata, self.key_buf.as_slice());
        Ok(self)
    }
}

impl<'a> BatchBuilder<'a, Storage, Hash, MilestoneIndex> for StorageBatch<'a> {
    type Error = OpError;
    fn try_insert(mut self, hash: &Hash, milestone_index: &MilestoneIndex) -> Result<Self, (Self, Self::Error)> {
        let ms_hash_to_ms_index = self.storage.inner.cf_handle(MILESTONE_HASH_TO_INDEX).unwrap();
        self.key_buf.clear();
        self.value_buf.clear();
        hash.write_to(&mut self.key_buf);
        milestone_index.write_to(&mut self.value_buf);
        self.batch
            .put_cf(&ms_hash_to_ms_index, self.key_buf.as_slice(), self.value_buf.as_slice());
        Ok(self)
    }

    fn try_delete(mut self, hash: &Hash) -> Result<Self, (Self, Self::Error)> {
        let ms_hash_to_ms_index = self.storage.inner.cf_handle(MILESTONE_HASH_TO_INDEX).unwrap();
        self.key_buf.clear();
        hash.write_to(&mut self.key_buf);
        self.batch.delete_cf(&ms_hash_to_ms_index, self.key_buf.as_slice());
        Ok(self)
    }
}
