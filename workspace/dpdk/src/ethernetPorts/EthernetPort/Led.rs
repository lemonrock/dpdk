// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn turnLedOn(&self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_led_on(self.portIdentifier()) };
		if likely!(result == 0)
		{
			return Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_led_on()", result),
			}
		}
	}

	#[inline(always)]
	pub fn turnLedOff(&self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_led_off(self.portIdentifier()) };
		if likely!(result == 0)
		{
			return Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_led_off()", result),
			}
		}
	}
}
