#ifdef __cplusplus
extern "C" {
#endif

#include <rocksdb/c.h>

typedef struct rocksdb_options_t rocksdb_options_t;

extern ROCKSDB_LIBRARY_API
    void rocksdb_options_set_new_compact_on_deletion_collector_factory(
        rocksdb_options_t *opt,
        size_t sliding_window_size,
        size_t deletion_trigger);

#ifdef __cplusplus
} /* end extern "C" */
#endif
