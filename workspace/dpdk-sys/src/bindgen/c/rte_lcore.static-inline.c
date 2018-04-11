#include <rte_lcore.h>
#include "bindgen/c/rte_lcore.static-inline.h"

unsigned rust_rte_lcore_to_socket_id(unsigned lcore_id)
{
	return rte_lcore_to_socket_id(lcore_id);
}

unsigned rust_rte_lcore_id( )
{
	return rte_lcore_id();
}

unsigned rust_rte_lcore_count( )
{
	return rte_lcore_count();
}

unsigned rust_rte_get_next_lcore(unsigned i, int skip_master, int wrap)
{
	return rte_get_next_lcore(i, skip_master, wrap);
}

unsigned rust_rte_get_master_lcore( )
{
	return rte_get_master_lcore();
}

int rust_rte_lcore_is_enabled(unsigned lcore_id)
{
	return rte_lcore_is_enabled(lcore_id);
}

int rust_rte_lcore_index(int lcore_id)
{
	return rte_lcore_index(lcore_id);
}
