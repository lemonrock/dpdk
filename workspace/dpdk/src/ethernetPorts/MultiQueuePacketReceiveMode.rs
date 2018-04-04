// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub flags MultiQueuePacketReceiveMode: u32
	{
		const ReceiveSideScaling = 1,
		const DataCentreBridging = 2,
		const VMDq = 4,
	}
}

impl Default for MultiQueuePacketReceiveMode
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::ReceiveSideScaling
	}
}

impl MultiQueuePacketReceiveMode
{
	#[inline(always)]
	pub fn as_rte_eth_rx_mq_mode(&self) -> rte_eth_rx_mq_mode
	{
		unsafe { transmute(self.bits) }
	}
	
	#[inline(always)]
	pub fn enableReceiveSideScaling(&mut self)
	{
		self.insert(MultiQueuePacketReceiveMode::ReceiveSideScaling);
	}

	#[inline(always)]
	pub fn disableReceiveSideScaling(&mut self)
	{
		self.remove(MultiQueuePacketReceiveMode::ReceiveSideScaling);
	}
}
