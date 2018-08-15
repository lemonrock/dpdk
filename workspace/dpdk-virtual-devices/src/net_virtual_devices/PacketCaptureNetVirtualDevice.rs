// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet capture (pcap) net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PacketCaptureNetVirtualDevice
{
	/// Receive.
	pub receive: PacketCaptureFileOrInterface,
	
	/// Transmit.
	pub transmit: PacketCaptureFileOrInterface,
}

impl VirtualDevice for PacketCaptureNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::PacketCapture;
	
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		if self.receive.is_interface() && self.transmit.is_interface() && self.receive == self.transmit
		{
			format!(",{}", self.receive.format(PacketCaptureFileOrInterfacePrefix::Both))
		}
		else
		{
			format!(",{},{}", self.receive.format(PacketCaptureFileOrInterfacePrefix::Receive), self.transmit.format(PacketCaptureFileOrInterfacePrefix::Transmit))
		}
	}
}

impl NetVirtualDevice for PacketCaptureNetVirtualDevice
{
}
