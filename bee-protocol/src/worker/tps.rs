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

use crate::protocol::Protocol;

use std::time::Duration;

use async_std::{future::ready, prelude::*};
use futures::channel::oneshot::Receiver;
use log::info;

pub(crate) struct TpsWorker {
    incoming: u64,
    new: u64,
    outgoing: u64,
}

impl TpsWorker {
    pub(crate) fn new() -> Self {
        Self {
            incoming: 0,
            new: 0,
            outgoing: 0,
        }
    }

    fn tps(&mut self) {
        let incoming = Protocol::get().metrics.transaction_broadcast_received();
        let new = Protocol::get().metrics.new_transactions_received();
        let outgoing = Protocol::get().metrics.transaction_broadcast_sent();

        info!(
            "incoming {} new {} outgoing {}",
            incoming - self.incoming,
            new - self.new,
            outgoing - self.outgoing
        );

        self.incoming = incoming;
        self.new = new;
        self.outgoing = outgoing;
    }

    pub(crate) async fn run(mut self, mut shutdown: Receiver<()>) {
        info!("Running.");

        loop {
            match ready(Ok(()))
                .delay(Duration::from_millis(1000))
                .race(&mut shutdown)
                .await
            {
                Ok(_) => self.tps(),
                Err(_) => break,
            }
        }

        info!("Stopped.");
    }
}