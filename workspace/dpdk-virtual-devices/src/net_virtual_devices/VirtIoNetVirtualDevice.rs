// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A 'virtio' net virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtIoNetVirtualDevice
{
	location_of_virt_io_device_file: String,
	media_access_control_address: MediaAccessControlAddress,
	control_queue_index: u32,
	queue_size: u32,
	maximum_number_of_queue_pairs: u32,
}

impl VirtualDevice for VirtIoNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::VirtIoUser;

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		format!(",path={},mac={},cq={},queue_size={},queues={}", self.location_of_virt_io_device_file, self.media_access_control_address, self.control_queue_index, self.queue_size, self.maximum_number_of_queue_pairs)
	}
}

impl NetVirtualDevice for VirtIoNetVirtualDevice
{
}

impl VirtIoNetVirtualDevice
{
	#[allow(missing_docs)]
	pub const DefaultControlQueueIndex: u32 = 0;
	
	#[allow(missing_docs)]
	pub const DefaultQueueSize: u32 = 256;
	
	#[allow(missing_docs)]
	pub const DefaultMaximumNumberOfQueuePairs: u32 = 1;

	/// Defaul-like new instance.
	pub fn defaultish(location_of_virt_io_device_file: &Path, media_access_control_address: MediaAccessControlAddress) -> Self
	{
		Self::new(location_of_virt_io_device_file, media_access_control_address, Self::DefaultControlQueueIndex, Self::DefaultQueueSize, Self::DefaultMaximumNumberOfQueuePairs)
	}

	/// Creates a new insance.
	///
	/// `control_queue_index`, `queue_size`, and `maximum_number_of_queue_pairs` are 31-bit unsigned integers.
	/// `maximum_number_of_queue_pairs` can not be zero.
	pub fn new(location_of_virt_io_device_file: &Path, media_access_control_address: MediaAccessControlAddress, control_queue_index: u32, queue_size: u32, maximum_number_of_queue_pairs: u32) -> Self
	{
		assert_ne!(maximum_number_of_queue_pairs, 0, "maximum_number_of_queue_pairs can not be zero");
		assert!(location_of_virt_io_device_file.exists(), "path does not exist");

		let location_of_virt_io_device_file = location_of_virt_io_device_file.to_str().expect("path is not a valid UTF-8 string").to_owned();

		if maximum_number_of_queue_pairs > 1 && control_queue_index == 0
		{
			assert!(false, "If there are multiple queues '{}' then the control queue index can not be zero", maximum_number_of_queue_pairs);
		}

		VirtIoNetVirtualDevice
		{
			location_of_virt_io_device_file,
			media_access_control_address,
			control_queue_index,
			queue_size,
			maximum_number_of_queue_pairs,
		}
	}
}
