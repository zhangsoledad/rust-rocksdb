#include <utilities/compact_on_deletion_collector.h>
#include "rocksdb/options.h"
#include "rocksdb/utilities/table_properties_collectors.h"

using rocksdb::Options;

extern "C" {
    struct rocksdb_options_t         { Options           rep; };

    void rocksdb_options_set_new_compact_on_deletion_collector_factory(
        rocksdb_options_t* opt,
        size_t sliding_window_size,
        size_t deletion_trigger) {
            opt->rep.table_properties_collector_factories.emplace_back(
                ROCKSDB_NAMESPACE::NewCompactOnDeletionCollectorFactory(sliding_window_size, deletion_trigger));
  }
}
