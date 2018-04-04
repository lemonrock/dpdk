// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct AfPacketNetVirtualDevice
{
	index: u5,
	interfaceName: NetworkInterfaceName,
	numberOfQueuePairs: u4,
	blockSize: u31,
	frameSize: u31,
	frameCount: u31,
}

impl VirtualDevice for AfPacketNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::AfPacket;
	
	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String
	{
		format!(",iface={},qpairs={},blocksz={},framesz={},framecnt={}", self.interfaceName.text(), self.numberOfQueuePairs, self.blockSize, self.frameSize, self.frameCount)
	}
}

impl NetVirtualDevice for AfPacketNetVirtualDevice
{
}

impl AfPacketNetVirtualDevice
{
	pub const MaximumNumberOfQueuePairs: u8 = 16;
	
	pub const DefaultNumberOfQueuePairs: u4 = 1;
	pub const DefaultBlockSize: u31 = 1 << 12;
	pub const DefaultFrameSize: u31 = 1 << 11;
	pub const DefaultFrameCount: u31 = 1 << 9;
	
	#[inline(always)]
	pub fn defaultish(index: u5, interfaceName: NetworkInterfaceName) -> Self
	{
		Self::slightlyDefaultish(index, interfaceName, Self::DefaultNumberOfQueuePairs)
	}
	
	#[inline(always)]
	pub fn slightlyDefaultish(index: u5, interfaceName: NetworkInterfaceName, numberOfQueuePairs: u4) -> Self
	{
		Self::new(index, interfaceName, numberOfQueuePairs, Self::DefaultBlockSize, Self::DefaultFrameSize, Self::DefaultFrameCount)
	}
	
	#[inline(always)]
	pub fn new(index: u5, interfaceName: NetworkInterfaceName, numberOfQueuePairs: u4, blockSize: u31, frameSize: u31, frameCount: u31) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);
		assert!(numberOfQueuePairs != 0, "numberOfQueuePairs can not be zero");
		assert!(numberOfQueuePairs < Self::MaximumNumberOfQueuePairs, "numberOfQueuePairs '{}' equals or exceeds MaximumNumberOfQueuePairs of '{}'", numberOfQueuePairs, Self::MaximumNumberOfQueuePairs);
		
		AfPacketNetVirtualDevice
		{
			index: index,
			interfaceName: interfaceName,
			numberOfQueuePairs: numberOfQueuePairs,
			blockSize: blockSize,
			frameSize: frameSize,
			frameCount: frameCount,
		}
	}
}
