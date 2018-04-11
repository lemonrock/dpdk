#include <rte_ip.h>
#include "bindgen/c/rte_ip.static-inline.h"

uint32_t rust___rte_raw_cksum(const void * buf, size_t len, uint32_t sum)
{
	return __rte_raw_cksum(buf, len, sum);
}

uint16_t rust_rte_raw_cksum(const void * buf, size_t len)
{
	return rte_raw_cksum(buf, len);
}

uint16_t rust_rte_ipv6_udptcp_cksum(const struct ipv6_hdr * ipv6_hdr, const void * l4_hdr)
{
	return rte_ipv6_udptcp_cksum(ipv6_hdr, l4_hdr);
}

uint16_t rust_rte_ipv6_phdr_cksum(const struct ipv6_hdr * ipv6_hdr, uint64_t ol_flags)
{
	return rte_ipv6_phdr_cksum(ipv6_hdr, ol_flags);
}

uint16_t rust_rte_ipv4_udptcp_cksum(const struct ipv4_hdr * ipv4_hdr, const void * l4_hdr)
{
	return rte_ipv4_udptcp_cksum(ipv4_hdr, l4_hdr);
}

uint16_t rust_rte_ipv4_phdr_cksum(const struct ipv4_hdr * ipv4_hdr, uint64_t ol_flags)
{
	return rte_ipv4_phdr_cksum(ipv4_hdr, ol_flags);
}

uint16_t rust_rte_ipv4_cksum(const struct ipv4_hdr * ipv4_hdr)
{
	return rte_ipv4_cksum(ipv4_hdr);
}

uint16_t rust___rte_raw_cksum_reduce(uint32_t sum)
{
	return __rte_raw_cksum_reduce(sum);
}

int rust_rte_raw_cksum_mbuf(const struct rte_mbuf * m, uint32_t off, uint32_t len, uint16_t * cksum)
{
	return rte_raw_cksum_mbuf(m, off, len, cksum);
}
