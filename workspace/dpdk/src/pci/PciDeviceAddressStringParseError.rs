// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// Device address string parse error.
	#[derive(Debug)]
	pub enum PciDeviceAddressStringParseError
	{
		#[allow(missing_docs)]
		LengthIsWrong(length: usize)
		{
			display("Length should be '{}' but was '{}'", PciBusInformation::NumberOfBytesInPciAddressString, length)
		}
		
		#[allow(missing_docs)]
		NoDomain
		{
		}
		
		#[allow(missing_docs)]
		CouldNotParseDomain(value: String, cause: ParseIntError)
		{
			display("Could not parse domain as u16 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		#[allow(missing_docs)]
		NoBus
		{
		}
		
		#[allow(missing_docs)]
		CouldNotParseBus(value: String, cause: ParseIntError)
		{
			display("Could not parse bus as u8 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		#[allow(missing_docs)]
		NoDeviceIdentifier
		{
		}
		
		#[allow(missing_docs)]
		CouldNotParseDeviceIdentifier(value: String, cause: ParseIntError)
		{
			display("Could not parse pci_device_identifier as u8 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		#[allow(missing_docs)]
		NoFunction
		{
		}
		
		#[allow(missing_docs)]
		CouldNotParseFunction(value: String, cause: ParseIntError)
		{
			display("Could not parse function as u8 '{}' caused by '{}'", value, cause)
			cause(cause)
		}
		
		#[allow(missing_docs)]
		FunctionExceeds4BitValue(value: u8)
		{
			display("Parsed function exceeds 4-bit value (ie is 16 or more) '{}'", value)
		}
	}
}
