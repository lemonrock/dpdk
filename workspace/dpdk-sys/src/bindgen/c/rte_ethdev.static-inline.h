#include <rte_ethdev.h>

uint16_t rust_rte_eth_tx_prepare(uint16_t port_id, uint16_t queue_id, struct rte_mbuf * * tx_pkts, uint16_t nb_pkts);

uint16_t rust_rte_eth_tx_burst(uint16_t port_id, uint16_t queue_id, struct rte_mbuf * * tx_pkts, uint16_t nb_pkts);

uint16_t rust_rte_eth_tx_buffer_flush(uint16_t port_id, uint16_t queue_id, struct rte_eth_dev_tx_buffer * buffer);

uint16_t rust_rte_eth_tx_buffer(uint16_t port_id, uint16_t queue_id, struct rte_eth_dev_tx_buffer * buffer, struct rte_mbuf * tx_pkt);

uint16_t rust_rte_eth_rx_burst(uint16_t port_id, uint16_t queue_id, struct rte_mbuf * * rx_pkts, const uint16_t nb_pkts);

int rust_rte_eth_tx_descriptor_status(uint16_t port_id, uint16_t queue_id, uint16_t offset);

int rust_rte_eth_rx_queue_count(uint16_t port_id, uint16_t queue_id);

int rust_rte_eth_rx_descriptor_status(uint16_t port_id, uint16_t queue_id, uint16_t offset);

int rust_rte_eth_rx_descriptor_done(uint16_t port_id, uint16_t queue_id, uint16_t offset);
