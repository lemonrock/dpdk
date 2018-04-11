#include <rte_ether.h>
#include "bindgen/c/rte_ether.static-inline.h"

void rust_ether_format_addr(char * buf, uint16_t size, const struct ether_addr * eth_addr)
{
	ether_format_addr(buf, size, eth_addr);
}

void rust_ether_addr_copy(const struct ether_addr * ea_from, struct ether_addr * ea_to)
{
	ether_addr_copy(ea_from, ea_to);
}

void rust_eth_random_addr(uint8_t * addr)
{
	eth_random_addr(addr);
}

int rust_rte_vlan_strip(struct rte_mbuf * m)
{
	return rte_vlan_strip(m);
}

int rust_rte_vlan_insert(struct rte_mbuf * * m)
{
	return rte_vlan_insert(m);
}

int rust_is_zero_ether_addr(const struct ether_addr * ea)
{
	return is_zero_ether_addr(ea);
}

int rust_is_valid_assigned_ether_addr(const struct ether_addr * ea)
{
	return is_valid_assigned_ether_addr(ea);
}

int rust_is_universal_ether_addr(const struct ether_addr * ea)
{
	return is_universal_ether_addr(ea);
}

int rust_is_unicast_ether_addr(const struct ether_addr * ea)
{
	return is_unicast_ether_addr(ea);
}

int rust_is_same_ether_addr(const struct ether_addr * ea1, const struct ether_addr * ea2)
{
	return is_same_ether_addr(ea1, ea2);
}

int rust_is_multicast_ether_addr(const struct ether_addr * ea)
{
	return is_multicast_ether_addr(ea);
}

int rust_is_local_admin_ether_addr(const struct ether_addr * ea)
{
	return is_local_admin_ether_addr(ea);
}

int rust_is_broadcast_ether_addr(const struct ether_addr * ea)
{
	return is_broadcast_ether_addr(ea);
}
