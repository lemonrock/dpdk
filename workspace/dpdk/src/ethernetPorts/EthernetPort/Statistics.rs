// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn resetStatistics(&self)
	{
		unsafe { rte_eth_stats_reset(self.portIdentifier()) }
	}

	#[inline(always)]
	pub fn getStatistics(&self) -> Result<rte_eth_stats, i32>
	{
		let mut statistics = unsafe { uninitialized() };
		let result = unsafe { rte_eth_stats_get(self.portIdentifier(), &mut statistics) };
		if likely!(result == 0)
		{
			Ok(statistics)
		}
		else
		{
			forget(statistics);
			Err(result)
		}
	}
}
