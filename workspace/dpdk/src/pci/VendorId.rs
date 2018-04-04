// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VendorId(pub u16);

impl VendorId
{
	pub const Amazon: VendorId = VendorId(0x1D0F);
	pub const Broadcom: VendorId = VendorId(0x14E4);
	pub const Chelsio: VendorId = VendorId(0x1425);
	pub const Cisco: VendorId = VendorId(0x1137);
	pub const Intel: VendorId = VendorId(0x8086);
	pub const Mellanox: VendorId = VendorId(0x15b3);
	pub const NetCope: VendorId = VendorId(0x1b26);
	pub const Netronome: VendorId = VendorId(0x19ee);
	pub const QLogic: VendorId = VendorId(0x1077);
	pub const Qumranet: VendorId = VendorId(0x1AF4);
	pub const VmWare: VendorId = VendorId(0x15AD);
	
	pub const VirtIO: VendorId = VendorId::Qumranet;
	
	const AnyOrInvalidRaw: u16 = 0xFFFF;
	
	#[inline(always)]
	pub fn new(vendorId: u16) -> Option<Self>
	{
		if vendorId == Self::AnyOrInvalidRaw
		{
			None
		}
		else
		{
			Some(VendorId(vendorId))
		}
	}
	
	pub const AnyOrInvalid: VendorId = VendorId(Self::AnyOrInvalidRaw);
	
	#[inline(always)]
	pub fn isAnyOrInvalid(&self) -> bool
	{
		self.0 == Self::AnyOrInvalidRaw
	}
	
	#[inline(always)]
	pub fn is(&self, other: u16) -> bool
	{
		self.0 == other
	}
}
