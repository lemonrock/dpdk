// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_vmdq_rx_conf
{
	pub nb_queue_pools: rte_eth_nb_pools,
	pub enable_default_pool: u8,
	pub default_pool: u8,
	pub enable_loop_back: u8,
	pub nb_pool_maps: u8,
	pub rx_mode: u32,
	pub pool_map: [rte_eth_vmdq_rx_conf__bindgen_ty_1; 64usize],
}

impl Default for rte_eth_vmdq_rx_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_vmdq_rx_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_eth_vmdq_rx_conf {{ nb_queue_pools: {:?}, pool_map: [{}] }}",
			self.nb_queue_pools,
			self.pool_map
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
