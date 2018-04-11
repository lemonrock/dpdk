#include <rte_mempool.h>

void rust_rte_mempool_put_bulk(struct rte_mempool * mp, void * const * obj_table, unsigned int n);

void rust_rte_mempool_put(struct rte_mempool * mp, void * obj);

void rust_rte_mempool_generic_put(struct rte_mempool * mp, void * const * obj_table, unsigned int n, struct rte_mempool_cache * cache);

void rust_rte_mempool_cache_flush(struct rte_mempool_cache * cache, struct rte_mempool * mp);

void rust___mempool_generic_put(struct rte_mempool * mp, void * const * obj_table, unsigned int n, struct rte_mempool_cache * cache);

void * rust_rte_mempool_get_priv(struct rte_mempool * mp);

struct rte_mempool_ops * rust_rte_mempool_get_ops(int ops_index);

struct rte_mempool_objtlr * rust___mempool_get_trailer(void * obj);

struct rte_mempool_objhdr * rust___mempool_get_header(void * obj);

struct rte_mempool_cache * rust_rte_mempool_default_cache(struct rte_mempool * mp, unsigned lcore_id);

struct rte_mempool * rust_rte_mempool_from_obj(void * obj);

rte_iova_t rust_rte_mempool_virt2iova(const void * elt);

int rust_rte_mempool_ops_enqueue_bulk(struct rte_mempool * mp, void * const * obj_table, unsigned n);

int rust_rte_mempool_ops_dequeue_bulk(struct rte_mempool * mp, void * * obj_table, unsigned n);

int rust_rte_mempool_get_bulk(struct rte_mempool * mp, void * * obj_table, unsigned int n);

int rust_rte_mempool_get(struct rte_mempool * mp, void * * obj_p);

int rust_rte_mempool_generic_get(struct rte_mempool * mp, void * * obj_table, unsigned int n, struct rte_mempool_cache * cache);

int rust_rte_mempool_full(const struct rte_mempool * mp);

int rust_rte_mempool_empty(const struct rte_mempool * mp);

int rust___mempool_generic_get(struct rte_mempool * mp, void * * obj_table, unsigned int n, struct rte_mempool_cache * cache);
