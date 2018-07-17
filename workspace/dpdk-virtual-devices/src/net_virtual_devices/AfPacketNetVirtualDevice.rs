// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux AF_PACKET net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct AfPacketNetVirtualDevice
{
	network_interface_name: NetworkInterfaceName,
	number_of_queue_pairs: u8,
	block_size: u32,
	frame_size: u32,
	frame_count: u32,
}

impl VirtualDevice for AfPacketNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::AfPacket;
	
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
	pub const DefaultNumberOfQueuePairs: u8 = 1;
	
	#[allow(missing_docs)]
	pub const DefaultBlockSize: u32 = 1 << 12;
	
	#[allow(missing_docs)]
	pub const DefaultFrameSize: u32 = 1 << 11;
	
	#[allow(missing_docs)]
	pub const DefaultFrameCount: u32 = 1 << 9;
	
	/// New instance with defaults.
	#[inline(always)]
	pub fn defaultish(network_interface_name: NetworkInterfaceName) -> Self
	{
		Self::slightly_defaultish(network_interface_name, Self::DefaultNumberOfQueuePairs)
	}
	
	/// New instance with most defaults.
	///
	/// `number_of_queue_pairs` is a 4-bit unsigned integer, but can not be zero.
	#[inline(always)]
	pub fn slightly_defaultish(network_interface_name: NetworkInterfaceName, number_of_queue_pairs: u8) -> Self
	{
		Self::new(network_interface_name, number_of_queue_pairs, Self::DefaultBlockSize, Self::DefaultFrameSize, Self::DefaultFrameCount)
	}

	/// New instance.
	///
	/// `number_of_queue_pairs` is a 4-bit unsigned integer, but can not be zero.
	/// `block_size`, `frame_size` and `frame_count` are all 31-bit unsigned integer.
	#[inline(always)]
	pub fn new(network_interface_name: NetworkInterfaceName, number_of_queue_pairs: u8, block_size: u32, frame_size: u32, frame_count: u32) -> Self
	{
		assert_ne!(number_of_queue_pairs, 0, "number_of_queue_pairs can not be zero");
		assert!(number_of_queue_pairs < Self::MaximumNumberOfQueuePairs, "number_of_queue_pairs '{}' equals or exceeds MaximumNumberOfQueuePairs of '{}'", number_of_queue_pairs, Self::MaximumNumberOfQueuePairs);

		Self
		{
			network_interface_name,
			number_of_queue_pairs,
			block_size,
			frame_size,
			frame_count,
		}
	}
}
