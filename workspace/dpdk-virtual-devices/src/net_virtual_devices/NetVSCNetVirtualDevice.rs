// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Microsoft Hyper-V NetVSC net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct NetVSCNetVirtualDevice
{
	/// Network interface names,
	pub network_interface_names: HashSet<NetworkInterfaceName>,
	
	/// Media access control (MAC) address.
	pub media_access_control_addresses: HashSet<MediaAccessControlAddress>,
	
	/// Forces the use of specified interfaces even if not detected as NetVSC.
	///
	/// Defaults to false.
	#[serde(default)]
	pub force: bool,
	
	/// Ignores the driver running (actually used to disable the auto-detection in Hyper-V Virtual Machine).
	///
	/// Defaults to false.
	#[serde(default)]
	pub ignore: bool,
}

impl VirtualDevice for NetVSCNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::NetVSC;
	
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		let mut string = String::new();
		
		for network_interface_name in self.network_interface_names.iter()
		{
			string.push_str(",iface=");
			string.push_str(&network_interface_name.text());
		}
		
		for media_access_control_address in self.media_access_control_addresses.iter()
		{
			string.push_str(",mac=");
			string.push_str(&format!("{}", media_access_control_address));
		}
		
		if self.force
		{
			string.push_str(",force=1");
		}
		
		if self.ignore
		{
			string.push_str(",ignore=1");
		}
		
		string
	}
}

impl NetVirtualDevice for NetVSCNetVirtualDevice
{
}
