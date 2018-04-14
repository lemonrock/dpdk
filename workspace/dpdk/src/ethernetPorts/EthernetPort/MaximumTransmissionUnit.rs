// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	// From RFC 791 (ETHER_MIN_MTU)
	pub const MinimumTransmissionUnitSize: u16 = 68;

	#[inline(always)]
	pub fn getMaximumTransmissionUnit(&self) -> u16
	{
		let mut mtu = unsafe { uninitialized() };

		let result = unsafe { rte_eth_dev_get_mtu(self.portIdentifier(), &mut mtu) };
		if likely(result == 0)
		{
			debug_assert!(mtu >= Self::MinimumTransmissionUnitSize, "mtu '{}' must be equal to or greater than the MinimumTransmissionUnitSize, '{}'", mtu, Self::MinimumTransmissionUnitSize);
			mtu
		}
		else
		{
			forget(mtu);

			match result
			{
				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_mtu()", result),
			}
		}
	}

	#[inline(always)]
	pub fn setMaximumTransmissionUnit(&self, sizeInBytes: u16) -> Result<(), CouldNotSetMaximumTransmissionUnitError>
	{
		debug_assert!(sizeInBytes >= Self::MinimumTransmissionUnitSize, "sizeInBytes '{}' must be equal to or greater than the MinimumTransmissionUnitSize, '{}'", sizeInBytes, Self::MinimumTransmissionUnitSize);

		let result = unsafe { rte_eth_dev_set_mtu(self.portIdentifier(), sizeInBytes) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(CouldNotSetMaximumTransmissionUnitError::IsUnsupportedByTheHardware),
				NegativeE::EBUSY => Err(CouldNotSetMaximumTransmissionUnitError::CanNotBeSetWhilstEthernetPortIsRunning), // Not whilst running

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
				NegativeE::EINVAL => panic!("The MTU '{}' is invalid", sizeInBytes),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_set_mtu()", result),
			}
		}
	}
}
