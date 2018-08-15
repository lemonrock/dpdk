// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux TUN net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TunNetVirtualDevice
{
	/// Use a non-default network interface name.
	#[serde(default)]
	pub network_interface_name: Option<NetworkInterfaceName>,
}

impl VirtualDevice for TunNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Tun;
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		if let Some(ref network_interface_name) = self.network_interface_name
		{
			format!(",iface={}", network_interface_name.text())
		}
		else
		{
			"".to_string()
		}
	}
}

impl NetVirtualDevice for TunNetVirtualDevice
{
}
