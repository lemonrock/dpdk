// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_item_vxlan_gpe
{
	pub flags: u8,
	pub rsvd0: [u8; 2usize],
	pub protocol: u8,
	pub vni: [u8; 3usize],
	pub rsvd1: u8,
}

impl Default for rte_flow_item_vxlan_gpe
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_item_vxlan_gpe
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_item_vxlan_gpe {{ rsvd0: {:?}, vni: {:?} }}", self.rsvd0, self.vni)
	}
}
