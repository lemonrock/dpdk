// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Virtual LAN value.
///
/// Defaults to no class-of-service and no drop-eligible indicator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct VirtualLanValue
{
	class_of_service: ClassOfService,
	drop_eligible_indicator: bool,
}

impl Default for VirtualLanValue
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			class_of_service: Default::default(),
			drop_eligible_indicator: false,
		}
	}
}

impl VirtualLanValue
{
	/// Virtual LAN value equivalent to unspecified.
	pub fn equivalent_to_unspecified(&self) -> bool
	{
		self.class_of_service == ClassOfService::BestEffort && self.drop_eligible_indicator == false
	}
}
