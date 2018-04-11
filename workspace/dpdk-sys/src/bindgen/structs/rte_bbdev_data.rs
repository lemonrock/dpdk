// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_bbdev_data
{
	pub name: [c_char; 64usize],
	pub dev_private: *mut c_void,
	pub num_queues: u16,
	pub queues: *mut rte_bbdev_queue_data,
	pub dev_id: u16,
	pub socket_id: c_int,
	pub started: bool,
	pub process_cnt: rte_atomic16_t,
}

impl Default for rte_bbdev_data
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_bbdev_data
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_bbdev_data {{ name: [{}], dev_private: {:?}, queues: {:?}, process_cnt: {:?} }}",
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
			self.dev_private,
			self.queues,
			self.process_cnt
		)
	}
}
