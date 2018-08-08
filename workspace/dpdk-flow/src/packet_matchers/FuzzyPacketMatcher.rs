// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Fuzzy pattern match.
///
/// Not all devices will support a fuzzy match.
///
/// Usually a fuzzy match is fast but the cost is accuracy, eg Signature Match only match pattern's hash value, but it is possible two different patterns have the same hash value.
///
/// Matching accuracy level can be configure by a `threshold`.
///
/// These are mapped internally by a DPDK driver to the different accuracy levels that the underlying device supports.
#[derive(Debug)]
#[repr(transparent)]
pub struct FuzzyPacketMatcher
{
	underlying: rte_flow_item_fuzzy,
}

impl Clone for FuzzyPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		FuzzyPacketMatcher
		{
			underlying: rte_flow_item_fuzzy
			{
				thresh: self.underlying.thresh,
			}
		}
	}
}

impl PartialEq for FuzzyPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.underlying.thresh == rhs.underlying.thresh
	}
}

impl Eq for FuzzyPacketMatcher
{
}

impl PartialOrd for FuzzyPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for FuzzyPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.underlying.thresh.cmp(&rhs.underlying.thresh)
	}
}

impl Hash for FuzzyPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.underlying.thresh.hash(hasher)
	}
}

impl PacketMatcher for FuzzyPacketMatcher
{
	type DpdkType = rte_flow_item_fuzzy;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_FUZZY;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_fuzzy_mask }
	}
}

impl FuzzyPacketMatcher
{
	/// Creates a new instance.
	///
	/// * a `threshold` of zero (0) is a perfect match.
	/// * a `threshold` of 2^32 - 1 is the fuzziest match.
	#[inline(always)]
	pub fn new(threshold: u32) -> Self
	{
		Self
		{
			underlying: rte_flow_item_fuzzy
			{
				thresh: threshold,
			}
		}
	}
}
