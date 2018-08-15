// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A virtual host virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtualHostNetVirtualDevice
{
	/// A vhost interface file.
	pub location_of_interface_file: PathBuf,
	
	/// Number of queues.
	///
	/// Can not be zero (0).
	///
	/// Defaults to one (1).
	#[serde(default = "VirtualHostNetVirtualDevice::number_of_queues_default")]
	pub number_of_queues: u8,
}

impl VirtualDevice for VirtualHostNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::VirtIoUser;

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		format!(",iface={},queues={}", self.location_of_interface_file.to_str().unwrap(), min(1, self.number_of_queues))
	}
}

impl NetVirtualDevice for VirtualHostNetVirtualDevice
{
}

impl VirtualHostNetVirtualDevice
{
	#[inline(always)]
	const fn number_of_queues_default() -> u8
	{
		1
	}
}
