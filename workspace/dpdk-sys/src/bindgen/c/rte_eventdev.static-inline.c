#include <rte_eventdev.h>
#include "bindgen/c/rte_eventdev.static-inline.h"

uint16_t rust_rte_event_enqueue_new_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events)
{
	return rte_event_enqueue_new_burst(dev_id, port_id, ev, nb_events);
}

uint16_t rust_rte_event_enqueue_forward_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events)
{
	return rte_event_enqueue_forward_burst(dev_id, port_id, ev, nb_events);
}

uint16_t rust_rte_event_enqueue_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events)
{
	return rte_event_enqueue_burst(dev_id, port_id, ev, nb_events);
}

uint16_t rust_rte_event_dequeue_burst(uint8_t dev_id, uint8_t port_id, struct rte_event ev[], uint16_t nb_events, uint64_t timeout_ticks)
{
	return rte_event_dequeue_burst(dev_id, port_id, ev, nb_events, timeout_ticks);
}

uint16_t rust___rte_event_enqueue_burst(uint8_t dev_id, uint8_t port_id, const struct rte_event ev[], uint16_t nb_events, const event_enqueue_burst_t fn)
{
	return __rte_event_enqueue_burst(dev_id, port_id, ev, nb_events, fn);
}
