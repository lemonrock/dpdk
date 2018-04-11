// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_get_ptype_inner_l2_name(ptype: u32) -> *const c_char;
	pub fn rte_get_ptype_inner_l3_name(ptype: u32) -> *const c_char;
	pub fn rte_get_ptype_inner_l4_name(ptype: u32) -> *const c_char;
	pub fn rte_get_ptype_l2_name(ptype: u32) -> *const c_char;
	pub fn rte_get_ptype_l3_name(ptype: u32) -> *const c_char;
	pub fn rte_get_ptype_l4_name(ptype: u32) -> *const c_char;
	pub fn rte_get_ptype_name(ptype: u32, buf: *mut c_char, buflen: usize) -> c_int;
	pub fn rte_get_ptype_tunnel_name(ptype: u32) -> *const c_char;
}
