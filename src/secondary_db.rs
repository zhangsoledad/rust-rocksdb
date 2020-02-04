// Copyright 2019 Xuejie Xiao
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
//

use ffi;
use ffi_util;

use crate::{
    db_iterator::DBRawIterator,
    db_options::ReadOptions,
    handle::Handle,
    open_raw::{OpenRaw, OpenRawFFI},
    ops, ColumnFamily, Error,
};

use libc::c_uchar;
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub struct SecondaryDB {
    pub(crate) inner: *mut ffi::rocksdb_t,
    cfs: BTreeMap<String, ColumnFamily>,
    path: PathBuf,
}

impl SecondaryDB {
    pub fn path(&self) -> &Path {
        &self.path.as_path()
    }

    pub fn try_catch_up_with_primary(&self) -> Result<(), Error> {
        Ok(unsafe { ffi_try!(ffi::rocksdb_try_catch_up_with_primary(self.inner,)) })
    }
}

pub struct SecondaryOpenDescriptor {
    secondary_path: String,
}

impl Default for SecondaryOpenDescriptor {
    fn default() -> Self {
        SecondaryOpenDescriptor {
            secondary_path: "".to_string(),
        }
    }
}

impl SecondaryOpenDescriptor {
    pub fn new(secondary_path: String) -> Self {
        Self { secondary_path }
    }
}

impl ops::Open for SecondaryDB {}
impl ops::OpenCF for SecondaryDB {}

impl OpenRaw for SecondaryDB {
    type Pointer = ffi::rocksdb_t;
    type Descriptor = SecondaryOpenDescriptor;

    fn open_ffi(input: OpenRawFFI<'_, Self::Descriptor>) -> Result<*mut Self::Pointer, Error> {
        if input.open_descriptor.secondary_path.is_empty() {
            return Err(Error::new(
                "Secondary DB must have secondary path provided!".to_string(),
            ));
        }
        let secondary_path = ffi_util::to_cpath(
            &input.open_descriptor.secondary_path,
            "Failed to convert path to CString when opening database.",
        )?;
        let pointer = unsafe {
            if input.num_column_families <= 0 {
                ffi_try!(ffi::rocksdb_open_as_secondary(
                    input.options,
                    input.path,
                    secondary_path.as_ptr(),
                ))
            } else {
                ffi_try!(ffi::rocksdb_open_as_secondary_column_families(
                    input.options,
                    input.path,
                    secondary_path.as_ptr(),
                    input.num_column_families,
                    input.column_family_names,
                    input.column_family_options,
                    input.column_family_handles,
                ))
            }
        };

        Ok(pointer)
    }

    fn build<I>(
        path: PathBuf,
        _open_descriptor: Self::Descriptor,
        pointer: *mut Self::Pointer,
        column_families: I,
    ) -> Result<Self, Error>
    where
        I: IntoIterator<Item = (String, *mut ffi::rocksdb_column_family_handle_t)>,
    {
        let cfs: BTreeMap<_, _> = column_families
            .into_iter()
            .map(|(k, h)| (k, ColumnFamily::new(h)))
            .collect();
        Ok(SecondaryDB {
            inner: pointer,
            cfs,
            path,
        })
    }
}

impl Handle<ffi::rocksdb_t> for SecondaryDB {
    fn handle(&self) -> *mut ffi::rocksdb_t {
        self.inner
    }
}

impl ops::Iterate for SecondaryDB {
    fn get_raw_iter(&self, readopts: &ReadOptions) -> DBRawIterator {
        unsafe {
            DBRawIterator {
                inner: ffi::rocksdb_create_iterator(self.inner, readopts.handle()),
                db: PhantomData,
            }
        }
    }
}

impl ops::IterateCF for SecondaryDB {
    fn get_raw_iter_cf(
        &self,
        cf_handle: &ColumnFamily,
        readopts: &ReadOptions,
    ) -> Result<DBRawIterator, Error> {
        unsafe {
            Ok(DBRawIterator {
                inner: ffi::rocksdb_create_iterator_cf(
                    self.inner,
                    readopts.handle(),
                    cf_handle.inner,
                ),
                db: PhantomData,
            })
        }
    }
}

impl ops::GetColumnFamilys for SecondaryDB {
    fn get_cfs(&self) -> &BTreeMap<String, ColumnFamily> {
        &self.cfs
    }
    fn get_mut_cfs(&mut self) -> &mut BTreeMap<String, ColumnFamily> {
        &mut self.cfs
    }
}

impl ops::Read for SecondaryDB {}

unsafe impl Send for SecondaryDB {}
unsafe impl Sync for SecondaryDB {}

impl Drop for SecondaryDB {
    fn drop(&mut self) {
        unsafe {
            for cf in self.cfs.values() {
                ffi::rocksdb_column_family_handle_destroy(cf.inner);
            }
            ffi::rocksdb_close(self.inner);
        }
    }
}

impl fmt::Debug for SecondaryDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Read-only RocksDB {{ path: {:?} }}", self.path())
    }
}
