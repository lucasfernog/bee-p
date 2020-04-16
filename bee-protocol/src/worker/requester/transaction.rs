use crate::{
    milestone::MilestoneIndex,
    protocol::Protocol,
};

use bee_bundle::Hash;
use bee_tangle::tangle;

use std::cmp::Ordering;

use futures::{
    channel::oneshot,
    future::FutureExt,
    select,
};
use log::info;

#[derive(Eq, PartialEq)]
pub(crate) struct TransactionRequesterWorkerEntry(pub(crate) Hash, pub(crate) MilestoneIndex);

// TODO check that this is the right order
impl PartialOrd for TransactionRequesterWorkerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for TransactionRequesterWorkerEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

pub(crate) struct TransactionRequesterWorker {}

impl TransactionRequesterWorker {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn process_request(&self, _hash: Hash, _index: MilestoneIndex) {
        // TODO use sender worker
        // TODO check that neighbor may have the tx (by the index)
        // TODO convert hash to bytes
        // let _bytes = TransactionRequest::new(hash).into_full_bytes();
        // TODO we don't have any peer_id here
    }

    pub(crate) async fn run(self, shutdown: oneshot::Receiver<()>) {
        info!("[TransactionRequesterWorker ] Running.");

        let mut shutdown_fused = shutdown.fuse();

        loop {
            select! {
                // TODO impl fused stream
                entry = Protocol::get().transaction_requester_worker.0.pop().fuse() => {
                    if let TransactionRequesterWorkerEntry(hash, index) = entry {
                        if !tangle().is_solid_entry_point(&hash) && !tangle().contains_transaction(&hash) {
                            self.process_request(hash, index);
                        }
                    }
                },
                _ = shutdown_fused => {
                    break;
                }
            }
        }

        info!("[TransactionRequesterWorker ] Stopped.");
    }
}
