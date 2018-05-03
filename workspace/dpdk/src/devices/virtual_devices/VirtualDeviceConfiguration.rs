// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct VirtualDeviceConfiguration<V: VirtualDevice, C>
{
	pub(crate) virtual_device: V,
	pub(crate) configuration: C,
}

impl<V: VirtualDevice> VirtualDeviceConfiguration<V, EthernetPortConfiguration>
{
	pub fn device_name(&self) -> VirtualDeviceName<<V as ::devices::virtual_devices::VirtualDevice>::V>
	{
		self.virtual_device.name()
	}
}