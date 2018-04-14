// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn enablePromiscuousReceive(&self)
	{
		unsafe { rte_eth_promiscuous_enable(self.portIdentifier()) };
	}

	#[inline(always)]
	pub fn disablePromiscuousReceive(&self)
	{
		unsafe { rte_eth_promiscuous_disable(self.portIdentifier()) };
	}

	#[inline(always)]
	pub fn isReceivePromiscuous(&self) -> Result<bool, ()>
	{
		let result = unsafe { rte_eth_promiscuous_get(self.portIdentifier()) };
		if unlikely(result == -1)
		{
			Err(())
		}
		else
		{
			Ok(isTrue(result))
		}
	}
}
