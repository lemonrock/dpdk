// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


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
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String
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
	#[inline(always)]
	pub fn fromAndToSameInterface(index: u5, interface: NetworkInterfaceName) -> Self
	{
		Self::fromInterfaceToInterface(index, interface.clone(), interface)
	}
	
	#[inline(always)]
	pub fn fromInterfaceToInterface(index: u5, fromInterface: NetworkInterfaceName, toInterface: NetworkInterfaceName) -> Self
	{
		let receive = PacketCaptureFileOrInterface::newInterface(fromInterface);
		let transmit = PacketCaptureFileOrInterface::newInterface(toInterface);
		Self::new(index, receive, transmit)
	}
	
	#[inline(always)]
	pub fn fromInterfaceToFile(index: u5, fromInterface: NetworkInterfaceName, toPacketCaptureFilePath: &Path) -> Self
	{
		let receive = PacketCaptureFileOrInterface::newInterface(fromInterface);
		let transmit = PacketCaptureFileOrInterface::newFile(toPacketCaptureFilePath);
		Self::new(index, receive, transmit)
	}
	
	#[inline(always)]
	pub fn fromFileToInterface(index: u5, fromPacketCaptureFilePath: &Path, toInterface: NetworkInterfaceName) -> Self
	{
		let receive = PacketCaptureFileOrInterface::newFile(fromPacketCaptureFilePath);
		let transmit = PacketCaptureFileOrInterface::newInterface(toInterface);
		Self::new(index, receive, transmit)
	}
	
	#[inline(always)]
	pub fn fromFileToFile(index: u5, fromPacketCaptureFilePath: &Path, toPacketCaptureFilePath: &Path) -> Self
	{
		assert!(fromPacketCaptureFilePath != toPacketCaptureFilePath, "fromPacketCaptureFilePath and toPacketCaptureFilePath can not be the same file");
		
		let receive = PacketCaptureFileOrInterface::newFile(fromPacketCaptureFilePath);
		let transmit = PacketCaptureFileOrInterface::newFile(toPacketCaptureFilePath);
		Self::new(index, receive, transmit)
	}
	
	#[inline(always)]
	fn new(index: u5, receive: PacketCaptureFileOrInterface, transmit: PacketCaptureFileOrInterface) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);
		
		PacketCaptureNetVirtualDevice
		{
			index: index,
			receive: receive,
			transmit: transmit,
		}
	}
}
