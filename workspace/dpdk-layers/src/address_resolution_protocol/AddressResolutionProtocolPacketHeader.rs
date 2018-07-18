// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct AddressResolutionProtocolPacketHeader
{
	#[allow(missing_docs)]
	pub hardware_type: HardwareType,
	
	#[allow(missing_docs)]
	pub protocol_type: EtherType,
	
	#[allow(missing_docs)]
	pub hardware_address_length: u8,
	
	#[allow(missing_docs)]
	pub protocol_address_length: u8,
	
	#[allow(missing_docs)]
	pub operation: Operation,
}

impl AddressResolutionProtocolPacketHeader
{
	pub(crate) const HeaderSizeU16: u16 = size_of::<AddressResolutionProtocolPacketHeader>() as u16;
	
	/// Use this to eliminate unwanted or invalid ARP traffic.
	#[inline(always)]
	pub(crate) fn is_header_invalid_for_internet_protocol_version_4(&self) -> bool
	{
		self.is_hardware_type_not_ethernet2() || self.is_protocol_type_not_internet_protocol_version_4() || self.is_hardware_address_length_not_valid_for_internet_protocol_version_4() || self.is_protocol_address_length_not_valid_for_internet_protocol_version_4()
	}
	
	#[inline(always)]
	fn is_hardware_type_not_ethernet2(&self) -> bool
	{
		self.hardware_type.is_not_ethernet2()
	}
	
	#[inline(always)]
	fn is_protocol_type_not_internet_protocol_version_4(&self) -> bool
	{
		self.protocol_type.is_not_internet_protocol_version_4()
	}
	
	#[inline(always)]
	fn is_hardware_address_length_not_valid_for_internet_protocol_version_4(&self) -> bool
	{
		self.hardware_address_length != MediaAccessControlAddress::SizeU8
	}
	
	#[inline(always)]
	fn is_protocol_address_length_not_valid_for_internet_protocol_version_4(&self) -> bool
	{
		self.protocol_address_length != InternetProtocolVersion4HostAddress::SizeU8
	}
}
