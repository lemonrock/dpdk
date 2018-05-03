// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_mempool_ops
{
	pub name: [c_char; 32usize],
	pub alloc: rte_mempool_alloc_t,
	pub free: rte_mempool_free_t,
	pub enqueue: rte_mempool_enqueue_t,
	pub dequeue: rte_mempool_dequeue_t,
	pub get_count: rte_mempool_get_count,
	pub calc_mem_size: rte_mempool_calc_mem_size_t,
	pub populate: rte_mempool_populate_t,
	pub get_info: rte_mempool_get_info_t,
	pub dequeue_contig_blocks: rte_mempool_dequeue_contig_blocks_t,
	pub __bindgen_padding_0: [u64; 3usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_mempool_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mempool_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_mempool_ops {{ name: [{}] }}",
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
				.collect::<String>()
		)
	}
}
