// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct VirtualDeviceConfigurations<V: VirtualDevice, C>
{
	map: HashMap<u8, VirtualDeviceConfiguration<V, C>>,
}

impl<V: VirtualDevice, C> Default for VirtualDeviceConfigurations<V, C>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::with_capacity(0)
	}
}

impl<V: VirtualDevice, C> VirtualDeviceConfigurations<V, C>
{
	#[inline(always)]
	pub(crate) fn with_capacity(capacity: usize) -> Self
	{
		VirtualDeviceConfigurations
		{
			map: HashMap::with_capacity(capacity),
		}
	}
	
	pub(crate) fn create_configuration(&mut self, virtual_device: V, configuration: C)
	{
		let index = virtual_device.index();
		let data = VirtualDeviceConfiguration
		{
			virtual_device,
			configuration,
		};

		self.map.insert(index, data).expect("Already created a configuration");
	}
	
	pub(crate) fn add_virtual_devices_sorted(&self, arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__vdev = "--vdev";
		}

		let mut virtual_devices: Vec<&V> = self.map.values().map(|value| &value.virtual_device).collect();
		virtual_devices.sort_by_key(|value| { value.index() });
		for virtual_device in virtual_devices
		{
			let argument = virtual_device.as_initialisation_argument();
			arguments.keyCStrValue(__vdev, &argument);
		}
	}
}
