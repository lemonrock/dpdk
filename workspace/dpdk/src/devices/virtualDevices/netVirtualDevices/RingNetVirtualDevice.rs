// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct RingNetVirtualDevice
{
	index: u5,
	nodeActions: HashMap<String, (NumaSocketId, RingNodeAction)>,
}

impl VirtualDevice for RingNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Ring;
	
	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String
	{
		let mut result = String::with_capacity(128);
		for (ringName, &(numaSocketId, ringNodeAction)) in &self.nodeActions
		{
			result.push_str(&format!(",{}:{}:{}", ringName, numaSocketId.as_u8(), ringNodeAction.asDpdkString()));
		}
		result
	}
}

impl NetVirtualDevice for RingNetVirtualDevice
{
}

impl RingNetVirtualDevice
{
	#[inline(always)]
	pub fn new(index: u5, nodeActions: HashMap<String, (NumaSocketId, RingNodeAction)>) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);
		
		RingNetVirtualDevice
		{
			index: index,
			nodeActions: nodeActions,
		}
	}
}
