// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn getEepromSize(&self) -> Result<u31, UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_dev_get_eeprom_length(self.portIdentifier()) };
		if likely!(result >= 0)
		{
			Ok(result as u31)
		}
		else
		{
			Err
			(
				match result
				{
					NegativeE::ENOTSUP => UnsupportedByHardwareError::IsUnsupportedByTheHardware,

					NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
					
					otherDeviceDriverError if otherDeviceDriverError.is_negative() => UnsupportedByHardwareError::IsUnsupportedByTheHardware,
				
					_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_eeprom_length()", result),
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn getEepromInformation(&self) -> Result<EepromInformation, UnsupportedByHardwareError>
	{
		let mut value = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_get_eeprom(self.portIdentifier(), &mut value) };
		if likely!(result >= 0)
		{
			Ok(EepromInformation(value))
		}
		else
		{
			Err
			(
				match result
				{
					NegativeE::ENOTSUP => UnsupportedByHardwareError::IsUnsupportedByTheHardware,

					NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
					
					otherDeviceDriverError if otherDeviceDriverError.is_negative() => UnsupportedByHardwareError::IsUnsupportedByTheHardware,
				
					_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_eeprom()", result),
				}
			)
		}
	}
	
	// Doesn't really mutate, but the C API is horrible
	#[inline(always)]
	pub fn reprogramEeprom(&self, eepromInformation: &EepromInformation) -> Result<(), UnsupportedByHardwareError>
	{
		let mut value = eepromInformation.0;
		let result = unsafe { rte_eth_dev_set_eeprom(self.portIdentifier(), &mut value) };
		if likely!(result >= 0)
		{
			Ok(())
		}
		else
		{
			forget(value);
			
			Err
			(
				match result
				{
					NegativeE::ENOTSUP => UnsupportedByHardwareError::IsUnsupportedByTheHardware,

					NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
					
					otherDeviceDriverError if otherDeviceDriverError.is_negative() => UnsupportedByHardwareError::IsUnsupportedByTheHardware,
				
					_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_eeprom()", result),
				}
			)
		}
	}
}
