// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct MemoryConfiguration
{
	limits: Option<MemoryLimits>,
	channels: Option<MemoryChannels>,
	ranks: Option<MemoryRanks>,
}

impl Default for MemoryConfiguration
{
	fn default() -> Self
	{
		MemoryConfiguration
		{
			limits: None,
			channels: None,
			ranks: None,
		}
	}
}

impl MemoryConfiguration
{
	pub fn addTo(&self, dpdkRteInitData: &mut DpdkRteInitData)
	{
		dpdkRteInitData.addMemorySettings(self.limits, self.channels, self.ranks);
	}
}
