// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_net_crc_calc(data: *const c_void, data_len: u32, type_: rte_net_crc_type) -> u32;
	pub fn rte_net_crc_set_alg(alg: rte_net_crc_alg);
	pub fn rte_net_get_ptype(m: *const rte_mbuf, hdr_lens: *mut rte_net_hdr_lens, layers: u32) -> u32;
	pub fn rte_net_make_rarp_packet(mpool: *mut rte_mempool, mac: *const ether_addr) -> *mut rte_mbuf;
	pub fn rte_net_skip_ip6_ext(proto: u16, m: *const rte_mbuf, off: *mut u32, frag: *mut c_int) -> c_int;
}
