include rocksdb/src.mk

rocksdb_lib_sources.txt: rocksdb/src.mk
	@echo -n "${LIB_SOURCES}" | tr ' ' '\n' > rocksdb_lib_sources.txt

gen_lib_sources: rocksdb_lib_sources.txt

gen_patches:
	bindgen patches/rocksdb.h \
		--size_t-is-usize --blacklist-type max_align_t --ctypes-prefix libc \
		--generate types,functions \
		--blacklist-type rocksdb_cache_t \
		--blacklist-type rocksdb_env_t \
		--blacklist-type rocksdb_options_t \
		`sed -n '/^extern ROCKSDB_LIBRARY_API/ {n;p}' patches/rocksdb.h | sed -n 's/^    .* \([a-z0-9_]*\)(.*$$/\1/p' | xargs -I {} echo -n " --whitelist-function {}"` \
		-o src/patches.rs -- -I rocksdb/include
