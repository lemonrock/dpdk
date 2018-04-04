// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PacketCaptureFileOrInterface
{
	File(String),
	Interface(NetworkInterfaceName),
}

impl PacketCaptureFileOrInterface
{
	pub fn newFile(packetCaptureFilePath: &Path) -> Self
	{
		assert!(packetCaptureFilePath.exists(), "packetCaptureFile does not exist");
		let file = packetCaptureFilePath.to_str().expect("packetCaptureFilePath is not a valid UTF-8 string").to_owned();
		PacketCaptureFileOrInterface::File(file)
	}
	
	pub fn newInterface(interface: NetworkInterfaceName) -> Self
	{
		PacketCaptureFileOrInterface::Interface(interface)
	}
	
	pub fn isInterface(&self) -> bool
	{
		match *self
		{
			PacketCaptureFileOrInterface::File(_) => false,
			PacketCaptureFileOrInterface::Interface(_) => true,
		}
	}
	
	pub fn format(&self, packetCaptureFileOrInterfacePrefix: PacketCaptureFileOrInterfacePrefix) -> String
	{
		match *self
		{
			PacketCaptureFileOrInterface::File(ref packetCaptureFilePath) =>
			{
				debug_assert!(packetCaptureFileOrInterfacePrefix != PacketCaptureFileOrInterfacePrefix::Both);
				
				format!("{}pcap={}", packetCaptureFileOrInterfacePrefix.prefix().to_owned(), packetCaptureFilePath)
			},
			PacketCaptureFileOrInterface::Interface(ref interfaceName) => format!("{}iface={}", packetCaptureFileOrInterfacePrefix.prefix(), interfaceName.text()),
		}
	}
}
