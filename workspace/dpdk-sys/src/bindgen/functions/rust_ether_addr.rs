// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rust_eth_random_addr(addr: *mut u8);
	pub fn rust_ether_addr_copy(ea_from: *const ether_addr, ea_to: *mut ether_addr);
	pub fn rust_ether_format_addr(buf: *mut c_char, size: u16, eth_addr: *const ether_addr);
	pub fn rust_is_broadcast_ether_addr(ea: *const ether_addr) -> c_int;
	pub fn rust_is_local_admin_ether_addr(ea: *const ether_addr) -> c_int;
	pub fn rust_is_multicast_ether_addr(ea: *const ether_addr) -> c_int;
	pub fn rust_is_same_ether_addr(ea1: *const ether_addr, ea2: *const ether_addr) -> c_int;
	pub fn rust_is_unicast_ether_addr(ea: *const ether_addr) -> c_int;
	pub fn rust_is_universal_ether_addr(ea: *const ether_addr) -> c_int;
	pub fn rust_is_valid_assigned_ether_addr(ea: *const ether_addr) -> c_int;
	pub fn rust_is_zero_ether_addr(ea: *const ether_addr) -> c_int;
}
