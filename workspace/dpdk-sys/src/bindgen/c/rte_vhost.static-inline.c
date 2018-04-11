#include <rte_vhost.h>
#include "bindgen/c/rte_vhost.static-inline.h"

uint64_t rust_rte_vhost_gpa_to_vva(struct rte_vhost_memory * mem, uint64_t gpa)
{
	return rte_vhost_gpa_to_vva(mem, gpa);
}
