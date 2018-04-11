#include <rte_ethdev.h>
#include "bindgen/c/rte_ethdev.static-inline.h"

uint16_t rust_rte_eth_tx_prepare(uint16_t port_id, uint16_t queue_id, struct rte_mbuf * * tx_pkts, uint16_t nb_pkts)
{
	return rte_eth_tx_prepare(port_id, queue_id, tx_pkts, nb_pkts);
}

uint16_t rust_rte_eth_tx_burst(uint16_t port_id, uint16_t queue_id, struct rte_mbuf * * tx_pkts, uint16_t nb_pkts)
{
	return rte_eth_tx_burst(port_id, queue_id, tx_pkts, nb_pkts);
}

uint16_t rust_rte_eth_tx_buffer_flush(uint16_t port_id, uint16_t queue_id, struct rte_eth_dev_tx_buffer * buffer)
{
	return rte_eth_tx_buffer_flush(port_id, queue_id, buffer);
}

uint16_t rust_rte_eth_tx_buffer(uint16_t port_id, uint16_t queue_id, struct rte_eth_dev_tx_buffer * buffer, struct rte_mbuf * tx_pkt)
{
	return rte_eth_tx_buffer(port_id, queue_id, buffer, tx_pkt);
}

uint16_t rust_rte_eth_rx_burst(uint16_t port_id, uint16_t queue_id, struct rte_mbuf * * rx_pkts, const uint16_t nb_pkts)
{
	return rte_eth_rx_burst(port_id, queue_id, rx_pkts, nb_pkts);
}

int rust_rte_eth_tx_descriptor_status(uint16_t port_id, uint16_t queue_id, uint16_t offset)
{
	return rte_eth_tx_descriptor_status(port_id, queue_id, offset);
}

int rust_rte_eth_rx_queue_count(uint16_t port_id, uint16_t queue_id)
{
	return rte_eth_rx_queue_count(port_id, queue_id);
}

int rust_rte_eth_rx_descriptor_status(uint16_t port_id, uint16_t queue_id, uint16_t offset)
{
	return rte_eth_rx_descriptor_status(port_id, queue_id, offset);
}

int rust_rte_eth_rx_descriptor_done(uint16_t port_id, uint16_t queue_id, uint16_t offset)
{
	return rte_eth_rx_descriptor_done(port_id, queue_id, offset);
}
