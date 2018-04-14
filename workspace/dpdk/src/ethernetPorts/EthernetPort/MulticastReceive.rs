// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn enableAllMulticastReceive(&self)
	{
		unsafe { rte_eth_allmulticast_enable(self.portIdentifier()) };
	}

	#[inline(always)]
	pub fn disableAllMulticastReceive(&self)
	{
		unsafe { rte_eth_allmulticast_disable(self.portIdentifier()) };
	}

	#[inline(always)]
	pub fn isReceiveAllMulticast(&self) -> Result<bool, ()>
	{
		let result = unsafe { rte_eth_allmulticast_get(self.portIdentifier()) };
		if unlikely(result == -1)
		{
			Err(())
		}
		else
		{
			Ok(isTrue(result))
		}
	}

	#[inline(always)]
	pub fn setMulticastMediaAccessControlAddressesToFilter(&self, multicastSet: &HashSet<MediaAccessControlAddress>) -> Result<(), UnsupportedOrFullError>
	{
		let length = multicastSet.len() as uint32_t;
		if length == 0
		{
			return Ok(());
		}
		let mut list: Vec<MediaAccessControlAddress> = Vec::with_capacity(length as usize);

		for multicastMediaAccessControlAddress in multicastSet
		{
			list.push(*multicastMediaAccessControlAddress);
		}
		let mut set = list.as_mut_slice();

		let result = unsafe { rte_eth_dev_set_mc_addr_list(self.portIdentifier(), set.internalMutablePointer(), length) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedOrFullError::IsUnsupportedByTheHardware),
				NegativeE::ENOSPC => Err(UnsupportedOrFullError::MaximumNumberOfItemsAssigned),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_set_mc_addr_list()", result),
			}
		}
	}

	#[inline(always)]
	pub fn clearMulticastMediaAccessControlAddressesToFilter(&self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_dev_set_mc_addr_list(self.portIdentifier(), null_mut(), 0) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENOSPC => panic!("Maximum number of items assigned but we tried to clear in rte_eth_dev_set_mc_addr_list()"),
				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_set_mc_addr_list()", result),
			}
		}
	}
}
