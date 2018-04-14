// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// PCI device identifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciDeviceIdentifier(u16);

impl PciDeviceIdentifier
{
	const AnyOrInvalidRaw: u16 = 0xFFFF;
	
	/// Any or invalid.
	pub const AnyOrInvalid: PciDeviceIdentifier = PciDeviceIdentifier(Self::AnyOrInvalidRaw);
	
	/// New.
	#[inline(always)]
	pub fn new(pci_device_identifier: u16) -> Option<Self>
	{
		if pci_device_identifier == Self::AnyOrInvalidRaw
		{
			None
		}
		else
		{
			Some(PciDeviceIdentifier(pci_device_identifier))
		}
	}
	
	/// Is this any or invalid?
	#[inline(always)]
	pub fn is_any_or_invalid(&self) -> bool
	{
		self.0 == Self::AnyOrInvalidRaw
	}
	
	// Is?
	#[inline(always)]
	pub fn is(&self, other: u16) -> bool
	{
		self.0 == other
	}
}
