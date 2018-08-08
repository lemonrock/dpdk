// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Matches any protocol in place of the current layer, a single instance may also stand for several protocol layers.
///
/// This is usually specified as the first pattern item when looking for a protocol anywhere in a packet.
#[derive(Debug)]
#[repr(transparent)]
pub struct AnyPacketMatcher
{
	underlying: rte_flow_item_any,
}

impl Clone for AnyPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		AnyPacketMatcher
		{
			underlying: rte_flow_item_any
			{
				num: self.underlying.num,
			}
		}
	}
}

impl PartialEq for AnyPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.underlying.num == rhs.underlying.num
	}
}

impl Eq for AnyPacketMatcher
{
}

impl PartialOrd for AnyPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for AnyPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.underlying.num.cmp(&rhs.underlying.num)
	}
}

impl Hash for AnyPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.underlying.num.hash(hasher)
	}
}

impl PacketMatcher for AnyPacketMatcher
{
	type DpdkType = rte_flow_item_any;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_ANY;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_any_mask }
	}
}

impl AnyPacketMatcher
{
	/// Creates a new instance.
	///
	/// If `number_of_layers_covered` is zero then matches any layer.
	#[inline(always)]
	pub fn new(number_of_layers_covered: u32) -> Self
	{
		Self
		{
			underlying: rte_flow_item_any
			{
				num: number_of_layers_covered,
			}
		}
	}
}
