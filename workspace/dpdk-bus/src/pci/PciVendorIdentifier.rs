// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// PCI Vendor Identifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciVendorIdentifier(u16);

impl From<u16> for PciVendorIdentifier
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		PciVendorIdentifier(value)
	}
}

impl Into<u16> for PciVendorIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl PciVendorIdentifier
{
	#[allow(missing_docs)]
	pub const Amazon: PciVendorIdentifier = PciVendorIdentifier(0x1D0F);
	
	#[allow(missing_docs)]
	pub const Broadcom: PciVendorIdentifier = PciVendorIdentifier(0x14E4);
	
	#[allow(missing_docs)]
	pub const Chelsio: PciVendorIdentifier = PciVendorIdentifier(0x1425);
	
	#[allow(missing_docs)]
	pub const Cisco: PciVendorIdentifier = PciVendorIdentifier(0x1137);
	
	#[allow(missing_docs)]
	pub const Intel: PciVendorIdentifier = PciVendorIdentifier(0x8086);
	
	#[allow(missing_docs)]
	pub const Mellanox: PciVendorIdentifier = PciVendorIdentifier(0x15b3);
	
	#[allow(missing_docs)]
	pub const NetCope: PciVendorIdentifier = PciVendorIdentifier(0x1b26);
	
	#[allow(missing_docs)]
	pub const Netronome: PciVendorIdentifier = PciVendorIdentifier(0x19ee);
	
	#[allow(missing_docs)]
	pub const QLogic: PciVendorIdentifier = PciVendorIdentifier(0x1077);
	
	#[allow(missing_docs)]
	pub const Qumranet: PciVendorIdentifier = PciVendorIdentifier(0x1AF4);
	
	#[allow(missing_docs)]
	pub const VmWare: PciVendorIdentifier = PciVendorIdentifier(0x15AD);
	
	#[allow(missing_docs)]
	pub const VirtIO: PciVendorIdentifier = PciVendorIdentifier::Qumranet;
	
	const AnyOrInvalidRaw: u16 = 0xFFFF;
	
	/// Any or Invalid PCI identifier.
	pub const AnyOrInvalid: PciVendorIdentifier = PciVendorIdentifier(Self::AnyOrInvalidRaw);
	
	/// New instance.
	///
	/// Returns None if the `pci_vendor_identifier` is `0xFFFF`.
	#[inline(always)]
	pub fn new(pci_vendor_identifier: u16) -> Option<Self>
	{
		if pci_vendor_identifier == Self::AnyOrInvalidRaw
		{
			None
		}
		else
		{
			Some(PciVendorIdentifier(pci_vendor_identifier))
		}
	}
	
	/// Is this identifier any or invalid?
	#[inline(always)]
	pub fn is_any_or_invalid(&self) -> bool
	{
		self.0 == Self::AnyOrInvalidRaw
	}
	
	/// Does this identifier match?
	#[inline(always)]
	pub fn is(&self, other: u16) -> bool
	{
		self.0 == other
	}
}
