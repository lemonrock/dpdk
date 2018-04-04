// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct NullNetVirtualDevice
{
	index: u5,
	packetSize: u31,
	shouldCopyPackets: bool,
}

impl VirtualDevice for NullNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Null;
	
	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String
	{
		let copy = if self.shouldCopyPackets
		{
			1
		}
		else
		{
			0
		};
		
		format!(",size={},copy={}", self.packetSize, copy)
	}
}

impl NetVirtualDevice for NullNetVirtualDevice
{
}

impl NullNetVirtualDevice
{
	pub const DefaultPacketSize: u31 = 64;
	pub const DefaultShouldNotCopyPackets: bool = false;

	#[inline(always)]
	pub fn defaultish(index: u5) -> Self
	{
		NullNetVirtualDevice::new(index, Self::DefaultPacketSize, Self::DefaultShouldNotCopyPackets)
	}
	
	pub fn new(index: u5, packetSize: u32, shouldCopyPackets: bool) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);
		
		NullNetVirtualDevice
		{
			index: index,
			packetSize: packetSize,
			shouldCopyPackets: shouldCopyPackets,
		}
	}
}
