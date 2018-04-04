// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LongestPrefixMatchName
{
	pub ethernetPortIdentifier: EthernetPortIdentifier,
	pub queueIdentifier: QueueIdentifier,
	pub virtualLanKey: VirtualLanKey,
}

impl LongestPrefixMatchName
{
	pub fn toName(&self, prefix: &str) -> String
	{
		let outerVlan = match self.virtualLanKey.0
		{
			None => 0,
			Some(value) => value.value(),
		};
		
		let innerVlan = match self.virtualLanKey.1
		{
			None => 0,
			Some(value) => value.value(),
		};
		
		format!("{}-{}-{}-{}-{}", prefix, self.ethernetPortIdentifier, self.queueIdentifier, outerVlan, innerVlan)
	}
}
