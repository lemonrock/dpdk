// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A 'VirtIO' net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtIoNetVirtualDevice
{
	/// Location of VirtIO device file.
	pub location_of_virt_io_device_file: PathBuf,
	
	/// Media access control (MAC) address.
	pub media_access_control_address: MediaAccessControlAddress,
	
	/// Control queue index.
	///
	/// Defaults to zero (0).
	#[serde(default = "VirtIoNetVirtualDevice::control_queue_index_default")]
	pub control_queue_index: u32,
	
	/// Queue size.
	///
	/// Defaults to 256.
	///
	/// Can not be zero.
	#[serde(default = "VirtIoNetVirtualDevice::queue_size_default")]
	queue_size: u32,
	
	/// Maximum number of receive-transmit queue pairs.
	///
	/// Defaults to 1.
	///
	/// Can not be zero.
	#[serde(default = "VirtIoNetVirtualDevice::maximum_number_of_queue_pairs_default")]
	maximum_number_of_queue_pairs: u32,
}

impl VirtualDevice for VirtIoNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::VirtIoUser;

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		let queue_size = cap_u32_to_u31(min(1, self.queue_size));
		let maximum_number_of_queue_pairs = cap_u32_to_u31(min(1, self.maximum_number_of_queue_pairs));
		let control_queue_index = cap_u32_to_u31(self.control_queue_index);
		
		if maximum_number_of_queue_pairs > 1 && control_queue_index == 0
		{
			assert!(false, "If there are multiple queues '{}' then the control queue index can not be zero", maximum_number_of_queue_pairs);
		}
		
		format!(",path={},mac={},cq={},queue_size={},queues={}", self.location_of_virt_io_device_file.to_str().unwrap(), self.media_access_control_address, control_queue_index, queue_size, maximum_number_of_queue_pairs)
	}
}

impl NetVirtualDevice for VirtIoNetVirtualDevice
{
}

impl VirtIoNetVirtualDevice
{
	#[inline(always)]
	const fn control_queue_index_default() -> u32
	{
		0
	}
	
	#[inline(always)]
	const fn queue_size_default() -> u32
	{
		256
	}
	
	#[inline(always)]
	const fn maximum_number_of_queue_pairs_default() -> u32
	{
		1
	}
}
