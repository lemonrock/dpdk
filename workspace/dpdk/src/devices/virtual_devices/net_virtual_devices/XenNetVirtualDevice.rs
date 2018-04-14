// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct XenNetVirtualDevice
{
	index: u5,
	media_access_control_address: MediaAccessControlAddress,
}

impl VirtualDevice for XenNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Xen;

	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		format!(",mac={}", self.media_access_control_address)
	}
}

impl NetVirtualDevice for XenNetVirtualDevice
{
}

impl XenNetVirtualDevice
{
	pub fn new(index: u5, media_access_control_address: MediaAccessControlAddress) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);

		XenNetVirtualDevice
		{
			index,
			media_access_control_address,
		}
	}
}
