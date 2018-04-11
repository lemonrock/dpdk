// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_desc
{
	pub size: usize,
	pub attr: rte_flow_attr,
	pub items: *mut rte_flow_item,
	pub actions: *mut rte_flow_action,
	pub data: __IncompleteArrayField<u8>,
}

impl Default for rte_flow_desc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_desc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_desc {{ attr: {:?}, items: {:?}, actions: {:?}, data: {:?} }}", self.attr, self.items, self.actions, self.data)
	}
}
