#include <rte_net.h>
#include "bindgen/c/rte_net.static-inline.h"

int rust_rte_net_intel_cksum_prepare(struct rte_mbuf * m)
{
	return rte_net_intel_cksum_prepare(m);
}

int rust_rte_net_intel_cksum_flags_prepare(struct rte_mbuf * m, uint64_t ol_flags)
{
	return rte_net_intel_cksum_flags_prepare(m, ol_flags);
}
