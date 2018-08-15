// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux AF_PACKET net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct AfPacketNetVirtualDevice
{
	/// Network interface name.
	pub network_interface_name: NetworkInterfaceName,
	
	/// Number of queue pairs.
	///
	/// Maximum is 16; default is 1.
	///
	/// Can not be zero (0).
	#[serde(default = "AfPacketNetVirtualDevice::number_of_queue_pairs_default")]
	pub number_of_queue_pairs: u8,
	
	/// Block size in bytes.
	///
	/// Should be a power of 2.
	///
	/// Default is 1 << 12.
	#[serde(default = "AfPacketNetVirtualDevice::block_size_default")]
	pub block_size: u32,
	
	/// Frame size.
	///
	/// Should be a power of 2.
	///
	/// Default is 1 << 11.
	#[serde(default = "AfPacketNetVirtualDevice::frame_size_default")]
	pub frame_size: u32,
	
	/// Frame count.
	///
	/// Should be a power of 2.
	///
	/// Default is 1 << 9.
	#[serde(default = "AfPacketNetVirtualDevice::frame_count_default")]
	pub frame_count: u32,
}

impl VirtualDevice for AfPacketNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::AfPacket;
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		let number_of_queue_pairs = min(Self::MaximumNumberOfQueuePairs, max(self.number_of_queue_pairs, 1));
		let block_size = cap_u32_to_u31_maximum_power_of_two(self.block_size);
		let frame_size = cap_u32_to_u31_maximum_power_of_two(self.frame_size);
		let frame_count = cap_u32_to_u31_maximum_power_of_two(self.frame_count);
		
		format!(",iface={},qpairs={},blocksz={},framesz={},framecnt={}", self.network_interface_name.text(), number_of_queue_pairs, block_size, frame_size, frame_count)
	}
}

impl NetVirtualDevice for AfPacketNetVirtualDevice
{
}

impl AfPacketNetVirtualDevice
{
	const MaximumNumberOfQueuePairs: u8 = 16;
	
	#[inline(always)]
	const fn number_of_queue_pairs_default() -> u8
	{
		1
	}
	
	#[inline(always)]
	const fn block_size_default() -> u32
	{
		1 << 12
	}
	
	#[inline(always)]
	const fn frame_size_default() -> u32
	{
		1 << 11
	}
	
	#[inline(always)]
	const fn frame_count_default() -> u32
	{
		1 << 9
	}
}
