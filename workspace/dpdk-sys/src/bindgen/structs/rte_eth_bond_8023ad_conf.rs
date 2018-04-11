// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_bond_8023ad_conf
{
	pub fast_periodic_ms: u32,
	pub slow_periodic_ms: u32,
	pub short_timeout_ms: u32,
	pub long_timeout_ms: u32,
	pub aggregate_wait_timeout_ms: u32,
	pub tx_period_ms: u32,
	pub rx_marker_period_ms: u32,
	pub update_timeout_ms: u32,
	pub slowrx_cb: rte_eth_bond_8023ad_ext_slowrx_fn,
	pub agg_selection: rte_bond_8023ad_agg_selection,
}

impl Default for rte_eth_bond_8023ad_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_bond_8023ad_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_bond_8023ad_conf {{ agg_selection: {:?} }}", self.agg_selection)
	}
}
