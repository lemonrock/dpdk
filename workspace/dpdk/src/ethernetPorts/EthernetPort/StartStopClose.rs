// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn start(&self) -> Result<(), i32>
	{
		let result = unsafe { rte_eth_dev_start(self.portIdentifier()) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			if likely(result < 0)
			{
				Err(result)
			}
			else
			{
				panic!("Unexpected positive error code '{}' from rte_eth_dev_start()", result);
			}
		}
	}

	#[inline(always)]
	pub fn stop(&self)
	{
		unsafe { rte_eth_dev_stop(self.portIdentifier()) }
	}

	#[inline(always)]
	pub fn close(&self)
	{
		unsafe { rte_eth_dev_close(self.portIdentifier()) };
	}
}
