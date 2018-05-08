// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtIoNetVirtualDevice
{
	locationOfVirtIoDeviceFile: String,
	media_access_control_address: MediaAccessControlAddress,
	controlQueueIndex: u31,
	queueSize: u31,
	maximumNumberOfQueuePairs: u31,
}

impl VirtualDevice for VirtIoNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::VirtIoUser;

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		format!(",path={},mac={},cq={},queue_size={},queues={}", self.locationOfVirtIoDeviceFile, self.media_access_control_address, self.controlQueueIndex, self.queueSize, self.maximumNumberOfQueuePairs)
	}
}

impl NetVirtualDevice for VirtIoNetVirtualDevice
{
}

impl VirtIoNetVirtualDevice
{
	pub const DefaultControlQueueIndex: u31 = 0;
	pub const DefaultQueueSize: u31 = 256;
	pub const DefaultMaximumNumberOfQueuePairs: u31 = 1;

	pub fn defaultish(locationOfVirtIoDeviceFile: &Path, media_access_control_address: MediaAccessControlAddress) -> Self
	{
		Self::new(locationOfVirtIoDeviceFile, media_access_control_address, Self::DefaultControlQueueIndex, Self::DefaultQueueSize, Self::DefaultMaximumNumberOfQueuePairs)
	}

	pub fn new(locationOfVirtIoDeviceFile: &Path, media_access_control_address: MediaAccessControlAddress, controlQueueIndex: u31, queueSize: u31, maximumNumberOfQueuePairs: u31) -> Self
	{
		assert_ne!(maximumNumberOfQueuePairs, 0, "maximumNumberOfQueuePairs can not be zero");
		assert!(locationOfVirtIoDeviceFile.exists(), "path does not exist");

		let locationOfVirtIoDeviceFile = locationOfVirtIoDeviceFile.to_str().expect("path is not a valid UTF-8 string").to_owned();

		if maximumNumberOfQueuePairs > 1 && controlQueueIndex == 0
		{
			assert!(false, "If there are multiple queues '{}' then the control queue index can not be zero", maximumNumberOfQueuePairs);
		}

		VirtIoNetVirtualDevice
		{
			locationOfVirtIoDeviceFile,
			media_access_control_address,
			controlQueueIndex,
			queueSize,
			maximumNumberOfQueuePairs,
		}
	}
}
