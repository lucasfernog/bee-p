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

use bee_common::worker::Error as WorkerError;
use bee_protocol::Milestone;

use futures::{
    channel::{mpsc, oneshot},
    future::FutureExt,
    select,
    stream::StreamExt,
};
use log::info;

pub(crate) struct LedgerConfirmationWorkerEvent(pub(crate) Milestone);

pub(crate) struct LedgerConfirmationWorker {}

impl LedgerConfirmationWorker {
    pub fn new() -> Self {
        Self {}
    }

    fn confirm(&self, milestone: Milestone) {
        info!("Confirming milestone {}.", milestone.index().0);
    }

    pub async fn run(
        self,
        receiver: mpsc::UnboundedReceiver<LedgerConfirmationWorkerEvent>,
        shutdown: oneshot::Receiver<()>,
    ) -> Result<(), WorkerError> {
        info!("Running.");

        let mut receiver_fused = receiver.fuse();
        let mut shutdown_fused = shutdown.fuse();

        loop {
            select! {
                _ = shutdown_fused => break,
                event = receiver_fused.next() => {
                    if let Some(LedgerConfirmationWorkerEvent(milestone)) = event {
                        self.confirm(milestone)
                    }
                }
            }
        }

        info!("Stopped.");

        Ok(())
    }
}
