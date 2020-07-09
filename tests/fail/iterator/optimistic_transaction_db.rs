extern crate ckb_rocksdb as rocksdb;

use rocksdb::{IteratorMode, OptimisticTransactionDB};
use rocksdb::ops::{Open, Iterate};

fn main() {
    let _iter = {
        let db = OptimisticTransactionDB::open_default("foo").unwrap();
        db.iterator(IteratorMode::Start)
    };
}
