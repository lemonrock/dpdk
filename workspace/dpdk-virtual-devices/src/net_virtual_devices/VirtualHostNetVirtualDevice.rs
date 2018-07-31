// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A virtual host virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtualHostNetVirtualDevice
{
	interface: String,
	queues: u8,
}

impl VirtualDevice for VirtualHostNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::VirtIoUser;

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		format!(",iface={},queues={}", self.interface, self.queues)
	}
}

impl NetVirtualDevice for VirtualHostNetVirtualDevice
{
}

impl VirtualHostNetVirtualDevice
{
	/// Creates a new instance.
	///
	/// `queues` can not be zero.
	pub fn new(interface: &Path, queues: u8) -> Self
	{
		assert!(interface.exists(), "interface '{:?}' does not exist", interface);
		assert_ne!(queues, 0, "queues can not be zero");

		VirtualHostNetVirtualDevice
		{
			interface: interface.to_str().expect("interface is not a valid UTF-8 string").to_owned(),
			queues,
		}
	}
}
