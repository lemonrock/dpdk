// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtualDeviceName<V: DeviceDriverName>
{
	virtualDeviceDriverName: V,
	index: u5,
}

impl<V: DeviceDriverName> DeviceName for VirtualDeviceName<V>
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		format!("{}{}", self.virtualDeviceDriverName.value().to_owned(), self.indexToBase32LowerCase())
	}
}

impl<V: DeviceDriverName> VirtualDeviceName<V>
{
	// Maximum number of ethernet ports dictates this value
	pub const MaximumIndex: u8 = 32;
	
	#[inline(always)]
	pub fn new(virtualDeviceDriverName: V, index: u5) -> Self
	{
		assert!(index < Self::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, Self::MaximumIndex);
		
		VirtualDeviceName
		{
			virtualDeviceDriverName: virtualDeviceDriverName,
			index: index,
		}
	}
	
	#[inline(always)]
	pub fn isNotBackedByDriverName(&self, virtualDeviceDriverName: V) -> bool
	{
		self.virtualDeviceDriverName != virtualDeviceDriverName
	}
	
	#[inline(always)]
	fn indexToBase32LowerCase(&self) -> char
	{
		let index = self.index;
		match index
		{
			0...9 => (48 + index) as char,
			10...31 => (97 + index - 10) as char,
		
			_ => panic!("index can not be {} or greater, but it was: '{}", Self::MaximumIndex, index),
		}
	}
}
