#include <rte_mempool.h>
#include "bindgen/c/rte_mempool.static-inline.h"

void rust_rte_mempool_put_bulk(struct rte_mempool * mp, void * const * obj_table, unsigned int n)
{
	rte_mempool_put_bulk(mp, obj_table, n);
}

void rust_rte_mempool_put(struct rte_mempool * mp, void * obj)
{
	rte_mempool_put(mp, obj);
}

void rust_rte_mempool_generic_put(struct rte_mempool * mp, void * const * obj_table, unsigned int n, struct rte_mempool_cache * cache)
{
	rte_mempool_generic_put(mp, obj_table, n, cache);
}

void rust_rte_mempool_cache_flush(struct rte_mempool_cache * cache, struct rte_mempool * mp)
{
	rte_mempool_cache_flush(cache, mp);
}

void rust___mempool_generic_put(struct rte_mempool * mp, void * const * obj_table, unsigned int n, struct rte_mempool_cache * cache)
{
	__mempool_generic_put(mp, obj_table, n, cache);
}

void * rust_rte_mempool_get_priv(struct rte_mempool * mp)
{
	return rte_mempool_get_priv(mp);
}

struct rte_mempool_ops * rust_rte_mempool_get_ops(int ops_index)
{
	return rte_mempool_get_ops(ops_index);
}

struct rte_mempool_objtlr * rust___mempool_get_trailer(void * obj)
{
	return __mempool_get_trailer(obj);
}

struct rte_mempool_objhdr * rust___mempool_get_header(void * obj)
{
	return __mempool_get_header(obj);
}

struct rte_mempool_cache * rust_rte_mempool_default_cache(struct rte_mempool * mp, unsigned lcore_id)
{
	return rte_mempool_default_cache(mp, lcore_id);
}

struct rte_mempool * rust_rte_mempool_from_obj(void * obj)
{
	return rte_mempool_from_obj(obj);
}

rte_iova_t rust_rte_mempool_virt2iova(const void * elt)
{
	return rte_mempool_virt2iova(elt);
}

int rust_rte_mempool_ops_enqueue_bulk(struct rte_mempool * mp, void * const * obj_table, unsigned n)
{
	return rte_mempool_ops_enqueue_bulk(mp, obj_table, n);
}

int rust_rte_mempool_ops_dequeue_bulk(struct rte_mempool * mp, void * * obj_table, unsigned n)
{
	return rte_mempool_ops_dequeue_bulk(mp, obj_table, n);
}

int rust_rte_mempool_get_bulk(struct rte_mempool * mp, void * * obj_table, unsigned int n)
{
	return rte_mempool_get_bulk(mp, obj_table, n);
}

int rust_rte_mempool_get(struct rte_mempool * mp, void * * obj_p)
{
	return rte_mempool_get(mp, obj_p);
}

int rust_rte_mempool_generic_get(struct rte_mempool * mp, void * * obj_table, unsigned int n, struct rte_mempool_cache * cache)
{
	return rte_mempool_generic_get(mp, obj_table, n, cache);
}

int rust_rte_mempool_full(const struct rte_mempool * mp)
{
	return rte_mempool_full(mp);
}

int rust_rte_mempool_empty(const struct rte_mempool * mp)
{
	return rte_mempool_empty(mp);
}

int rust___mempool_generic_get(struct rte_mempool * mp, void * * obj_table, unsigned int n, struct rte_mempool_cache * cache)
{
	return __mempool_generic_get(mp, obj_table, n, cache);
}
