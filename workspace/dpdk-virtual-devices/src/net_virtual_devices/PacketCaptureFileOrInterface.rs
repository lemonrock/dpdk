// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet capture (pcap) file or interface.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PacketCaptureFileOrInterface
{
	/// File.
	File(String),
	
	/// Network interface.
	Interface(NetworkInterfaceName),
}

impl PacketCaptureFileOrInterface
{
	/// New instance.
	#[inline(always)]
	pub fn new_file(packet_capture_file_path: &Path) -> Self
	{
		assert!(packet_capture_file_path.exists(), "packetCaptureFile does not exist");
		let file = packet_capture_file_path.to_str().expect("packet_capture_file_path is not a valid UTF-8 string").to_owned();
		PacketCaptureFileOrInterface::File(file)
	}
	
	/// New instance.
	#[inline(always)]
	pub fn new_interface(interface: NetworkInterfaceName) -> Self
	{
		PacketCaptureFileOrInterface::Interface(interface)
	}
	
	/// Is interface.
	#[inline(always)]
	pub fn is_interface(&self) -> bool
	{
		use self::PacketCaptureFileOrInterface::*;
		
		match *self
		{
			File(_) => false,
			Interface(_) => true,
		}
	}
	
	#[inline(always)]
	pub(crate) fn format(&self, packet_capture_file_or_interface_prefix: PacketCaptureFileOrInterfacePrefix) -> String
	{
		use self::PacketCaptureFileOrInterface::*;
		
		match *self
		{
			File(ref packet_capture_file_path) =>
			{
				debug_assert!(packet_capture_file_or_interface_prefix != PacketCaptureFileOrInterfacePrefix::Both);
				
				format!("{}pcap={}", packet_capture_file_or_interface_prefix.prefix().to_owned(), packet_capture_file_path)
			},
			Interface(ref network_interface_name) => format!("{}iface={}", packet_capture_file_or_interface_prefix.prefix(), network_interface_name.text()),
		}
	}
}
