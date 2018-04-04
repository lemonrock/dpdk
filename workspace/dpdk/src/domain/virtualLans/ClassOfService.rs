// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ClassOfService
{
	Background = 1,
	BestEffort = 0, // Yes, this is correct
	ExcellentEffort = 2,
	CriticalApplication = 3,
	Video = 4,
	Voice = 5,
	InterNetworkControl = 6,
	NetworkControl = 7,
}

impl Default for ClassOfService
{
	#[inline(always)]
	fn default() -> Self
	{
		ClassOfService::BestEffort
	}
}

impl PartialOrd for ClassOfService
{
	fn partial_cmp(&self, other: &ClassOfService) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for ClassOfService
{
	fn cmp(&self, other: &ClassOfService) -> Ordering
	{
		self.orderValue().cmp(&other.orderValue())
	}
}

impl ClassOfService
{
	#[inline(always)]
	pub fn equivalentDifferentiatedServiceCodePoint(&self) -> DifferentiatedServiceCodePoint
	{
		match *self
		{
			ClassOfService::Background => DifferentiatedServiceCodePoint::DefaultForwarding,
			ClassOfService::BestEffort => DifferentiatedServiceCodePoint::AssuredForwardingClass1LowDropProbability,
			ClassOfService::ExcellentEffort => DifferentiatedServiceCodePoint::AssuredForwardingClass2LowDropProbability,
			ClassOfService::CriticalApplication => DifferentiatedServiceCodePoint::AssuredForwardingClass3LowDropProbability,
			ClassOfService::Video => DifferentiatedServiceCodePoint::AssuredForwardingClass4LowDropProbability,
			ClassOfService::Voice => DifferentiatedServiceCodePoint::ExpeditedForwarding,
			ClassOfService::InterNetworkControl => DifferentiatedServiceCodePoint::InterNetworkControl,
			ClassOfService::NetworkControl => DifferentiatedServiceCodePoint::NetworkControl,
		}
	}
	
	#[inline(always)]
	fn orderValue(&self) -> u8
	{
		match *self
		{
			ClassOfService::Background => 0,
			ClassOfService::BestEffort => 1,
			other @ _ => other as u8,
		}
	}
}
