// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux TAP net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TapNetVirtualDevice
{
	network_interface_name: Option<NetworkInterfaceName>,
	
	media_access_control_address: TapMediaAccessControlAddress,

	remote: Option<NetworkInterfaceName>,
}

impl VirtualDevice for TapNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Tap;
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		let iface = if let Some(ref network_interface_name) = self.network_interface_name
		{
			format!(",iface={}", network_interface_name.text())
		}
		else
		{
			"".to_string()
		};
		
		let mac = self.media_access_control_address.to_string();
		
		let remote = if let Some(ref remote) = self.remote
		{
			format!(",remote={}", remote.text())
		}
		else
		{
			"".to_string()
		};
		
		format!("{}{}{}", iface, mac, remote)
	}
}

impl NetVirtualDevice for TapNetVirtualDevice
{
}
