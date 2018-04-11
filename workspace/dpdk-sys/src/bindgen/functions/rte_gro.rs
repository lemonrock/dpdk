// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_gro_ctx_create(param: *const rte_gro_param) -> *mut c_void;
	pub fn rte_gro_ctx_destroy(ctx: *mut c_void);
	pub fn rte_gro_get_pkt_count(ctx: *mut c_void) -> u64;
	pub fn rte_gro_reassemble(pkts: *mut *mut rte_mbuf, nb_pkts: u16, ctx: *mut c_void) -> u16;
	pub fn rte_gro_reassemble_burst(pkts: *mut *mut rte_mbuf, nb_pkts: u16, param: *const rte_gro_param) -> u16;
	pub fn rte_gro_timeout_flush(ctx: *mut c_void, timeout_cycles: u64, gro_types: u64, out: *mut *mut rte_mbuf, max_nb_out: u16) -> u16;
}
