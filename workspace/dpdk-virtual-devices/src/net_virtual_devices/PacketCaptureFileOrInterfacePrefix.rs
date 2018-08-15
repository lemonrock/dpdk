// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet capture (pcap) choice.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PacketCaptureFileOrInterfacePrefix
{
	/// Used for receive.
	Receive,
	
	/// Used for transmit.
	Transmit,
	
	/// Used for receive and transmit.
	Both,
}

impl Default for PacketCaptureFileOrInterfacePrefix
{
	#[inline(always)]
	fn default() -> Self
	{
		PacketCaptureFileOrInterfacePrefix::Both
	}
}

impl PacketCaptureFileOrInterfacePrefix
{
	#[inline(always)]
	fn prefix(&self) -> &'static str
	{
		use self::PacketCaptureFileOrInterfacePrefix::*;
		
		match *self
		{
			Receive => "rx_",
			Transmit => "tx_",
			Both => "",
		}
	}
}
