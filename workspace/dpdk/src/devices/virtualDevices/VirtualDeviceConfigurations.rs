// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct VirtualDeviceConfigurations<V: VirtualDevice, C>
{
	map: HashMap<u8, VirtualDeviceConfiguration<V, C>>,
}

impl<V: VirtualDevice, C> VirtualDeviceConfigurations<V, C>
{
	pub fn empty() -> Self
	{
		Self::with_capacity(0)
	}
	
	pub fn with_capacity(capacity: usize) -> Self
	{
		VirtualDeviceConfigurations
		{
			map: HashMap::with_capacity(capacity),
		}
	}
	
	pub fn createConfiguration(&mut self, virtualDevice: V, configuration: C)
	{
		let index = virtualDevice.index();
		let data = VirtualDeviceConfiguration
		{
			virtualDevice: virtualDevice,
			configuration: configuration,
		};
		
		self.map.insert(index, data).expect("Already created a configuration");
	}
	
	pub fn addVirtualDevicesSorted(&self, arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__vdev = "--vdev";
		}
			
		let mut virtualDevices: Vec<&V> = self.map.values().map(|value| &value.virtualDevice).collect();
		virtualDevices.sort_by_key(|value| { value.index() });
		for virtualDevice in virtualDevices
		{
			let argument = virtualDevice.asInitialisationArgument();
			arguments.keyCStrValue(__vdev, &argument);
		}
	}
}
