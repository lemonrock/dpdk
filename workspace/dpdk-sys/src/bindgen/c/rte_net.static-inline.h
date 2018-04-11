#include <rte_net.h>

int rust_rte_net_intel_cksum_prepare(struct rte_mbuf * m);

int rust_rte_net_intel_cksum_flags_prepare(struct rte_mbuf * m, uint64_t ol_flags);
