#include <rte_ip_frag.h>

struct ipv6_extension_fragment * rust_rte_ipv6_frag_get_ipv6_fragment_header(struct ipv6_hdr * hdr);

int rust_rte_ipv4_frag_pkt_is_fragmented(const struct ipv4_hdr * hdr);
