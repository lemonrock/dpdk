// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// VLAN class of service.
///
/// Defaults to `BestEffort`.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ClassOfService
{
	Background = 1,
	BestEffort = 0, // Yes, this ordering is correct.
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
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for ClassOfService
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.order_value().cmp(&other.order_value())
	}
}

impl ClassOfService
{
	/// Equivalent `DifferentiatedServiceCodePoint`.
	#[inline(always)]
	pub fn equivalent_differentiated_service_code_point(&self) -> DifferentiatedServiceCodePoint
	{
		use self::ClassOfService::*;
		
		match *self
		{
			Background => DifferentiatedServiceCodePoint::DefaultForwarding,
			BestEffort => DifferentiatedServiceCodePoint::AssuredForwardingClass1LowDropProbability,
			ExcellentEffort => DifferentiatedServiceCodePoint::AssuredForwardingClass2LowDropProbability,
			CriticalApplication => DifferentiatedServiceCodePoint::AssuredForwardingClass3LowDropProbability,
			Video => DifferentiatedServiceCodePoint::AssuredForwardingClass4LowDropProbability,
			Voice => DifferentiatedServiceCodePoint::ExpeditedForwarding,
			InterNetworkControl => DifferentiatedServiceCodePoint::InterNetworkControl,
			NetworkControl => DifferentiatedServiceCodePoint::NetworkControl,
		}
	}
	
	#[inline(always)]
	fn order_value(&self) -> u8
	{
		use self::ClassOfService::*;
		
		match *self
		{
			Background => 0,
			
			BestEffort => 1,
			
			other @ _ => other as u8,
		}
	}
}
