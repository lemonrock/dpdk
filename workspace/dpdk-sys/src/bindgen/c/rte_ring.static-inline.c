#include <rte_ring.h>
#include "bindgen/c/rte_ring.static-inline.h"

unsigned rust_rte_ring_sp_enqueue_burst(struct rte_ring * r, void * const * obj_table, unsigned int n, unsigned int * free_space)
{
	return rte_ring_sp_enqueue_burst(r, obj_table, n, free_space);
}

unsigned rust_rte_ring_sc_dequeue_burst(struct rte_ring * r, void * * obj_table, unsigned int n, unsigned int * available)
{
	return rte_ring_sc_dequeue_burst(r, obj_table, n, available);
}

unsigned rust_rte_ring_mp_enqueue_burst(struct rte_ring * r, void * const * obj_table, unsigned int n, unsigned int * free_space)
{
	return rte_ring_mp_enqueue_burst(r, obj_table, n, free_space);
}

unsigned rust_rte_ring_mc_dequeue_burst(struct rte_ring * r, void * * obj_table, unsigned int n, unsigned int * available)
{
	return rte_ring_mc_dequeue_burst(r, obj_table, n, available);
}

unsigned rust_rte_ring_free_count(const struct rte_ring * r)
{
	return rte_ring_free_count(r);
}

unsigned rust_rte_ring_enqueue_burst(struct rte_ring * r, void * const * obj_table, unsigned int n, unsigned int * free_space)
{
	return rte_ring_enqueue_burst(r, obj_table, n, free_space);
}

unsigned rust_rte_ring_dequeue_burst(struct rte_ring * r, void * * obj_table, unsigned int n, unsigned int * available)
{
	return rte_ring_dequeue_burst(r, obj_table, n, available);
}

unsigned rust_rte_ring_count(const struct rte_ring * r)
{
	return rte_ring_count(r);
}

unsigned int rust_rte_ring_sp_enqueue_bulk(struct rte_ring * r, void * const * obj_table, unsigned int n, unsigned int * free_space)
{
	return rte_ring_sp_enqueue_bulk(r, obj_table, n, free_space);
}

unsigned int rust_rte_ring_sc_dequeue_bulk(struct rte_ring * r, void * * obj_table, unsigned int n, unsigned int * available)
{
	return rte_ring_sc_dequeue_bulk(r, obj_table, n, available);
}

unsigned int rust_rte_ring_mp_enqueue_bulk(struct rte_ring * r, void * const * obj_table, unsigned int n, unsigned int * free_space)
{
	return rte_ring_mp_enqueue_bulk(r, obj_table, n, free_space);
}

unsigned int rust_rte_ring_mc_dequeue_bulk(struct rte_ring * r, void * * obj_table, unsigned int n, unsigned int * available)
{
	return rte_ring_mc_dequeue_bulk(r, obj_table, n, available);
}

unsigned int rust_rte_ring_get_size(const struct rte_ring * r)
{
	return rte_ring_get_size(r);
}

unsigned int rust_rte_ring_get_capacity(const struct rte_ring * r)
{
	return rte_ring_get_capacity(r);
}

unsigned int rust_rte_ring_enqueue_bulk(struct rte_ring * r, void * const * obj_table, unsigned int n, unsigned int * free_space)
{
	return rte_ring_enqueue_bulk(r, obj_table, n, free_space);
}

unsigned int rust_rte_ring_dequeue_bulk(struct rte_ring * r, void * * obj_table, unsigned int n, unsigned int * available)
{
	return rte_ring_dequeue_bulk(r, obj_table, n, available);
}

unsigned int rust___rte_ring_do_enqueue(struct rte_ring * r, void * const * obj_table, unsigned int n, enum rte_ring_queue_behavior behavior, int is_sp, unsigned int * free_space)
{
	return __rte_ring_do_enqueue(r, obj_table, n, behavior, is_sp, free_space);
}

unsigned int rust___rte_ring_do_dequeue(struct rte_ring * r, void * * obj_table, unsigned int n, enum rte_ring_queue_behavior behavior, int is_sc, unsigned int * available)
{
	return __rte_ring_do_dequeue(r, obj_table, n, behavior, is_sc, available);
}

int rust_rte_ring_sp_enqueue(struct rte_ring * r, void * obj)
{
	return rte_ring_sp_enqueue(r, obj);
}

int rust_rte_ring_sc_dequeue(struct rte_ring * r, void * * obj_p)
{
	return rte_ring_sc_dequeue(r, obj_p);
}

int rust_rte_ring_mp_enqueue(struct rte_ring * r, void * obj)
{
	return rte_ring_mp_enqueue(r, obj);
}

int rust_rte_ring_mc_dequeue(struct rte_ring * r, void * * obj_p)
{
	return rte_ring_mc_dequeue(r, obj_p);
}

int rust_rte_ring_full(const struct rte_ring * r)
{
	return rte_ring_full(r);
}

int rust_rte_ring_enqueue(struct rte_ring * r, void * obj)
{
	return rte_ring_enqueue(r, obj);
}

int rust_rte_ring_empty(const struct rte_ring * r)
{
	return rte_ring_empty(r);
}

int rust_rte_ring_dequeue(struct rte_ring * r, void * * obj_p)
{
	return rte_ring_dequeue(r, obj_p);
}
