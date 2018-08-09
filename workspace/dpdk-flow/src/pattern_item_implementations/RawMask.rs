// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask for an `Pattern::Raw`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct RawMask
{
	/// Offset mask.
	pub offset: u32,
	
	/// Search area limit for start of pattern mask.
	pub search_area_limit_for_start_of_pattern: u16,
	
	/// Pattern mask.
	pub pattern: Box<[u8]>,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_raw,
}

custom_deserialize!
{
	RawMask,
	0 => offset,
	1 => search_area_limit_for_start_of_pattern,
	2 => pattern,
}

impl Clone for RawMask
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		let pattern = self.pattern.clone();
		Self
		{
			cached:
			{
				let mut clone = bitwise_clone!(self, rte_flow_item_raw);
				clone.pattern = pattern.as_ptr();
				clone
			},
			offset: self.offset,
			search_area_limit_for_start_of_pattern: self.search_area_limit_for_start_of_pattern,
			pattern,
		}
	}
}

impl PartialOrd for RawMask
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for RawMask
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.offset.cmp(&rhs.offset).then_with(|| self.search_area_limit_for_start_of_pattern.cmp(&rhs.search_area_limit_for_start_of_pattern)).then_with(|| self.pattern.cmp(&rhs.pattern))
	}
}

impl PartialEq for RawMask
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.offset == rhs.offset && self.search_area_limit_for_start_of_pattern == rhs.search_area_limit_for_start_of_pattern && self.pattern == rhs.pattern
	}
}

impl Eq for RawMask
{
}

impl Hash for RawMask
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.offset.hash(hasher);
		self.search_area_limit_for_start_of_pattern.hash(hasher);
		self.pattern.hash(hasher)
	}
}

impl MaskedPattern for RawMask
{
	type Type = rte_flow_item_raw;
}

impl Mask for RawMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPattern>::Type
	{
		&self.cached
	}
}

impl RawMask
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(offset: u32, search_area_limit_for_start_of_pattern: u16, mut pattern: Box<[u8]>) -> Self
	{
		debug_assert_ne!(pattern.len(), 0, "empty patterns are useless");
		debug_assert!(pattern.len() <= ::std::u16::MAX as usize, "pattern length '{}' exceeds ::std::u16::MAX '{}'", pattern.len(), ::std::u16::MAX);
		
		const Reserved: u32 = 0;
		
		Self
		{
			cached: rte_flow_item_raw
			{
				bitfield_1: rte_flow_item_raw::newbitfield_1(1, 1, Reserved),
				offset: offset as i32,
				limit: search_area_limit_for_start_of_pattern,
				length: pattern.len() as u16,
				pattern: pattern.as_mut_ptr(),
			},
			offset,
			search_area_limit_for_start_of_pattern,
			pattern,
		}
	}
}
