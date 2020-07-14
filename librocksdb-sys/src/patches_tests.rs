use std::{ffi, ptr};

fn error_message(ptr: *const libc::c_char) -> String {
    let cstr = unsafe { ffi::CStr::from_ptr(ptr as *const _) };
    let s = String::from_utf8_lossy(cstr.to_bytes()).into_owned();
    unsafe {
        libc::free(ptr as *mut ffi::c_void);
    }
    s
}

#[test]
fn rocksdb_options_load_from_file() {
    let mut errmsg: *mut libc::c_char = ptr::null_mut();
    let config_file = "rocksdb/tools/advisor/test/input_files/OPTIONS-000005";
    let result = unsafe {
        let config_cstring = ffi::CString::new(config_file.as_bytes());
        let env = crate::rocksdb_create_default_env();
        let cache = crate::rocksdb_cache_create_lru(1000);
        let ignore_unknown_options = false;

        let result = crate::rocksdb_options_load_from_file(
            config_cstring.unwrap().as_ptr() as *const _,
            env,
            ignore_unknown_options,
            cache,
            &mut errmsg,
        );

        crate::rocksdb_cache_destroy(cache);
        crate::rocksdb_env_destroy(env);

        result
    };
    assert!(errmsg.is_null(), "error: {}", error_message(errmsg));
    assert!(!result.db_opts.is_null());
    assert!(!result.cf_descs.is_null());
    unsafe {
        crate::rocksdb_column_family_descriptors_destroy(result.cf_descs);
    }
}
