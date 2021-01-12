extern crate ckb_rocksdb as rocksdb;

use crate::rocksdb::{ColumnFamily, ColumnFamilyDescriptor, Options, Snapshot, WriteBatch, DB};

#[test]
fn is_send() {
    // test (at compile time) that certain types implement the auto-trait Send, either directly for
    // pointer-wrapping types or transitively for types with all Send fields

    fn is_send<T: Send>() {
        // dummy function just used for its parameterized type bound
    }

    is_send::<DB>();
    is_send::<Snapshot>();
    is_send::<Options>();
    is_send::<ColumnFamilyDescriptor>();
    is_send::<ColumnFamily>();
    is_send::<WriteBatch>();
}

#[test]
fn is_sync() {
    // test (at compile time) that certain types implement the auto-trait Sync

    fn is_sync<T: Sync>() {
        // dummy function just used for its parameterized type bound
    }

    is_sync::<DB>();
    is_sync::<Snapshot>();
    is_sync::<Options>();
    is_sync::<ColumnFamilyDescriptor>();
}
