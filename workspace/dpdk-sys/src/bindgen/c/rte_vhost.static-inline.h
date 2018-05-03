#include <rte_vhost.h>

uint64_t rust_rte_vhost_va_from_guest_pa(struct rte_vhost_memory * mem, uint64_t gpa, uint64_t * len);
