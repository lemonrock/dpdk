#[allow(unused_extern_crates)] #[macro_use] extern crate rust_c;
extern crate libc;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_longlong;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::size_t;
use ::libc::ssize_t;
use ::libc::uint8_t;
use ::libc::uint16_t;
use ::libc::uint32_t;
use ::libc::uint64_t;
use ::libc::int8_t;
use ::libc::int16_t;
use ::libc::int32_t;
use ::libc::int64_t;
use ::libc::off_t;
use ::libc::FILE;
use ::libc::sockaddr_storage;
use ::libc::sockaddr;
#[cfg(unix)] use ::libc::in_addr;
#[cfg(unix)] use ::libc::in6_addr;
#[cfg(unix)] use ::libc::termios;
#[cfg(unix)] use ::libc::pthread_t;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::cpu_set_t;
#[cfg(target_os = "freebsd")] use ::libc::cpuset_t;
use ::libc::timespec;


#[repr(C, packed)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __m128i
{
	a: u64,
	b: u64,
}

c!
{
	#include "rust_types.h"
	#include <rte_cycles.h>
	#include <rte_errno.h>
	#include <rte_ethdev.h>
	#include <rte_ether.h>
	#include <rte_lcore.h>
	#include <rte_mbuf.h>
	#include <rte_pci.h>
	#include <rte_ring.h>
	#include <rte_time.h>
	#include <stdlib.h>
}

c!
{
	#include <tle_dring.h>
	
	#[inline(always)]
	fn rust_tle_dring_count(dr: *const tle_dring as "const struct tle_dring *") -> uint32_t as "uint32_t"
	{
		return tle_dring_count(dr);
	}

	#[inline(always)]
	fn rust_tle_dring_reset(dr: *mut tle_dring as "struct tle_dring *")
	{
		tle_dring_reset(dr);
	}

	#[inline(always)]
	fn rust_tle_drb_calc_size(num: uint32_t as "uint32_t") -> size_t as "size_t"
	{
		return rust_tle_drb_calc_size(num);
	}

	#[inline(always)]
	fn rust_tle_dring_mc_dequeue(dr: *mut tle_dring as "struct tle_dring *", objs: *mut *const c_void as "const void **", nb_obj: uint32_t as "uint32_t", drbs: *mut *mut tle_drb as "struct tle_drb **", nb_drb: *mut uint32_t as "uint32_t *") -> uint32_t as "uint32_t"
	{
		return tle_dring_mp_enqueue(dr, objs, nb_obj, drbs, nb_drb);
	}

	#[inline(always)]
	fn rust_tle_dring_mp_enqueue(dr: *mut tle_dring as "struct tle_dring *", objs: *mut *const c_void as "const void **", nb_obj: uint32_t as "uint32_t", drbs: *mut *mut tle_drb as "struct tle_drb **", nb_drb: *mut uint32_t as "uint32_t *") -> uint32_t as "uint32_t"
	{
		return tle_dring_mp_enqueue(dr, objs, nb_obj, drbs, nb_drb);
	}

	#[inline(always)]
	fn rust_tle_dring_sc_dequeue(dr: *mut tle_dring as "struct tle_dring *", objs: *mut *const c_void as "const void **", nb_obj: uint32_t as "uint32_t", drbs: *mut *mut tle_drb as "struct tle_drb **", nb_drb: *mut uint32_t as "uint32_t *") -> uint32_t as "uint32_t"
	{
		return tle_dring_sc_dequeue(dr, objs, nb_obj, drbs, nb_drb);
	}

	#[inline(always)]
	fn rust_tle_dring_sp_enqueue(dr: *mut tle_dring as "struct tle_dring *", objs: *mut *const c_void as "const void **", nb_obj: uint32_t as "uint32_t", drbs: *mut *mut tle_drb as "struct tle_drb **", nb_drb: *mut uint32_t as "uint32_t *") -> uint32_t as "uint32_t"
	{
		return tle_dring_sp_enqueue(dr, objs, nb_obj, drbs, nb_drb);
	}
}

c!
{
	#include <tle_event.h>

	#[inline(always)]
	fn rust_tle_event_state(ev: *const tle_event as "const struct tle_event *") -> tle_ev_state as "enum tle_ev_state"
	{
		return tle_event_state(ev);
	}

	#[inline(always)]
	fn rust_tle_event_raise(ev: *mut tle_event as "struct tle_event *")
	{
		tle_event_raise(ev);
	}

	#[inline(always)]
	fn rust_tle_event_down(ev: *mut tle_event as "struct tle_event *")
	{
		tle_event_down(ev);
	}

	#[inline(always)]
	fn rust_tle_event_active(ev: *mut tle_event as "struct tle_event *", st: tle_ev_state as "enum tle_ev_state")
	{
		tle_event_active(ev, st);
	}

	#[inline(always)]
	fn rust_tle_event_idle(ev: *mut tle_event as "struct tle_event *")
	{
		tle_event_idle(ev);
	}

	#[inline(always)]
	fn rust_tle_evq_idle(evq: *mut tle_evq as "struct tle_evq *", ev: *mut *mut tle_event as "struct tle_event **", num: uint32_t as "uint32_t")
	{
		tle_evq_idle(evq, ev, num);
	}

	#[inline(always)]
	fn rust_tle_evq_get(evq: *mut tle_evq as "struct tle_evq *", evd: *mut *const c_void as "const void **", num: uint32_t as "uint32_t") -> uint32_t as "uint32_t"
	{
		return tle_evq_get(evq, evd, num);
	}
}

c!
{
	#include <rte_lpm.h>
	
	#[inline(always)]
	fn rust_rte_lpm_lookup(lpm: *mut rte_lpm as "struct rte_lpm *", ip: u32 as "uint32_t", next_hop: *mut u32 as "uint32_t *") -> c_int as "int"
	{
		return rte_lpm_lookup(lpm, ip, next_hop);
	}
}

c!
{
	#[inline(always)]
	fn rust_RTE_DEV_TO_PCI(device: *mut rte_device as "struct rte_device *") -> *mut rte_pci_device as "struct rte_pci_device *"
	{
		return RTE_DEV_TO_PCI(device);
	}
}

c!
{
	#include <rte_ip_frag.h>
	
	#[inline(always)]
	fn rust_rte_ip_frag_table_destroy(tbl: *mut rte_ip_frag_tbl as "struct rte_ip_frag_tbl *")
	{
		rte_ip_frag_table_destroy(tbl);
	}
	
	#[inline(always)]
	fn rust_rte_ipv4_frag_pkt_is_fragmented(hdr: *const ipv4_hdr as "const struct ipv4_hdr *") -> c_int as "int"
	{
		return rte_ipv4_frag_pkt_is_fragmented(hdr);
	}
	
	#[inline(always)]
	fn rust_rte_ipv6_frag_get_ipv6_fragment_header(hdr: *mut ipv6_hdr as "struct ipv6_hdr *") -> *mut ipv6_extension_fragment as "struct ipv6_extension_fragment *"
	{
		return rte_ipv6_frag_get_ipv6_fragment_header(hdr);
	}
}

c!
{
	#[inline(always)]
	fn rust_rte_errno() -> c_int as "int"
	{
		return rte_errno;
	}

	#[inline(always)]
	fn rust_rte_reset_errno()
	{
		rte_errno = 0;
	}

	#[inline(always)]
	fn rust_rte_lcore_id() -> c_uint as "unsigned"
	{
		return rte_lcore_id();
	}

	#[inline(always)]
	fn rust_rte_get_master_lcore() -> c_uint as "unsigned"
	{
		return rte_get_master_lcore();
	}

	#[inline(always)]
	fn rust_rte_lcore_count() -> c_uint as "unsigned"
	{
		return rte_lcore_count();
	}

	#[inline(always)]
	fn rust_rte_lcore_index(lcore_id: c_uint as "unsigned") -> c_int as "int"
	{
		return rte_lcore_index(lcore_id);
	}

	#[inline(always)]
	fn rust_rte_get_next_lcore(i: c_uint as "unsigned", skip_master: c_int as "int", wrap: c_int as "int") -> c_uint as "unsigned"
	{
		return rte_get_next_lcore(i, skip_master, wrap);
	}
}

c!
{
	#[inline(always)]
	fn rust_packetBufferOverhead() -> size_t as "size_t"
	{
		sizeof(struct rte_mbuf) + RTE_PKTMBUF_HEADROOM;
	}
}

c!
{
	#[inline(always)]
	fn rust_rte_eth_txmode_setBitFields(txmode: *mut rte_eth_txmode as "struct rte_eth_txmode *", hw_vlan_reject_tagged: bool as "bool_", hw_vlan_reject_untagged: bool as "bool_", hw_vlan_insert_pvid: bool as "bool_")
	{
		txmode->hw_vlan_reject_tagged = hw_vlan_reject_tagged;
		txmode->hw_vlan_reject_untagged = hw_vlan_reject_untagged;
		txmode->hw_vlan_insert_pvid = hw_vlan_insert_pvid;
	}
	
	#[inline(always)]
	fn rust_rte_eth_rxmode_setBitFields(rxmode: *mut rte_eth_rxmode as "struct rte_eth_rxmode *", header_split: bool as "bool_", hw_ip_checksum: bool as "bool_", hw_vlan_filter: bool as "bool_", hw_vlan_strip: bool as "bool_", hw_vlan_extend: bool as "bool_", jumbo_frame: bool as "bool_", hw_strip_crc: bool as "bool_", enable_scatter: bool as "bool_", enable_lro: bool as "bool_")
	{
		rxmode->header_split = header_split;
		rxmode->hw_ip_checksum = hw_ip_checksum;
		rxmode->hw_vlan_filter = hw_vlan_filter;
		rxmode->hw_vlan_strip = hw_vlan_strip;
		rxmode->hw_vlan_extend = hw_vlan_extend;
		rxmode->jumbo_frame = jumbo_frame;
		rxmode->hw_strip_crc = hw_strip_crc;
		rxmode->enable_scatter = enable_scatter;
		rxmode->enable_lro = enable_lro;
	}
	
	#[inline(always)]
	fn rust_rte_eth_link_getBitField_link_duplex(link: *const rte_eth_link as "const struct rte_eth_link *") -> uint16_t as "uint16_t"
	{
		return link->link_duplex;
	}
	
	#[inline(always)]
	fn rust_rte_eth_link_getBitField_link_autoneg(link: *const rte_eth_link as "const struct rte_eth_link *") -> uint16_t as "uint16_t"
	{
		return link->link_autoneg;
	}
	
	#[inline(always)]
	fn rust_rte_eth_link_getBitField_link_status(link: *const rte_eth_link as "const struct rte_eth_link *") -> uint16_t as "uint16_t"
	{
		return link->link_status;
	}
}

c! {
	#[inline(always)]
	fn rust_rte_eth_rx_burst(port_id: uint8_t as "uint8_t", queue_id: uint16_t as "uint16_t", rx_pkts: *mut *mut rte_mbuf as "struct rte_mbuf **", nb_pkts: uint16_t as "const uint16_t") -> uint16_t as "uint16_t"
	{
		return rte_eth_rx_burst(port_id, queue_id, rx_pkts, nb_pkts);
	}
	
	#[inline(always)]
	fn rust_rte_eth_rx_queue_count(port_id: uint8_t as "uint8_t", queue_id: uint16_t as "uint16_t") -> c_int as "int"
	{
		return rte_eth_rx_queue_count(port_id, queue_id);
	}
	
	#[inline(always)]
	fn rust_rte_eth_rx_descriptor_done(port_id: uint8_t as "uint8_t", queue_id: uint16_t as "uint16_t", offset: uint16_t as "uint16_t") -> c_int as "int"
	{
		return rte_eth_rx_descriptor_done(port_id, queue_id, offset);
	}
	
	#[inline(always)]
	fn rust_rte_eth_tx_burst(port_id: uint8_t as "uint8_t", queue_id: uint16_t as "uint16_t", tx_pkts: *mut *mut rte_mbuf as "struct rte_mbuf **", nb_pkt: uint16_t as "uint16_t") -> uint16_t as "uint16_t"
	{
		return rte_eth_tx_burst(port_id, queue_id, tx_pkts, nb_pkt);
	}
	
	#[inline(always)]
	fn rust_RTE_ETH_TX_BUFFER_SIZE(sz: uint16_t as "uint16_t") -> size_t as "size_t"
	{
		return RTE_ETH_TX_BUFFER_SIZE(sz);
	}
	
	#[inline(always)]
	fn rust_rte_eth_tx_buffer_flush(port_id: uint8_t as "uint8_t", queue_id: uint16_t as "uint16_t", buffer: *mut rte_eth_dev_tx_buffer as "struct rte_eth_dev_tx_buffer *") -> uint16_t as "uint16_t"
	{
		return rte_eth_tx_buffer_flush(port_id, queue_id, buffer);
	}
	
	#[inline(always)]
	fn rust_rte_eth_tx_buffer(port_id: uint8_t as "uint8_t", queue_id: uint16_t as "uint16_t", buffer: *mut rte_eth_dev_tx_buffer as "struct rte_eth_dev_tx_buffer *", tx_pkt: *mut rte_mbuf as "struct rte_mbuf *") -> uint16_t as "uint16_t"
	{
		return rte_eth_tx_buffer(port_id, queue_id, buffer, tx_pkt);
	}
}

c!
{
	#[inline(always)]
	fn rust_is_zero_ether_addr(ea: *const ether_addr as "const struct ether_addr *") -> c_int as "int"
	{
		return is_zero_ether_addr(ea);
	}

	#[inline(always)]
	fn rust_is_unicast_ether_addr(ea: *const ether_addr as "const struct ether_addr *") -> c_int as "int"
	{
		return is_unicast_ether_addr(ea);
	}

	#[inline(always)]
	fn rust_is_multicast_ether_addr(ea: *const ether_addr as "const struct ether_addr *") -> c_int as "int"
	{
		return is_multicast_ether_addr(ea);
	}

	#[inline(always)]
	fn rust_is_broadcast_ether_addr(ea: *const ether_addr as "const struct ether_addr *") -> c_int as "int"
	{
		return is_broadcast_ether_addr(ea);
	}

	#[inline(always)]
	fn rust_is_universal_ether_addr(ea: *const ether_addr as "const struct ether_addr *") -> c_int as "int"
	{
		return is_universal_ether_addr(ea);
	}

	#[inline(always)]
	fn rust_is_local_admin_ether_addr(ea: *const ether_addr as "const struct ether_addr *") -> c_int as "int"
	{
		return is_local_admin_ether_addr(ea);
	}

	#[inline(always)]
	fn rust_is_valid_assigned_ether_addr(ea: *const ether_addr as "const struct ether_addr *") -> c_int as "int"
	{
		return is_valid_assigned_ether_addr(ea);
	}
	
	#[inline(always)]
	fn rust_eth_random_addr(addr: *mut uint8_t as "uint8_t *")
	{
		eth_random_addr(addr);
	}
}

c! {
	#[inline(always)]
	fn rust_rte_pktmbuf_data_room_size(mp: *mut rte_mempool as "struct rte_mempool *") -> uint16_t as "uint16_t"
	{
		return rte_pktmbuf_data_room_size(mp);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_alloc(mp: *mut rte_mempool as "struct rte_mempool *") -> *mut rte_mbuf as "struct rte_mbuf *"
	{
		return rte_pktmbuf_alloc(mp);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_alloc_bulk(pool: *mut rte_mempool as "struct rte_mempool *", mbufs: *mut *mut rte_mbuf as "struct rte_mbuf **", count: u32 as "unsigned") -> c_int as "int"
	{
		return rte_pktmbuf_alloc_bulk(pool, mbufs, count);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_clone(md: *mut rte_mbuf as "struct rte_mbuf *", mp: *mut rte_mempool as "struct rte_mempool *") -> *mut rte_mbuf as "struct rte_mbuf *"
	{
		return rte_pktmbuf_clone(md, mp);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_reset(m: *mut rte_mbuf as "struct rte_mbuf *")
	{
		rte_pktmbuf_reset(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_attach(mi: *mut rte_mbuf as "struct rte_mbuf *", m: *mut rte_mbuf as "struct rte_mbuf *")
	{
		rte_pktmbuf_attach(mi, m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_detach(m: *mut rte_mbuf as "struct rte_mbuf *")
	{
		rte_pktmbuf_detach(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_free_seg(m: *mut rte_mbuf as "struct rte_mbuf *")
	{
		rte_pktmbuf_free_seg(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_free(m: *mut rte_mbuf as "struct rte_mbuf *")
	{
		rte_pktmbuf_free(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_refcnt_update(m: *mut rte_mbuf as "struct rte_mbuf *", v: int16_t as "int16_t")
	{
		rte_pktmbuf_refcnt_update(m, v);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_headroom(m: *const rte_mbuf as "const struct rte_mbuf *") -> uint16_t as "uint16_t"
	{
		return rte_pktmbuf_headroom(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_tailroom(m: *const rte_mbuf as "const struct rte_mbuf *") -> uint16_t as "uint16_t"
	{
		return rte_pktmbuf_tailroom(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_lastseg(m: *mut rte_mbuf as "struct rte_mbuf *") -> *mut rte_mbuf as "struct rte_mbuf *"
	{
		return rte_pktmbuf_lastseg(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_prepend(m: *const rte_mbuf as "struct rte_mbuf *", len: uint16_t as "uint16_t") -> *mut c_char as "char *"
	{
		return rte_pktmbuf_prepend(m, len);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_append(m: *const rte_mbuf as "struct rte_mbuf *", len: uint16_t as "uint16_t") -> *mut c_char as "char *"
	{
		return rte_pktmbuf_append(m, len);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_adj(m: *const rte_mbuf as "struct rte_mbuf *", len: uint16_t as "uint16_t") -> *mut c_char as "char *"
	{
		return rte_pktmbuf_adj(m, len);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_trim(m: *const rte_mbuf as "struct rte_mbuf *", len: uint16_t as "uint16_t") -> c_int as "int"
	{
		return rte_pktmbuf_trim(m, len);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_is_contiguous(m: *const rte_mbuf as "const struct rte_mbuf *") -> c_int as "int"
	{
		return rte_pktmbuf_is_contiguous(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_chain(head: *mut rte_mbuf as "struct rte_mbuf *", tail: *mut rte_mbuf as "struct rte_mbuf *") -> c_int as "int"
	{
		return rte_pktmbuf_chain(head, tail);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_mtophys_mtod_offset(m: *mut rte_mbuf as "struct rte_mbuf *", o: size_t as "size_t") -> *mut c_void as "void *"
	{
		return m->buf_addr + m->data_off + o;
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_mtophys_mtod(m: *mut rte_mbuf as "struct rte_mbuf *") -> *mut c_void as "void *"
	{
		return m->buf_addr + m->data_off;
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_mtophys_offset(m: *mut rte_mbuf as "struct rte_mbuf *", o: size_t as "size_t") -> phys_addr_t as "phys_addr_t"
	{
		return rte_pktmbuf_mtophys_offset(m, o);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_mtophys(m: *mut rte_mbuf as "struct rte_mbuf *") -> phys_addr_t as "phys_addr_t"
	{
		return rte_pktmbuf_mtophys(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_pkt_len(m: *mut rte_mbuf as "struct rte_mbuf *") -> uint32_t as "uint32_t"
	{
		return rte_pktmbuf_pkt_len(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_data_len(m: *mut rte_mbuf as "struct rte_mbuf *") -> uint16_t as "uint16_t"
	{
		return rte_pktmbuf_data_len(m);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_read(m: *mut rte_mbuf as "struct rte_mbuf *", off: uint32_t as "uint32_t", len: uint32_t as "uint32_t", buf: *mut c_void as "void *") -> *const c_void as "const void *"
	{
		return rte_pktmbuf_read(m, off, len, buf);
	}
}

// Functions not formally accepted for use by DPDK but essential for correct operation
c!
{
	#include <rte_memcpy.h>
	
	// This code derived from that originally proposed in a patch and rejected for a reason I do not know; it is essential to work correctly with mbufs with more than one segment
	// See http://dpdk.org/dev/patchwork/patch/17226/
	#[inline(always)]
	fn rust_rte_pktmbuf_write(m: *mut rte_mbuf as "const struct rte_mbuf *", off: uint32_t as "uint32_t", len: uint32_t as "uint32_t", buf: *const c_void as "const void *") -> c_int as "int"
	{
		char *dstA = rte_pktmbuf_mtod_offset(m, char *, off);
	
		if (buf == dstA)
		{
			return 0;
		}
		
		if (off + len <= rte_pktmbuf_data_len(m))
		{
			rte_memcpy(dstA, buf, len);
			return 0;
		}
		
		const struct rte_mbuf *seg = m;
		uint32_t buf_off = 0, copy_len;
		char *dst;
	
		if (off + len > rte_pktmbuf_pkt_len(m))
		{
			return -1;
		}
		
		// Find first relevant segment
		while (off >= rte_pktmbuf_data_len(seg))
		{
			off -= rte_pktmbuf_data_len(seg);
			seg = seg->next;
		}
	
		dst = rte_pktmbuf_mtod_offset(seg, char *, off);
		if (buf == dst)
		{
			return 0;
		}
		
		if (off + len <= rte_pktmbuf_data_len(seg))
		{
			rte_memcpy(dst, buf, len);
			return 0;
		}
		
		while (len > 0)
		{
			copy_len = rte_pktmbuf_data_len(seg) - off;
			if (copy_len > len)
			{
				copy_len = len;
			}
			dst = rte_pktmbuf_mtod_offset(seg, char *, off);
			rte_memcpy(dst, (const char *)buf + buf_off, copy_len);
			off = 0;
			buf_off += copy_len;
			len -= copy_len;
			seg = seg->next;
		}
	
		return 0;
	}
}

// Wrappers for bitfields
c!
{
	#[inline(always)]
	fn rust_rte_pktmbuf_setMajorLengthBitfields(m: *mut rte_mbuf as "struct rte_mbuf *", layer2HeaderLength: uint8_t as "uint8_t", layer3HeaderLength: uint8_t as "uint8_t", layer4HeaderLength: uint8_t as "uint8_t")
	{
		// See also _mbuf_tx_offload in TLDK example l4fwd pkt.c
		m->tx_offload = (uint64_t) layer2HeaderLength | ((uint64_t) layer3HeaderLength) << 7 | ((uint64_t) layer4HeaderLength) << 16;
	}
}

// Wrappers for inconvenient macros
c!
{
	#[inline(always)]
	fn rust_rte_pktmbuf_mtod(m: *mut rte_mbuf as "struct rte_mbuf *") -> *mut c_void as "void *"
	{
		return rte_pktmbuf_mtod(m, void *);
	}
	
	#[inline(always)]
	fn rust_rte_pktmbuf_mtod_offset(m: *mut rte_mbuf as "struct rte_mbuf *", o: uint16_t as "uint16_t") -> *mut c_void as "void *"
	{
		return rte_pktmbuf_mtod_offset(m, void *, o);
	}
}

c!
{
	#include <rte_net.h>

	#[inline(always)]
	fn rust_rte_net_intel_cksum_flags_prepare(m: *mut rte_mbuf as "struct rte_mbuf *", ol_flags: uint64_t as "uint64_t") -> c_int as "int"
	{
		return rte_net_intel_cksum_flags_prepare(m, ol_flags);
	}
	
	#[inline(always)]
	fn rust_rte_net_intel_cksum_prepare(m: *mut rte_mbuf as "struct rte_mbuf *") -> c_int as "int"
	{
		return rte_net_intel_cksum_prepare(m);
	}
}

c!
{
	#include <rte_ip.h>
	
	#[inline(always)]
	fn rust_rte_ipv4_cksum(ipv4_hdr: *const ipv4_hdr as "const struct ipv4_hdr *") -> uint16_t as "uint16_t"
	{
		return rte_ipv4_cksum(ipv4_hdr);
	}
}

c!
{
	#[inline(always)]
	fn rust_rte_timespec_to_ns(ts: *const timespec as "const struct timespec *") -> uint64_t as "uint64_t"
	{
		return rte_timespec_to_ns(ts);
	}
	
	#[inline(always)]
	fn rust_rte_ns_to_timespec(nsec: uint64_t as "uint64_t") -> timespec as "struct timespec"
	{
		return rte_ns_to_timespec(nsec);
	}
	
	#[inline(always)]
	fn rust_rte_rdtsc() -> uint64_t as "uint64_t"
	{
		return rte_rdtsc();
	}
	
	#[inline(always)]
	fn rust_rte_rdtsc_precise() -> uint64_t as "uint64_t"
	{
		return rte_rdtsc_precise();
	}
	
	#[inline(always)]
	fn rust_rte_get_tsc_cycles() -> uint64_t as "uint64_t"
	{
		return rte_get_tsc_cycles();
	}
	
	#[inline(always)]
	fn rust_rte_get_timer_cycles() -> uint64_t as "uint64_t"
	{
		return rte_get_timer_cycles();
	}
	
	#[inline(always)]
	fn rust_rte_get_timer_hz() -> uint64_t as "uint64_t"
	{
		return rte_get_timer_hz();
	}
	
	#[inline(always)]
	fn rust_rte_delay_ms(ms: c_uint as "unsigned")
	{
		rte_delay_ms(ms);
	}
}

c!
{
	#[inline(always)]
	fn rust_rte_ring_full(r: *const rte_ring as "const struct rte_ring *") -> c_int as "int"
	{
		return rte_ring_full(r);
	}

	#[inline(always)]
	fn rust_rte_ring_empty(r: *const rte_ring as "const struct rte_ring *") -> c_int as "int"
	{
		return rte_ring_empty(r);
	}

	#[inline(always)]
	fn rust_rte_ring_count(r: *const rte_ring as "const struct rte_ring *") -> c_uint as "unsigned"
	{
		return rte_ring_count(r);
	}

	#[inline(always)]
	fn rust_rte_ring_free_count(r: *const rte_ring as "const struct rte_ring *") -> c_uint as "unsigned"
	{
		return rte_ring_free_count(r);
	}
	
	#[inline(always)]
	fn rust_rte_ring_mp_enqueue(r: *mut rte_ring as "struct rte_ring *", obj: *mut c_void as "void *") -> c_int as "int"
	{
		return rte_ring_mp_enqueue(r, obj);
	}
	
	#[inline(always)]
	fn rust_rte_ring_sp_enqueue(r: *mut rte_ring as "struct rte_ring *", obj: *mut c_void as "void *") -> c_int as "int"
	{
		return rte_ring_sp_enqueue(r, obj);
	}
	
	#[inline(always)]
	fn rust_rte_ring_enqueue(r: *mut rte_ring as "struct rte_ring *", obj: *mut c_void as "void *") -> c_int as "int"
	{
		return rte_ring_enqueue(r, obj);
	}
}

c!
{
	#[inline(always)]
	fn rust_rte_ring_mp_enqueue_bulk(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *const c_void as "void * const *", n: c_uint as "unsigned") -> c_int as "int"
	{
		return rte_ring_mp_enqueue_bulk(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_sp_enqueue_bulk(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *const c_void as "void * const *", n: c_uint as "unsigned") -> c_int as "int"
	{
		return rte_ring_sp_enqueue_bulk(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_enqueue_bulk(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *const c_void as "void * const *", n: c_uint as "unsigned") -> c_int as "int"
	{
		return rte_ring_enqueue_bulk(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_mp_enqueue_burst(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *const c_void as "void * const *", n: c_uint as "unsigned") -> c_uint as "unsigned"
	{
		return rte_ring_mp_enqueue_burst(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_sp_enqueue_burst(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *const c_void as "void * const *", n: c_uint as "unsigned") -> c_uint as "unsigned"
	{
		return rte_ring_sp_enqueue_burst(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_enqueue_burst(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *const c_void as "void * const *", n: c_uint as "unsigned") -> c_uint as "unsigned"
	{
		return rte_ring_enqueue_burst(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_mc_dequeue(r: *mut rte_ring as "struct rte_ring *", obj_p: *mut *mut c_void as "void * *") -> c_int as "int"
	{
		return rte_ring_mc_dequeue(r, obj_p);
	}

	#[inline(always)]
	fn rust_rte_ring_sc_dequeue(r: *mut rte_ring as "struct rte_ring *", obj_p: *mut *mut c_void as "void * *") -> c_int as "int"
	{
		return rte_ring_sc_dequeue(r, obj_p);
	}
	
	#[inline(always)]
	fn rust_rte_ring_dequeue(r: *mut rte_ring as "struct rte_ring *", obj_p: *mut *mut c_void as "void * *") -> c_int as "int"
	{
		return rte_ring_dequeue(r, obj_p);
	}
	
	#[inline(always)]
	fn rust_rte_ring_mc_dequeue_bulk(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *mut c_void as "void * *", n: c_uint as "unsigned") -> c_int as "int"
	{
		return rte_ring_mc_dequeue_bulk(r, obj_table, n);
	}

	#[inline(always)]
	fn rust_rte_ring_sc_dequeue_bulk(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *mut c_void as "void * *", n: c_uint as "unsigned") -> c_int as "int"
	{
		return rte_ring_sc_dequeue_bulk(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_dequeue_bulk(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *mut c_void as "void * *", n: c_uint as "unsigned") -> c_int as "int"
	{
		return rte_ring_dequeue_bulk(r, obj_table, n);
	}

	#[inline(always)]
	fn rust_rte_ring_mc_dequeue_burst(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *mut c_void as "void * *", n: c_uint as "unsigned") -> c_uint as "unsigned"
	{
		return rte_ring_mc_dequeue_burst(r, obj_table, n);
	}

	#[inline(always)]
	fn rust_rte_ring_sc_dequeue_burst(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *mut c_void as "void * *", n: c_uint as "unsigned") -> c_uint as "unsigned"
	{
		return rte_ring_sc_dequeue_burst(r, obj_table, n);
	}
	
	#[inline(always)]
	fn rust_rte_ring_dequeue_burst(r: *mut rte_ring as "struct rte_ring *", obj_table: *mut *mut c_void as "void * *", n: c_uint as "unsigned") -> c_uint as "unsigned"
	{
		return rte_ring_dequeue_burst(r, obj_table, n);
	}
}
