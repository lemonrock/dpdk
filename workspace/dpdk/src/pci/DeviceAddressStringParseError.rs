// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum DeviceAddressStringParseError
	{
		LengthIsWrong(length: usize)
		{
			display("Length should be '{}' but was '{}'", PciBusInformation::NumberOfBytesInPciAddressString, length)
		}
		
		NoDomain
		
		CouldNotParseDomain(value: String, cause: ParseIntError)
		{
			display("Could not parse domain as u16 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		NoBus
		
		CouldNotParseBus(value: String, cause: ParseIntError)
		{
			display("Could not parse bus as u8 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		NoDeviceIdentifier
		
		CouldNotParseDeviceIdentifier(value: String, cause: ParseIntError)
		{
			display("Could not parse deviceIdentifier as u8 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		NoFunction
		
		CouldNotParseFunction(value: String, cause: ParseIntError)
		{
			display("Could not parse function as u8 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		FunctionExceeds4BitValue(value: u8)
		{
			display("Parsed function exceeds 4-bit value (ie is 16 or more) '{}'", value)
		}
	}
}
