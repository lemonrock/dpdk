// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension trait for `rte_eth_link`.
#[allow(non_camel_case_types)]
pub trait rte_eth_linkExt
{
	/// Returns `None` if the link is down.
	///
	/// Returns `Some(is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second)` if the link is up.
	#[inline(always)]
	fn is_down(&self) -> bool;
	
	/// Only valid if link is up.
	///
	/// Returns `(is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second)`.
	#[inline(always)]
	fn if_is_up(&self) -> (bool, bool, u32);
}

impl rte_eth_linkExt for rte_eth_link
{
	#[inline(always)]
	fn is_down(&self) -> bool
	{
		self.link_status() == 0
	}
	
	#[inline(always)]
	fn if_is_up(&self) -> (bool, bool, u32)
	{
		let is_full_duplex = self.link_duplex() == 1;
		let was_auto_negotiated = self.link_autoneg() == 1;
		let speed_in_megabits_per_second = self.link_speed;
		
		(is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second)
	}
}
