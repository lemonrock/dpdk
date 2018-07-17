// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum Bridge
{
	HostBridge = 0x00,
	ISABridge = 0x01,
	EISABridge = 0x02,
	MicroChannelBridge = 0x03,
	PciBridge = 0x04,
	
	/// PCMCIA.
	PcmciaBridge = 0x05,
	NuBusBridge = 0x06,
	CardBusBridge = 0x07,
	
	/// RACEway.
	RaceWayBridge = 0x08,
	SemiTransparentPciToPciBridge = 0x09,
	InfiniBandToPciHostBridge = 0x0a,
	
	/// No effective sub class.
	Bridge = 0x80,
}
