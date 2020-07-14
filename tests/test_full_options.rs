// Copyright 2020 Nervos Core Dev
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate ckb_rocksdb as rocksdb;

use crate::rocksdb::{prelude::*, FullOptions, TemporaryDBPath, DB};

#[test]
fn test_options_load_from_file() {
    let FullOptions {
        mut db_opts,
        cf_descriptors,
    } = {
        let config_file = "tests/resources/OPTIONS-000001";
        let opts_res = FullOptions::load_from_file(config_file, None, false);
        assert!(opts_res.is_ok());
        opts_res.unwrap()
    };
    let path = TemporaryDBPath::new();
    {
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);
        let db = DB::open_cf_descriptors(&db_opts, &path, cf_descriptors).unwrap();
        let cf1_opt = db.cf_handle("col_fam_A");
        assert!(cf1_opt.is_some());
    }
}
