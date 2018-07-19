// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressResolutionProtocolPacketHeader
{
	/// Hardware type.
	///
	/// Should be Ethernet 2 if received over ethernet.
	pub hardware_type: HardwareType,
	
	/// Protocol type.
	///
	/// Only Internet Protocol Version 4 is in common use in 2018.
	#[allow(missing_docs)]
	pub protocol_type: EtherType,
	
	/// Length of hardware addresses.
	///
	/// Should be 6 for `hardware_type` of Ethernet 2.
	#[allow(missing_docs)]
	pub hardware_address_length: u8,
	
	/// Length of protocol addresses.
	///
	/// Should be 4 for `protocol_type` of Internet Protocol Version 4.
	#[allow(missing_docs)]
	pub protocol_address_length: u8,
	
	/// Operation.
	///
	/// Only Request and Reply are in common use in 2018.
	#[allow(missing_docs)]
	pub operation: Operation,
}

impl Display for AddressResolutionProtocolPacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl AddressResolutionProtocolPacketHeader
{
	pub(crate) const HeaderSizeU16: u16 = size_of::<AddressResolutionProtocolPacketHeader>() as u16;
	
	#[inline(always)]
	pub(crate) fn is_header_invalid_for_internet_protocol_version_4(&self) -> bool
	{
		self.is_hardware_type_not_ethernet_2() || self.is_protocol_type_not_internet_protocol_version_4() || self.is_hardware_address_length_not_valid_for_ethernet_2() || self.is_protocol_address_length_not_valid_for_internet_protocol_version_4()
	}
	
	#[inline(always)]
	fn is_hardware_type_not_ethernet_2(&self) -> bool
	{
		self.hardware_type.is_not_ethernet_2()
	}
	
	#[inline(always)]
	fn is_protocol_type_not_internet_protocol_version_4(&self) -> bool
	{
		self.protocol_type.is_not_internet_protocol_version_4()
	}
	
	#[inline(always)]
	fn is_hardware_address_length_not_valid_for_ethernet_2(&self) -> bool
	{
		self.hardware_address_length != MediaAccessControlAddress::SizeU8
	}
	
	#[inline(always)]
	fn is_protocol_address_length_not_valid_for_internet_protocol_version_4(&self) -> bool
	{
		self.protocol_address_length != InternetProtocolVersion4HostAddress::SizeU8
	}
}
