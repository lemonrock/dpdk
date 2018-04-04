// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct VirtualLanValue
{
	classOfService: ClassOfService,
	dropEligibleIndicator: bool,
}

impl Default for VirtualLanValue
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			classOfService: Default::default(),
			dropEligibleIndicator: false,
		}
	}
}

impl VirtualLanValue
{
	pub fn equivalentToUnspecified(&self) -> bool
	{
		self.classOfService == ClassOfService::BestEffort && self.dropEligibleIndicator == false
	}
}
