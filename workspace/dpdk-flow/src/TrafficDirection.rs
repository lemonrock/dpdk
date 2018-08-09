// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Traffic direction.
///
/// Defaults to `Ingress`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum TrafficDirection
{
	/// Applies to traffic ingress.
	Ingress,
	
	/// Applies to traffic egress.
	Egress,
	
	/// Applies to traffic in both the ingress and egress directions.
	Both,
}

impl Default for TrafficDirection
{
	#[inline(always)]
	fn default() -> Self
	{
		TrafficDirection::Ingress
	}
}

impl TrafficDirection
{
	#[inline(always)]
	pub(crate) fn ingress_and_egress_bits(self) -> (u32, u32)
	{
		use self::TrafficDirection::*;
		
		match self
		{
			Ingress => (1, 0),
			Egress => (0, 1),
			Both => (1, 1),
		}
	}
}
