// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet processing.
pub trait InternetProtocolVersion6PacketProcessing
{
	/// Process.
	#[inline(always)]
	fn process<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses);
}

impl InternetProtocolVersion6PacketProcessing for InternetProtocolVersion6Packet
{
	#[inline(always)]
	fn process<'a>(&'a mut self, packet: impl Packet, packet_processing: &PacketProcessing<impl PacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'a EthernetAddresses)
	{
		self.header.process(packet, packet_processing, layer_3_length, ethernet_addresses)
	}
}
