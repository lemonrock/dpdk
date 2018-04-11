#include <rte_ip.h>

uint32_t rust___rte_raw_cksum(const void * buf, size_t len, uint32_t sum);

uint16_t rust_rte_raw_cksum(const void * buf, size_t len);

uint16_t rust_rte_ipv6_udptcp_cksum(const struct ipv6_hdr * ipv6_hdr, const void * l4_hdr);

uint16_t rust_rte_ipv6_phdr_cksum(const struct ipv6_hdr * ipv6_hdr, uint64_t ol_flags);

uint16_t rust_rte_ipv4_udptcp_cksum(const struct ipv4_hdr * ipv4_hdr, const void * l4_hdr);

uint16_t rust_rte_ipv4_phdr_cksum(const struct ipv4_hdr * ipv4_hdr, uint64_t ol_flags);

uint16_t rust_rte_ipv4_cksum(const struct ipv4_hdr * ipv4_hdr);

uint16_t rust___rte_raw_cksum_reduce(uint32_t sum);

int rust_rte_raw_cksum_mbuf(const struct rte_mbuf * m, uint32_t off, uint32_t len, uint16_t * cksum);
