#include <rte_ether.h>

void rust_ether_format_addr(char * buf, uint16_t size, const struct ether_addr * eth_addr);

void rust_ether_addr_copy(const struct ether_addr * ea_from, struct ether_addr * ea_to);

void rust_eth_random_addr(uint8_t * addr);

int rust_rte_vlan_strip(struct rte_mbuf * m);

int rust_rte_vlan_insert(struct rte_mbuf * * m);

int rust_is_zero_ether_addr(const struct ether_addr * ea);

int rust_is_valid_assigned_ether_addr(const struct ether_addr * ea);

int rust_is_universal_ether_addr(const struct ether_addr * ea);

int rust_is_unicast_ether_addr(const struct ether_addr * ea);

int rust_is_same_ether_addr(const struct ether_addr * ea1, const struct ether_addr * ea2);

int rust_is_multicast_ether_addr(const struct ether_addr * ea);

int rust_is_local_admin_ether_addr(const struct ether_addr * ea);

int rust_is_broadcast_ether_addr(const struct ether_addr * ea);
