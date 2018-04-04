// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EthernetPortTransmitModeConfiguration
{
	pub multiQueuePacketTransmitMode: MultiQueuePacketTransmitMode,
	pub portBasedVlanInsertId: u16,
	pub hardwareShouldRejectSendingOutVlanTaggedPackets: bool,
	pub hardwareShouldRejectSendingOutVlanUntaggedPackets: bool,
	pub hardwarePortBasedVlanInsertionEnabled: bool,
}

impl Default for EthernetPortTransmitModeConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		EthernetPortTransmitModeConfiguration
		{
			multiQueuePacketTransmitMode: MultiQueuePacketTransmitMode::default(),
			portBasedVlanInsertId: 0,
			hardwareShouldRejectSendingOutVlanTaggedPackets: false,
			hardwareShouldRejectSendingOutVlanUntaggedPackets: false,
			hardwarePortBasedVlanInsertionEnabled: false,
		}
	}
}

impl EthernetPortTransmitModeConfiguration
{
	#[inline(always)]
	pub fn as_rte_eth_txmode(&self) -> rte_eth_txmode
	{	
		let mut txmode = rte_eth_txmode
		{
			mq_mode: self.multiQueuePacketTransmitMode.as_rte_eth_tx_mq_mode(),
			pvid: self.portBasedVlanInsertId,
			__bindgen_bitfield: 0,
		};
		
		unsafe
		{
			rust_rte_eth_txmode_setBitFields
			(
				&mut txmode,
				self.hardwareShouldRejectSendingOutVlanTaggedPackets,
				self.hardwareShouldRejectSendingOutVlanUntaggedPackets,
				self.hardwarePortBasedVlanInsertionEnabled
			)
		}
		
		txmode
	}
}
