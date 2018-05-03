#include <rte_mbuf.h>

void rust_rte_pktmbuf_reset_headroom(struct rte_mbuf * m);

void rust_rte_pktmbuf_reset(struct rte_mbuf * m);

void rust_rte_pktmbuf_refcnt_update(struct rte_mbuf * m, int16_t v);

void rust_rte_pktmbuf_free_seg(struct rte_mbuf * m);

void rust_rte_pktmbuf_free(struct rte_mbuf * m);

void rust_rte_pktmbuf_detach(struct rte_mbuf * m);

void rust_rte_pktmbuf_attach(struct rte_mbuf * mi, struct rte_mbuf * m);

void rust_rte_mbuf_refcnt_set(struct rte_mbuf * m, uint16_t new_value);

void rust_rte_mbuf_raw_free(struct rte_mbuf * m);

void rust_rte_mbuf_prefetch_part2(struct rte_mbuf * m);

void rust_rte_mbuf_prefetch_part1(struct rte_mbuf * m);

void rust_rte_mbuf_ext_refcnt_set(struct rte_mbuf_ext_shared_info * shinfo, uint16_t new_value);

void rust___rte_pktmbuf_free_extbuf(struct rte_mbuf * m);

void rust___rte_pktmbuf_free_direct(struct rte_mbuf * m);

void __rte_experimental rust_rte_pktmbuf_attach_extbuf(struct rte_mbuf * m, void * buf_addr, rte_iova_t buf_iova, uint16_t buf_len, struct rte_mbuf_ext_shared_info * shinfo);

uint16_t rust_rte_pktmbuf_tailroom(const struct rte_mbuf * m);

uint16_t rust_rte_pktmbuf_priv_size(struct rte_mempool * mp);

uint16_t rust_rte_pktmbuf_headroom(const struct rte_mbuf * m);

uint16_t rust_rte_pktmbuf_data_room_size(struct rte_mempool * mp);

uint16_t rust_rte_mbuf_refcnt_update(struct rte_mbuf * m, int16_t value);

uint16_t rust_rte_mbuf_refcnt_read(const struct rte_mbuf * m);

uint16_t rust_rte_mbuf_ext_refcnt_update(struct rte_mbuf_ext_shared_info * shinfo, int16_t value);

uint16_t rust_rte_mbuf_ext_refcnt_read(const struct rte_mbuf_ext_shared_info * shinfo);

uint16_t rust___rte_mbuf_refcnt_update(struct rte_mbuf * m, int16_t value);

struct rte_mbuf_ext_shared_info * rust_rte_pktmbuf_ext_shinfo_init_helper(void * buf_addr, uint16_t * buf_len, rte_mbuf_extbuf_free_callback_t free_cb, void * fcb_opaque);

struct rte_mbuf * rust_rte_pktmbuf_prefree_seg(struct rte_mbuf * m);

struct rte_mbuf * rust_rte_pktmbuf_lastseg(struct rte_mbuf * m);

struct rte_mbuf * rust_rte_pktmbuf_clone(struct rte_mbuf * md, struct rte_mempool * mp);

struct rte_mbuf * rust_rte_pktmbuf_alloc(struct rte_mempool * mp);

struct rte_mbuf * rust_rte_mbuf_raw_alloc(struct rte_mempool * mp);

struct rte_mbuf * rust_rte_mbuf_from_indirect(struct rte_mbuf * mi);

rte_iova_t rust_rte_mbuf_data_iova_default(const struct rte_mbuf * mb);

rte_iova_t rust_rte_mbuf_data_iova(const struct rte_mbuf * mb);

int rust_rte_validate_tx_offload(const struct rte_mbuf * m);

int rust_rte_pktmbuf_trim(struct rte_mbuf * m, uint16_t len);

int rust_rte_pktmbuf_linearize(struct rte_mbuf * mbuf);

int rust_rte_pktmbuf_is_contiguous(const struct rte_mbuf * m);

int rust_rte_pktmbuf_chain(struct rte_mbuf * head, struct rte_mbuf * tail);

int rust_rte_pktmbuf_alloc_bulk(struct rte_mempool * pool, struct rte_mbuf * * mbufs, unsigned count);

const void * rust_rte_pktmbuf_read(const struct rte_mbuf * m, uint32_t off, uint32_t len, void * buf);

char * rust_rte_pktmbuf_prepend(struct rte_mbuf * m, uint16_t len);

char * rust_rte_pktmbuf_append(struct rte_mbuf * m, uint16_t len);

char * rust_rte_pktmbuf_adj(struct rte_mbuf * m, uint16_t len);

char * rust_rte_mbuf_to_baddr(struct rte_mbuf * md);
