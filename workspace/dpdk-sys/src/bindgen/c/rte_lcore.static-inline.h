#include <rte_lcore.h>

unsigned rust_rte_lcore_to_socket_id(unsigned lcore_id);

unsigned rust_rte_lcore_id( );

unsigned rust_rte_lcore_count( );

unsigned rust_rte_get_next_lcore(unsigned i, int skip_master, int wrap);

unsigned rust_rte_get_master_lcore( );

int rust_rte_lcore_is_enabled(unsigned lcore_id);

int rust_rte_lcore_index(int lcore_id);
