#include <rte_mbuf.h>
#include "bindgen/c/rte_mbuf.static-inline.h"

void rust_rte_pktmbuf_reset_headroom(struct rte_mbuf * m)
{
	rte_pktmbuf_reset_headroom(m);
}

void rust_rte_pktmbuf_reset(struct rte_mbuf * m)
{
	rte_pktmbuf_reset(m);
}

void rust_rte_pktmbuf_refcnt_update(struct rte_mbuf * m, int16_t v)
{
	rte_pktmbuf_refcnt_update(m, v);
}

void rust_rte_pktmbuf_free_seg(struct rte_mbuf * m)
{
	rte_pktmbuf_free_seg(m);
}

void rust_rte_pktmbuf_free(struct rte_mbuf * m)
{
	rte_pktmbuf_free(m);
}

void rust_rte_pktmbuf_detach(struct rte_mbuf * m)
{
	rte_pktmbuf_detach(m);
}

void rust_rte_pktmbuf_attach(struct rte_mbuf * mi, struct rte_mbuf * m)
{
	rte_pktmbuf_attach(mi, m);
}

void rust_rte_mbuf_refcnt_set(struct rte_mbuf * m, uint16_t new_value)
{
	rte_mbuf_refcnt_set(m, new_value);
}

void rust_rte_mbuf_raw_free(struct rte_mbuf * m)
{
	rte_mbuf_raw_free(m);
}

void rust_rte_mbuf_prefetch_part2(struct rte_mbuf * m)
{
	rte_mbuf_prefetch_part2(m);
}

void rust_rte_mbuf_prefetch_part1(struct rte_mbuf * m)
{
	rte_mbuf_prefetch_part1(m);
}

void rust_rte_mbuf_ext_refcnt_set(struct rte_mbuf_ext_shared_info * shinfo, uint16_t new_value)
{
	rte_mbuf_ext_refcnt_set(shinfo, new_value);
}

void rust___rte_pktmbuf_free_extbuf(struct rte_mbuf * m)
{
	__rte_pktmbuf_free_extbuf(m);
}

void rust___rte_pktmbuf_free_direct(struct rte_mbuf * m)
{
	__rte_pktmbuf_free_direct(m);
}

void __rte_experimental rust_rte_pktmbuf_attach_extbuf(struct rte_mbuf * m, void * buf_addr, rte_iova_t buf_iova, uint16_t buf_len, struct rte_mbuf_ext_shared_info * shinfo)
{
	return rte_pktmbuf_attach_extbuf(m, buf_addr, buf_iova, buf_len, shinfo);
}

uint16_t rust_rte_pktmbuf_tailroom(const struct rte_mbuf * m)
{
	return rte_pktmbuf_tailroom(m);
}

uint16_t rust_rte_pktmbuf_priv_size(struct rte_mempool * mp)
{
	return rte_pktmbuf_priv_size(mp);
}

uint16_t rust_rte_pktmbuf_headroom(const struct rte_mbuf * m)
{
	return rte_pktmbuf_headroom(m);
}

uint16_t rust_rte_pktmbuf_data_room_size(struct rte_mempool * mp)
{
	return rte_pktmbuf_data_room_size(mp);
}

uint16_t rust_rte_mbuf_refcnt_update(struct rte_mbuf * m, int16_t value)
{
	return rte_mbuf_refcnt_update(m, value);
}

uint16_t rust_rte_mbuf_refcnt_read(const struct rte_mbuf * m)
{
	return rte_mbuf_refcnt_read(m);
}

uint16_t rust_rte_mbuf_ext_refcnt_update(struct rte_mbuf_ext_shared_info * shinfo, int16_t value)
{
	return rte_mbuf_ext_refcnt_update(shinfo, value);
}

uint16_t rust_rte_mbuf_ext_refcnt_read(const struct rte_mbuf_ext_shared_info * shinfo)
{
	return rte_mbuf_ext_refcnt_read(shinfo);
}

uint16_t rust___rte_mbuf_refcnt_update(struct rte_mbuf * m, int16_t value)
{
	return __rte_mbuf_refcnt_update(m, value);
}

struct rte_mbuf_ext_shared_info * rust_rte_pktmbuf_ext_shinfo_init_helper(void * buf_addr, uint16_t * buf_len, rte_mbuf_extbuf_free_callback_t free_cb, void * fcb_opaque)
{
	return rte_pktmbuf_ext_shinfo_init_helper(buf_addr, buf_len, free_cb, fcb_opaque);
}

struct rte_mbuf * rust_rte_pktmbuf_prefree_seg(struct rte_mbuf * m)
{
	return rte_pktmbuf_prefree_seg(m);
}

struct rte_mbuf * rust_rte_pktmbuf_lastseg(struct rte_mbuf * m)
{
	return rte_pktmbuf_lastseg(m);
}

struct rte_mbuf * rust_rte_pktmbuf_clone(struct rte_mbuf * md, struct rte_mempool * mp)
{
	return rte_pktmbuf_clone(md, mp);
}

struct rte_mbuf * rust_rte_pktmbuf_alloc(struct rte_mempool * mp)
{
	return rte_pktmbuf_alloc(mp);
}

struct rte_mbuf * rust_rte_mbuf_raw_alloc(struct rte_mempool * mp)
{
	return rte_mbuf_raw_alloc(mp);
}

struct rte_mbuf * rust_rte_mbuf_from_indirect(struct rte_mbuf * mi)
{
	return rte_mbuf_from_indirect(mi);
}

rte_iova_t rust_rte_mbuf_data_iova_default(const struct rte_mbuf * mb)
{
	return rte_mbuf_data_iova_default(mb);
}

rte_iova_t rust_rte_mbuf_data_iova(const struct rte_mbuf * mb)
{
	return rte_mbuf_data_iova(mb);
}

int rust_rte_validate_tx_offload(const struct rte_mbuf * m)
{
	return rte_validate_tx_offload(m);
}

int rust_rte_pktmbuf_trim(struct rte_mbuf * m, uint16_t len)
{
	return rte_pktmbuf_trim(m, len);
}

int rust_rte_pktmbuf_linearize(struct rte_mbuf * mbuf)
{
	return rte_pktmbuf_linearize(mbuf);
}

int rust_rte_pktmbuf_is_contiguous(const struct rte_mbuf * m)
{
	return rte_pktmbuf_is_contiguous(m);
}

int rust_rte_pktmbuf_chain(struct rte_mbuf * head, struct rte_mbuf * tail)
{
	return rte_pktmbuf_chain(head, tail);
}

int rust_rte_pktmbuf_alloc_bulk(struct rte_mempool * pool, struct rte_mbuf * * mbufs, unsigned count)
{
	return rte_pktmbuf_alloc_bulk(pool, mbufs, count);
}

const void * rust_rte_pktmbuf_read(const struct rte_mbuf * m, uint32_t off, uint32_t len, void * buf)
{
	return rte_pktmbuf_read(m, off, len, buf);
}

char * rust_rte_pktmbuf_prepend(struct rte_mbuf * m, uint16_t len)
{
	return rte_pktmbuf_prepend(m, len);
}

char * rust_rte_pktmbuf_append(struct rte_mbuf * m, uint16_t len)
{
	return rte_pktmbuf_append(m, len);
}

char * rust_rte_pktmbuf_adj(struct rte_mbuf * m, uint16_t len)
{
	return rte_pktmbuf_adj(m, len);
}

char * rust_rte_mbuf_to_baddr(struct rte_mbuf * md)
{
	return rte_mbuf_to_baddr(md);
}
