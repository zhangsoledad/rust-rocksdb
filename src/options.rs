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

use std::{ffi::CStr, path};

use crate::{db_options::new_cache, ffi, ffi_util, ColumnFamilyDescriptor, Error, Options};

pub struct FullOptions {
    pub db_opts: Options,
    pub cf_descriptors: Vec<ColumnFamilyDescriptor>,
}

impl FullOptions {
    pub fn load_from_file<P>(
        file: P,
        cache_size: Option<usize>,
        ignore_unknown_options: bool,
    ) -> Result<Self, Error>
    where
        P: AsRef<path::Path>,
    {
        let cpath = ffi_util::to_cpath(
            file,
            "Failed to convert path to CString when load config file.",
        )?;

        let cache = if let Some(cache_size) = cache_size {
            new_cache(cache_size)
        } else {
            unsafe { ffi::rocksdb_null_cache() }
        };

        unsafe {
            let env = ffi::rocksdb_create_default_env();
            let result = ffi_try!(ffi::rocksdb_options_load_from_file(
                cpath.as_ptr(),
                env,
                ignore_unknown_options,
                cache,
            ));
            ffi::rocksdb_env_destroy(env);
            let db_opts = result.db_opts;
            let cf_descs = result.cf_descs;
            let cf_descs_size = ffi::rocksdb_column_family_descriptors_count(cf_descs);
            let mut cf_descriptors = Vec::new();
            for index in 0..cf_descs_size {
                let name_raw = ffi::rocksdb_column_family_descriptors_name(cf_descs, index);
                let name_cstr = CStr::from_ptr(name_raw as *const _);
                let name = String::from_utf8_lossy(name_cstr.to_bytes());
                let cf_opts_inner = ffi::rocksdb_column_family_descriptors_options(cf_descs, index);
                let cf_opts = Options {
                    inner: cf_opts_inner,
                };
                cf_descriptors.push(ColumnFamilyDescriptor::new(name, cf_opts));
            }
            ffi::rocksdb_column_family_descriptors_destroy(cf_descs);
            Ok(Self {
                db_opts: Options { inner: db_opts },
                cf_descriptors,
            })
        }
    }
}
