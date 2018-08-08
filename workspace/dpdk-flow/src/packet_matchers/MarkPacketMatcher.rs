// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mark pattern match.
///
/// Not all devices will support a mark match, and, of those that do, not all will support the full range from 0 to 2^32 - 1 inclusive.
///
/// A mark match matches a packet that has previously been 'marked' with a marking action. Marks are stored inside the `rte_mbuf` in the same union as the Receive Side Scaling (RSS) hash.
///
/// As of DPDK 18.05, this functionality is experimental.
#[derive(Debug)]
#[repr(transparent)]
pub struct MarkPacketMatcher
{
	underlying: rte_flow_item_mark,
}

impl Clone for MarkPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		MarkPacketMatcher
		{
			underlying: rte_flow_item_mark
			{
				id: self.underlying.id,
			}
		}
	}
}

impl PartialEq for MarkPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.underlying.id == rhs.underlying.id
	}
}

impl Eq for MarkPacketMatcher
{
}

impl PartialOrd for MarkPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for MarkPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.underlying.id.cmp(&rhs.underlying.id)
	}
}

impl Hash for MarkPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.underlying.id.hash(hasher)
	}
}

impl PacketMatcher for MarkPacketMatcher
{
	type DpdkType = rte_flow_item_mark;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_FUZZY;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		static Mask: rte_flow_item_mark = rte_flow_item_mark
		{
			id: 0xFFFFFFFF,
		};
		
		&Mask
	}
}

impl MarkPacketMatcher
{
	/// Creates a new instance.
	///
	/// This matcher is always an exact match.
	#[inline(always)]
	pub fn new(mark: u32) -> Self
	{
		Self
		{
			underlying: rte_flow_item_mark
			{
				id: mark,
			}
		}
	}
}
