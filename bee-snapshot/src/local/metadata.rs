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

use std::collections::HashMap;

pub struct LocalSnapshotMetadata {
    pub(crate) hash: Hash,
    pub(crate) index: u32,
    pub(crate) timestamp: u64,
    pub(crate) solid_entry_points: HashMap<Hash, u32>,
    pub(crate) seen_milestones: Vec<Hash>,
}

impl LocalSnapshotMetadata {
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn solid_entry_points(&self) -> &HashMap<Hash, u32> {
        &self.solid_entry_points
    }

    pub fn seen_milestones(&self) -> &Vec<Hash> {
        &self.seen_milestones
    }
}
