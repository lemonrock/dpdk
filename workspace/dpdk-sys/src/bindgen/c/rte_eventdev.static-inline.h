#include <rte_eventdev.h>

uint16_t rust_rte_event_enqueue_new_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events);

uint16_t rust_rte_event_enqueue_forward_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events);

uint16_t rust_rte_event_enqueue_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events);

uint16_t rust_rte_event_dequeue_burst(uint8_t dev_id, uint8_t port_id, struct rte_event ev[], uint16_t nb_events, uint64_t timeout_ticks);

uint16_t rust___rte_event_enqueue_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events, const event_enqueue_burst_t fn);
