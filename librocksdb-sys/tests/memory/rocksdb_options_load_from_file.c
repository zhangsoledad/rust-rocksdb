#include "patches/rocksdb.h"

int main (int argc, char *argv[])
{
    const char* config_file = "rocksdb/tools/advisor/test/input_files/OPTIONS-000005";


    rocksdb_env_t* env = rocksdb_create_default_env();
    bool ignore_unknown_options = false;
    rocksdb_cache_t* cache = rocksdb_cache_create_lru(1000);
    char **errptr;

    rocksdb_fulloptions_t fullopts = rocksdb_options_load_from_file(
        config_file,
        env,
        ignore_unknown_options,
        cache,
        errptr);

    return 0;
}
