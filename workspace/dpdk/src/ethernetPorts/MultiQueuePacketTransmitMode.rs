// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub flags MultiQueuePacketTransmitMode: u32
	{
		const DataCentreBridging = 1,
		const VMDq = 2,
	}
}

impl Default for MultiQueuePacketTransmitMode
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl MultiQueuePacketTransmitMode
{
	#[inline(always)]
	pub fn as_rte_eth_tx_mq_mode(&self) -> rte_eth_tx_mq_mode
	{
		match self.bits
		{
			0 => rte_eth_tx_mq_mode::ETH_MQ_TX_NONE,
			1 => rte_eth_tx_mq_mode::ETH_MQ_TX_DCB,
			2 => rte_eth_tx_mq_mode::ETH_MQ_TX_VMDQ_ONLY,
			3 => rte_eth_tx_mq_mode::ETH_MQ_TX_VMDQ_DCB,
			
			illegal @ _ => panic!("MultiQueuePacketTransmitMode should not have bits values '{}'", illegal),
		}
	}
}
