// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtIoForContainersNetVirtualDevice
{
	index: u5,
	locationOfVirtIoDeviceFile: String,
	mediaAccessControlAddress: MediaAccessControlAddress,
	controlQueueIndex: u31,
	queueSize: u31,
	maximumNumberOfQueuePairs: u31,
}

impl VirtualDevice for VirtIoForContainersNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::VirtIoUser;
	
	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String
	{
		format!(",path={},mac={},cq={},queue_size={},queues={}", self.locationOfVirtIoDeviceFile, self.mediaAccessControlAddress, self.controlQueueIndex, self.queueSize, self.maximumNumberOfQueuePairs)
	}
}

impl NetVirtualDevice for VirtIoForContainersNetVirtualDevice
{
}

impl VirtIoForContainersNetVirtualDevice
{
	pub const DefaultControlQueueIndex: u31 = 0;
	pub const DefaultQueueSize: u31 = 256;
	pub const DefaultMaximumNumberOfQueuePairs: u31 = 1;
	
	pub fn defaultish(index: u5, locationOfVirtIoDeviceFile: &Path, mediaAccessControlAddress: MediaAccessControlAddress) -> Self
	{
		Self::new(index, locationOfVirtIoDeviceFile, mediaAccessControlAddress, Self::DefaultControlQueueIndex, Self::DefaultQueueSize, Self::DefaultMaximumNumberOfQueuePairs)
	}
	
	pub fn new(index: u5, locationOfVirtIoDeviceFile: &Path, mediaAccessControlAddress: MediaAccessControlAddress, controlQueueIndex: u31, queueSize: u31, maximumNumberOfQueuePairs: u31) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);
		assert!(maximumNumberOfQueuePairs != 0, "maximumNumberOfQueuePairs can not be zero");
		assert!(locationOfVirtIoDeviceFile.exists(), "path does not exist");
		
		let locationOfVirtIoDeviceFile = locationOfVirtIoDeviceFile.to_str().expect("path is not a valid UTF-8 string").to_owned();
		
		if maximumNumberOfQueuePairs > 1 && controlQueueIndex == 0
		{
			assert!(false, "If there are multiple queues '{}' then the control queue index can not be zero", maximumNumberOfQueuePairs);
		}
		
		VirtIoForContainersNetVirtualDevice
		{
			index: index,
			locationOfVirtIoDeviceFile: locationOfVirtIoDeviceFile,
			mediaAccessControlAddress: mediaAccessControlAddress,
			controlQueueIndex: controlQueueIndex,
			queueSize: queueSize,
			maximumNumberOfQueuePairs: maximumNumberOfQueuePairs,
		}
	}
}
