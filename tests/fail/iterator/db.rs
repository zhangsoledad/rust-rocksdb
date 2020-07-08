extern crate ckb_rocksdb as rocksdb;

use rocksdb::{IteratorMode, DB};
use rocksdb::ops::{Open, Iterate};

fn main() {
    let _iter = {
        let db = DB::open_default("foo").unwrap();
        db.iterator(IteratorMode::Start)
    };
}
