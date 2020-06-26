use crate::{handle::Handle, ColumnFamily, Error};
use libc::c_char;
use std::ffi::CString;

pub trait SetOptions {
    fn set_options(&self, opts: &[(&str, &str)]) -> Result<(), Error>;
    fn set_options_cf(&self, cf: &ColumnFamily, opts: &[(&str, &str)]) -> Result<(), Error>;
}

impl<T> SetOptions for T
where
    T: Handle<ffi::rocksdb_t>,
{
    fn set_options(&self, opts: &[(&str, &str)]) -> Result<(), Error> {
        let copts = build_coptions(opts)?;
        let cnames: Vec<*const c_char> = copts.iter().map(|opt| opt.0.as_ptr()).collect();
        let cvalues: Vec<*const c_char> = copts.iter().map(|opt| opt.1.as_ptr()).collect();
        let count = opts.len() as i32;

        unsafe {
            ffi_try!(ffi::rocksdb_set_options(
                self.handle(),
                count,
                cnames.as_ptr(),
                cvalues.as_ptr(),
            ));
        }
        Ok(())
    }

    fn set_options_cf(&self, cf: &ColumnFamily, opts: &[(&str, &str)]) -> Result<(), Error> {
        let copts = build_coptions(opts)?;
        let cnames: Vec<*const c_char> = copts.iter().map(|opt| opt.0.as_ptr()).collect();
        let cvalues: Vec<*const c_char> = copts.iter().map(|opt| opt.1.as_ptr()).collect();
        let count = opts.len() as i32;

        unsafe {
            ffi_try!(ffi::rocksdb_set_options_cf(
                self.handle(),
                cf.handle(),
                count,
                cnames.as_ptr(),
                cvalues.as_ptr(),
            ));
        }
        Ok(())
    }
}

fn build_coptions(opts: &[(&str, &str)]) -> Result<Vec<(CString, CString)>, Error> {
    opts.iter()
        .map(|(name, value)| {
            let cname = match CString::new(name.as_bytes()) {
                Ok(cname) => cname,
                Err(e) => return Err(Error::new(format!("Invalid option name `{}`", e))),
            };
            let cvalue = match CString::new(value.as_bytes()) {
                Ok(cvalue) => cvalue,
                Err(e) => return Err(Error::new(format!("Invalid option value: `{}`", e))),
            };
            Ok((cname, cvalue))
        })
        .collect()
}
