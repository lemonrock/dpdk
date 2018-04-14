// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_devargs
{
	pub next: rte_devargs_1,
	pub type_: rte_devtype,
	pub policy: rte_dev_policy,
	pub bus: *mut rte_bus,
	pub name: [c_char; 64usize],
	pub args: *mut c_char,
}

impl Default for rte_devargs
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_devargs
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_devargs {{ next: {:?}, type: {:?}, policy: {:?}, bus: {:?}, name: [{}], args: {:?} }}",
			self.next,
			self.type_,
			self.policy,
			self.bus,
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.args
		)
	}
}
