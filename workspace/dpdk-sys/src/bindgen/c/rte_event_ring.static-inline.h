#include <rte_event_ring.h>

unsigned int rust_rte_event_ring_get_size(const struct rte_event_ring * r);

unsigned int rust_rte_event_ring_get_capacity(const struct rte_event_ring * r);

unsigned int rust_rte_event_ring_free_count(const struct rte_event_ring * r);

unsigned int rust_rte_event_ring_enqueue_burst(struct rte_event_ring * r, const struct rte_event * events, unsigned int n, uint16_t * free_space);

unsigned int rust_rte_event_ring_dequeue_burst(struct rte_event_ring * r, struct rte_event * events, unsigned int n, uint16_t * available);

unsigned int rust_rte_event_ring_count(const struct rte_event_ring * r);
