// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux AF_PACKET net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct AfPacketNetVirtualDevice
{
	index: u5,
	network_interface_name: NetworkInterfaceName,
	number_of_queue_pairs: u4,
	block_size: u31,
	frame_size: u31,
	frame_count: u31,
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
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		format!(",iface={},qpairs={},blocksz={},framesz={},framecnt={}", self.network_interface_name.text(), self.number_of_queue_pairs, self.block_size, self.frame_size, self.frame_count)
	}
}

impl NetVirtualDevice for AfPacketNetVirtualDevice
{
}

impl AfPacketNetVirtualDevice
{
	#[allow(missing_docs)]
	pub const MaximumNumberOfQueuePairs: u8 = 16;
	
	#[allow(missing_docs)]
	pub const DefaultNumberOfQueuePairs: u4 = 1;
	
	#[allow(missing_docs)]
	pub const DefaultBlockSize: u31 = 1 << 12;
	
	#[allow(missing_docs)]
	pub const DefaultFrameSize: u31 = 1 << 11;
	
	#[allow(missing_docs)]
	pub const DefaultFrameCount: u31 = 1 << 9;
	
	/// New instance with defaults.
	#[inline(always)]
	pub fn defaultish(index: u5, network_interface_name: NetworkInterfaceName) -> Self
	{
		Self::slightly_defaultish(index, network_interface_name, Self::DefaultNumberOfQueuePairs)
	}
	
	/// New instance with most defaults.
	#[inline(always)]
	pub fn slightly_defaultish(index: u5, network_interface_name: NetworkInterfaceName, number_of_queue_pairs: u4) -> Self
	{
		Self::new(index, network_interface_name, number_of_queue_pairs, Self::DefaultBlockSize, Self::DefaultFrameSize, Self::DefaultFrameCount)
	}

	/// New instance.
	#[inline(always)]
	pub fn new(index: u5, network_interface_name: NetworkInterfaceName, number_of_queue_pairs: u4, block_size: u31, frame_size: u31, frame_count: u31) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);
		assert_ne!(number_of_queue_pairs, 0, "number_of_queue_pairs can not be zero");
		assert!(number_of_queue_pairs < Self::MaximumNumberOfQueuePairs, "number_of_queue_pairs '{}' equals or exceeds MaximumNumberOfQueuePairs of '{}'", number_of_queue_pairs, Self::MaximumNumberOfQueuePairs);

		Self
		{
			index,
			network_interface_name,
			number_of_queue_pairs,
			block_size,
			frame_size,
			frame_count,
		}
	}
}
