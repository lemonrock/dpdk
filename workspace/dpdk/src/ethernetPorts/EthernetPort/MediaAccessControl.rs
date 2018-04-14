// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn setDefaultMediaAccessControlAddress(&self, media_access_control_address: MediaAccessControlAddress) -> Result<(), UnsupportedByHardwareError>
	{
		let mut value = media_access_control_address.0;
		let result = unsafe { rte_eth_dev_default_mac_addr_set(self.portIdentifier(), &mut value) };

		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err
			(
				match result
				{
					NegativeE::ENOTSUP => UnsupportedByHardwareError::IsUnsupportedByTheHardware,

					NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
					NegativeE::EINVAL => panic!("The Media Access Control Address '{}' is invalid", media_access_control_address),

					_ => panic!("Unexpected error code '{}' from rte_eth_dev_default_mac_addr_set()", result),
				}
			)
		}
	}

	#[inline(always)]
	pub fn getDefaultMediaAccessControlAddress(&self) -> MediaAccessControlAddress
	{
		let mut value = unsafe { uninitialized() };
		unsafe { rte_eth_macaddr_get(self.portIdentifier(), &mut value) };
		MediaAccessControlAddress(value)
	}

	const NoVMDqPool: u32 = 0;

	#[inline(always)]
	pub fn addMediaAccessControlAddress(&self, media_access_control_address: MediaAccessControlAddress, vmdqPoolIndex: Option<u6>) -> Result<(), UnsupportedOrFullError>
	{
		let pool = match vmdqPoolIndex
		{
			None => Self::NoVMDqPool,
			Some(pool) =>
			{
				assert_ne!(pool, 0, "vmdqPoolIndex can not be zero");
				assert!(pool < 64, "vmdqPoolIndex must be less than 64, not '{}'", pool);
				pool as u32
			}
		};

		let mut value = media_access_control_address.0;
		let result = unsafe { rte_eth_dev_mac_addr_add(self.portIdentifier(), &mut value, pool) };

		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err
			(
				match result
				{
					NegativeE::ENOTSUP => UnsupportedOrFullError::IsUnsupportedByTheHardware,
					NegativeE::ENOSPC => UnsupportedOrFullError::MaximumNumberOfItemsAssigned,

					NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
					NegativeE::EINVAL => panic!("The Media Access Control Address '{}' is invalid", media_access_control_address),

					_ => panic!("Unexpected error code '{}' from rte_eth_dev_mac_addr_add()", result),
				}
			)
		}
	}

	// Successful even if media_access_control_address is not assigned to EthernetPort
	#[inline(always)]
	pub fn removeMediaAccessControlAddress(&self, media_access_control_address: MediaAccessControlAddress) -> Result<(), RemoveMediaAccessControlAddressError>
	{
		let mut value = media_access_control_address.0;
		let result = unsafe { rte_eth_dev_mac_addr_remove(self.portIdentifier(), &mut value) };

		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err
			(
				match result
				{
					NegativeE::ENOTSUP => RemoveMediaAccessControlAddressError::IsUnsupportedByTheHardware,
					NegativeE::EADDRINUSE => RemoveMediaAccessControlAddressError::CanNotRemoveDefaultMediaAccessControlAddress,

					NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

					_ => panic!("Unexpected error code '{}' from rte_eth_dev_mac_addr_remove()", result),
				}
			)
		}
	}
}
