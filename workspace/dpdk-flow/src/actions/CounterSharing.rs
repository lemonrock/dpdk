// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Is a counter globally shared?
///
/// A global counter may not be 'global' in the code sense; it implies that the counter is shared across a switch, a network card or even just a port identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum CounterSharing
{
	/// Counter is unique to flow.
	FlowUnique,
	
	/// Counter is globally shared.
	GloballyShared,
}

impl Default for CounterSharing
{
	#[inline(always)]
	fn default() -> Self
	{
		CounterSharing::FlowUnique
	}
}
