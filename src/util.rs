// Copyright 2019 Tyler Neely
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

use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

use crate::{Options, DB};

/// Ensures that DB::Destroy is called and the directory is deleted
/// for this database when TemporaryDBPath is dropped.
pub struct TemporaryDBPath {
    dir: TempDir,
}

impl TemporaryDBPath {
    pub fn new() -> TemporaryDBPath {
        let dir = tempdir().unwrap();
        TemporaryDBPath { dir }
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.dir.path().join(path)
    }
}

impl Default for TemporaryDBPath {
    fn default() -> TemporaryDBPath {
        let dir = tempdir().unwrap();
        TemporaryDBPath { dir }
    }
}

impl AsRef<Path> for TemporaryDBPath {
    fn as_ref(&self) -> &Path {
        self.dir.path()
    }
}
