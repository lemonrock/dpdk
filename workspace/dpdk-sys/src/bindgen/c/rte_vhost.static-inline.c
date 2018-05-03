#include <rte_vhost.h>
#include "bindgen/c/rte_vhost.static-inline.h"

uint64_t rust_rte_vhost_va_from_guest_pa(struct rte_vhost_memory * mem, uint64_t gpa, uint64_t * len)
{
	return rte_vhost_va_from_guest_pa(mem, gpa, len);
}
