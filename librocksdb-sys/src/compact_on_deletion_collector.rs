/* automatically generated by rust-bindgen 0.56.0 */

extern "C" {
    pub fn rocksdb_options_set_new_compact_on_deletion_collector_factory(
        opt: *mut rocksdb_options_t,
        sliding_window_size: usize,
        deletion_trigger: usize,
    );
}
