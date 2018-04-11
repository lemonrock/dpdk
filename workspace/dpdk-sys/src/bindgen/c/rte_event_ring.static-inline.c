#include <rte_event_ring.h>
#include "bindgen/c/rte_event_ring.static-inline.h"

unsigned int rust_rte_event_ring_get_size(const struct rte_event_ring * r)
{
	return rte_event_ring_get_size(r);
}

unsigned int rust_rte_event_ring_get_capacity(const struct rte_event_ring * r)
{
	return rte_event_ring_get_capacity(r);
}

unsigned int rust_rte_event_ring_free_count(const struct rte_event_ring * r)
{
	return rte_event_ring_free_count(r);
}

unsigned int rust_rte_event_ring_enqueue_burst(struct rte_event_ring * r, const struct rte_event * events, unsigned int n, uint16_t * free_space)
{
	return rte_event_ring_enqueue_burst(r, events, n, free_space);
}

unsigned int rust_rte_event_ring_dequeue_burst(struct rte_event_ring * r, struct rte_event * events, unsigned int n, uint16_t * available)
{
	return rte_event_ring_dequeue_burst(r, events, n, available);
}

unsigned int rust_rte_event_ring_count(const struct rte_event_ring * r)
{
	return rte_event_ring_count(r);
}
