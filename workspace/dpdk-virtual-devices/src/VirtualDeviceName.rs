// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Name of a virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtualDeviceName<V: DeviceDriverName>
{
	virtual_device_driver_name: V,
	index: VirtualDeviceIndex,
}

impl<V: DeviceDriverName> DeviceName for VirtualDeviceName<V>
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		format!("{}{}", self.virtual_device_driver_name.value().to_owned(), self.index_to_base32_lower_case())
	}
}

impl<V: DeviceDriverName> VirtualDeviceName<V>
{
	/// New instance.
	///
	/// `index` is a 5-bit unsigned integer.
	#[inline(always)]
	pub fn new(virtual_device_driver_name: V, index: VirtualDeviceIndex) -> Self
	{
		VirtualDeviceName
		{
			virtual_device_driver_name,
			index,
		}
	}

	#[inline(always)]
	pub(crate) fn is_not_backed_by_driver_name(&self, virtual_device_driver_name: V) -> bool
	{
		self.virtual_device_driver_name != virtual_device_driver_name
	}
	
	#[inline(always)]
	fn index_to_base32_lower_case(&self) -> char
	{
		let index = self.index.into();
		match index
		{
			0...9 => (48u8 + index) as char,
			10...31 => (97u8 + index - 10) as char,

			_ => unreachable!(),
		}
	}
}
