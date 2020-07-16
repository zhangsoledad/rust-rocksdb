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
    let full_opts = {
        let config_file = "tests/resources/OPTIONS-000001";
        let opts_res = FullOptions::load_from_file(config_file, None, false);
        assert!(opts_res.is_ok());
        opts_res.unwrap()
    };

    assert_eq!(full_opts.cf_descriptors.len(), 2);
    let cf_default_name = "default";
    assert!(
        full_opts
            .cf_descriptors
            .iter()
            .any(|cfd| cfd.name() == cf_default_name),
        "there must be column family named \"{}\"",
        cf_default_name
    );

    {
        let cfs_0 = &[];
        let cfs_1 = &["col_fam_A"];
        let cfs_2 = &["col_fam_A", "col_fam_B"];
        let cfs_3 = &["col_fam_B"];
        let mut fopts_0 = full_opts.clone();
        let mut fopts_1 = full_opts.clone();
        let mut fopts_2 = full_opts.clone();
        let mut fopts_3 = full_opts.clone();

        let res_0 = fopts_0.complete_column_families(cfs_0, false);
        let res_1 = fopts_1.complete_column_families(cfs_1, false);
        let res_2 = fopts_2.complete_column_families(cfs_2, false);
        let res_3 = fopts_3.complete_column_families(cfs_3, false);
        assert!(res_0.is_err(), "no column family was expected");
        assert!(res_1.is_ok(), "error: {}", res_1.unwrap_err());
        assert!(res_2.is_ok(), "error: {}", res_2.unwrap_err());
        assert!(res_3.is_err(), "expect \"col_fam_B\"; has \"col_fam_A\"");
        assert_eq!(fopts_1.cf_descriptors.len(), 2);
        assert_eq!(fopts_2.cf_descriptors.len(), 3);

        let res_1_0 = fopts_1.clone().complete_column_families(cfs_0, false);
        let res_1_1 = fopts_1.clone().complete_column_families(cfs_1, false);
        let res_1_2 = fopts_1.clone().complete_column_families(cfs_2, false);
        let res_1_3 = fopts_1.clone().complete_column_families(cfs_3, false);
        assert!(res_1_0.is_err(), "no column family was expected");
        assert!(res_1_1.is_ok(), "error: {}", res_1_1.unwrap_err());
        assert!(res_1_2.is_ok(), "error: {}", res_1_2.unwrap_err());
        assert!(res_1_3.is_err(), "expect \"col_fam_B\"; has \"col_fam_A\"");
        let res_1_0 = fopts_1.clone().complete_column_families(cfs_0, true);
        let res_1_3 = fopts_1.clone().complete_column_families(cfs_3, true);
        assert!(res_1_0.is_ok(), "error: {}", res_1_0.unwrap_err());
        assert!(res_1_3.is_ok(), "error: {}", res_1_3.unwrap_err());

        let res_2_0 = fopts_2.clone().complete_column_families(cfs_0, false);
        let res_2_1 = fopts_2.clone().complete_column_families(cfs_1, false);
        let res_2_2 = fopts_2.clone().complete_column_families(cfs_2, false);
        let res_2_3 = fopts_2.clone().complete_column_families(cfs_3, false);
        assert!(res_2_0.is_err(), "no column family was expected");
        assert!(res_2_1.is_err(), "1 column family was expected but has 2");
        assert!(res_2_2.is_ok(), "error: {}", res_2_2.unwrap_err());
        assert!(res_2_3.is_err(), "should not has \"col_fam_A\"");
        let res_2_0 = fopts_2.clone().complete_column_families(cfs_0, true);
        let res_2_1 = fopts_2.clone().complete_column_families(cfs_1, true);
        let res_2_3 = fopts_2.clone().complete_column_families(cfs_3, true);
        assert!(res_2_0.is_ok(), "error: {}", res_2_0.unwrap_err());
        assert!(res_2_1.is_ok(), "error: {}", res_2_1.unwrap_err());
        assert!(res_2_3.is_ok(), "error: {}", res_2_3.unwrap_err());
    }

    let path = TemporaryDBPath::new();
    {
        let mut fopts = full_opts.clone();
        let cf_names = &["col_fam_A", "col_fam_C"];
        assert!(fopts.complete_column_families(cf_names, false).is_ok());
        assert_eq!(fopts.cf_descriptors.len(), 3);

        let FullOptions {
            mut db_opts,
            cf_descriptors,
        } = fopts;
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);
        let db = DB::open_cf_descriptors(&db_opts, &path, cf_descriptors).unwrap();
        let cf_a_opt = db.cf_handle("col_fam_A");
        let cf_b_opt = db.cf_handle("col_fam_B");
        let cf_c_opt = db.cf_handle("col_fam_C");
        assert!(cf_a_opt.is_some());
        assert!(cf_b_opt.is_none());
        assert!(cf_c_opt.is_some());
    }
}
