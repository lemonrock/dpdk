// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_intr_handle
{
	pub _1: rte_intr_handle_1,
	pub fd: c_int,
	pub type_: rte_intr_handle_type,
	pub max_intr: u32,
	pub nb_efd: u32,
	pub efd_counter_size: u8,
	pub efds: [c_int; 32usize],
	pub elist: [rte_epoll_event; 32usize],
	pub intr_vec: *mut c_int,
}

impl Default for rte_intr_handle
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_intr_handle
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_intr_handle {{ _1: {:?}, type: {:?}, efds: [{}], elist: [{}], intr_vec: {:?} }}",
			self._1,
			self.type_,
			self.efds
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
			self.elist
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
			self.intr_vec
		)
	}
}
