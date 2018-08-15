// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux Kernel Native Interface (KNI) net(work) virtual device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct KernelNativeInterfaceNetVirtualDevice
{
	/// No request thread.
	#[serde(default)]
	pub no_request_thread: bool,
}

impl VirtualDevice for KernelNativeInterfaceNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::KernelNativeInterface;
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		if self.no_request_thread
		{
			format!(",no_request_thread={}", 1)
		}
		else
		{
			String::new()
		}
	}
}

impl NetVirtualDevice for KernelNativeInterfaceNetVirtualDevice
{
}
