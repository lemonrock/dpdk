// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// packet capture (pcap) net(work) virtual device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PacketCaptureNetVirtualDevice
{
	index: u5,
	receive: PacketCaptureFileOrInterface,
	transmit: PacketCaptureFileOrInterface,
}

impl VirtualDevice for PacketCaptureNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::PacketCapture;

	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		if self.receive.isInterface() && self.transmit.isInterface() && self.receive == self.transmit
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

impl PacketCaptureNetVirtualDevice
{
	/// New instance.
	#[inline(always)]
	pub fn from_and_to_same_interface(index: u5, interface: NetworkInterfaceName) -> Self
	{
		Self::from_interface_to_interface(index, interface.clone(), interface)
	}
	
	/// New instance.
	#[inline(always)]
	pub fn from_interface_to_interface(index: u5, from_interface: NetworkInterfaceName, to_interface: NetworkInterfaceName) -> Self
	{
		let receive = PacketCaptureFileOrInterface::newInterface(from_interface);
		let transmit = PacketCaptureFileOrInterface::newInterface(to_interface);
		Self::new(index, receive, transmit)
	}
	
	/// New instance.
	#[inline(always)]
	pub fn from_interface_to_file(index: u5, from_interface: NetworkInterfaceName, to_packet_capture_file_path: &Path) -> Self
	{
		let receive = PacketCaptureFileOrInterface::newInterface(from_interface);
		let transmit = PacketCaptureFileOrInterface::newFile(to_packet_capture_file_path);
		Self::new(index, receive, transmit)
	}
	
	/// New instance.
	#[inline(always)]
	pub fn from_file_to_interface(index: u5, from_packet_capture_file_path: &Path, to_interface: NetworkInterfaceName) -> Self
	{
		let receive = PacketCaptureFileOrInterface::newFile(from_packet_capture_file_path);
		let transmit = PacketCaptureFileOrInterface::newInterface(to_interface);
		Self::new(index, receive, transmit)
	}
	
	/// New instance.
	#[inline(always)]
	pub fn from_file_to_file(index: u5, from_packet_capture_file_path: &Path, to_packet_capture_file_path: &Path) -> Self
	{
		assert_ne!(from_packet_capture_file_path, to_packet_capture_file_path, "from_packet_capture_file_path and to_packet_capture_file_path can not be the same file");

		let receive = PacketCaptureFileOrInterface::newFile(from_packet_capture_file_path);
		let transmit = PacketCaptureFileOrInterface::newFile(to_packet_capture_file_path);
		Self::new(index, receive, transmit)
	}
	
	/// New instance.
	#[inline(always)]
	fn new(index: u5, receive: PacketCaptureFileOrInterface, transmit: PacketCaptureFileOrInterface) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);

		PacketCaptureNetVirtualDevice
		{
			index,
			receive,
			transmit,
		}
	}
}
