// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_ledger::storage::Backend as LedgerBackend;
use bee_storage::storage;

pub trait Backend: storage::Backend + LedgerBackend {}

impl<T> Backend for T where T: storage::Backend + LedgerBackend {}
