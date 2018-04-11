// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#include "lib.h"



// Static wrappers generated in bindgen-wrapper.conf.d/configuration.sh
#include "bindgen/c/lib.c"




int rust_rte_errno()
{
	return rte_errno;
}

void rust_rte_reset_errno()
{
	rte_errno = 0;
}



uint64_t rust_rte_timespec_to_ns(const struct timespec * ts)
{
	return rte_timespec_to_ns(ts);
}

struct timespec rust_rte_ns_to_timespec(uint64_t nsec)
{
	return rte_ns_to_timespec(nsec);
}



uint64_t rust_rte_rdtsc()
{
	return rte_rdtsc();
}

uint64_t rust_rte_rdtsc_precise()
{
	return rte_rdtsc_precise();
}

uint64_t rust_rte_get_tsc_cycles()
{
	return rte_get_tsc_cycles();
}

uint64_t rust_rte_get_timer_cycles()
{
	return rte_get_timer_cycles();
}

uint64_t rust_rte_get_timer_hz()
{
	return rte_get_timer_hz();
}

void rust_rte_delay_ms(unsigned ms)
{
	rte_delay_ms(ms);
}




struct rte_pci_device * rust_RTE_DEV_TO_PCI(struct rte_device * device)
{
	return RTE_DEV_TO_PCI(device);
}



// Wrapper for macro.
void * rust_rte_pktmbuf_mtod(struct rte_mbuf * m)
{
	return rte_pktmbuf_mtod(m, void *);
}

// Wrapper for macro.
void * rust_rte_pktmbuf_mtod_offset(struct rte_mbuf * m, uint16_t o)
{
	return rte_pktmbuf_mtod_offset(m, void *, o);
}



void rust_rte_mov16(uint8_t * dst, const uint8_t * src)
{
	rte_mov16(dst, src);
}

void rust_rte_mov32(uint8_t * dst, const uint8_t * src)
{
	rte_mov32(dst, src);
}

void rust_rte_mov64(uint8_t * dst, const uint8_t * src)
{
	rte_mov64(dst, src);
}

void rust_rte_mov128(uint8_t * dst, const uint8_t * src)
{
	rte_mov128(dst, src);
}

void * rust_rte_memcpy_generic(void * dst, const void * src, size_t n)
{
	return rte_memcpy_generic(dst, src, n);
}



// This code derived from that originally proposed in a patch and rejected for a reason I do not know; it is essential to work correctly with mbufs with more than one segment.
// See http://dpdk.org/dev/patchwork/patch/17226/
int rust_rte_pktmbuf_write(const struct rte_mbuf * m, uint32_t off, uint32_t len, const void * buf)
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
// Wrappers for bitfields
//c!
//{
//	#[inline(always)]
//	fn rust_rte_pktmbuf_setMajorLengthBitfields(m: *mut rte_mbuf as "struct rte_mbuf *", layer2HeaderLength: uint8_t as "uint8_t", layer3HeaderLength: uint8_t as "uint8_t", layer4HeaderLength: uint8_t as "uint8_t")
//	{
//		// See also _mbuf_tx_offload in TLDK example l4fwd pkt.c
//		m->tx_offload = (uint64_t) layer2HeaderLength | ((uint64_t) layer3HeaderLength) << 7 | ((uint64_t) layer4HeaderLength) << 16;
//	}
//}

//

// c!
// {
// 	#[inline(always)]
// 	fn rust_packetBufferOverhead() -> size_t as "size_t"
// 	{
// 		sizeof(struct rte_mbuf) + RTE_PKTMBUF_HEADROOM;
// 	}
// }

//	fn rust_rte_eth_txmode_setBitFields(txmode: *mut rte_eth_txmode as "struct rte_eth_txmode *", hw_vlan_reject_tagged: bool as "bool_", hw_vlan_reject_untagged: bool as "bool_", hw_vlan_insert_pvid: bool as "bool_")
//	{
//		txmode->hw_vlan_reject_tagged = hw_vlan_reject_tagged;
//		txmode->hw_vlan_reject_untagged = hw_vlan_reject_untagged;
//		txmode->hw_vlan_insert_pvid = hw_vlan_insert_pvid;
//	}
//
//	#[inline(always)]
//	fn rust_rte_eth_rxmode_setBitFields(rxmode: *mut rte_eth_rxmode as "struct rte_eth_rxmode *", header_split: bool as "bool_", hw_ip_checksum: bool as "bool_", hw_vlan_filter: bool as "bool_", hw_vlan_strip: bool as "bool_", hw_vlan_extend: bool as "bool_", jumbo_frame: bool as "bool_", hw_strip_crc: bool as "bool_", enable_scatter: bool as "bool_", enable_lro: bool as "bool_")
//	{
//		rxmode->header_split = header_split;
//		rxmode->hw_ip_checksum = hw_ip_checksum;
//		rxmode->hw_vlan_filter = hw_vlan_filter;
//		rxmode->hw_vlan_strip = hw_vlan_strip;
//		rxmode->hw_vlan_extend = hw_vlan_extend;
//		rxmode->jumbo_frame = jumbo_frame;
//		rxmode->hw_strip_crc = hw_strip_crc;
//		rxmode->enable_scatter = enable_scatter;
//		rxmode->enable_lro = enable_lro;
//	}
//
//	#[inline(always)]
//	fn rust_rte_eth_link_getBitField_link_duplex(link: *const rte_eth_link as "const struct rte_eth_link *") -> uint16_t as "uint16_t"
//	{
//		return link->link_duplex;
//	}
//
//	#[inline(always)]
//	fn rust_rte_eth_link_getBitField_link_autoneg(link: *const rte_eth_link as "const struct rte_eth_link *") -> uint16_t as "uint16_t"
//	{
//		return link->link_autoneg;
//	}
//
//	#[inline(always)]
//	fn rust_rte_eth_link_getBitField_link_status(link: *const rte_eth_link as "const struct rte_eth_link *") -> uint16_t as "uint16_t"
//	{
//		return link->link_status;
//	}
//}
