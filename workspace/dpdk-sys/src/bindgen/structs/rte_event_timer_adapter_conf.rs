// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_event_timer_adapter_conf
{
	pub event_dev_id: u8,
	pub timer_adapter_id: u16,
	pub socket_id: u32,
	pub clk_src: rte_event_timer_adapter_clk_src,
	pub timer_tick_ns: u64,
	pub max_tmo_ns: u64,
	pub nb_timers: u64,
	pub flags: u64,
}

impl Default for rte_event_timer_adapter_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_timer_adapter_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_timer_adapter_conf {{ clk_src: {:?} }}", self.clk_src)
	}
}
