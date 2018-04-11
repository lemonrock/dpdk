#include <rte_lpm.h>

void rust_rte_lpm_lookupx4(const struct rte_lpm * lpm, xmm_t ip, uint32_t hop[4], uint32_t defv);

int rust_rte_lpm_lookup_bulk_func(const struct rte_lpm * lpm, const uint32_t * ips, uint32_t * next_hops, const unsigned n);

int rust_rte_lpm_lookup(struct rte_lpm * lpm, uint32_t ip, uint32_t * next_hop);
