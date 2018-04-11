// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_kni_ops
{
	pub port_id: u16,
	pub change_mtu: Option<unsafe extern "C" fn(port_id: u16, new_mtu: c_uint) -> c_int>,
	pub config_network_if: Option<unsafe extern "C" fn(port_id: u16, if_up: u8) -> c_int>,
	pub config_mac_address: Option<unsafe extern "C" fn(port_id: u16, mac_addr: *mut u8) -> c_int>,
	pub config_promiscusity: Option<unsafe extern "C" fn(port_id: u16, to_on: u8) -> c_int>,
}

impl Default for rte_kni_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_kni_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_kni_ops {{ change_mtu: {:?}, config_network_if: {:?}, config_mac_address: {:?}, config_promiscusity: {:?} }}", self.change_mtu, self.config_network_if, self.config_mac_address, self.config_promiscusity)
	}
}
