// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification for an `Pattern::Raw`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct RawSpecification
{
	/// Offset.
	pub offset: i32,
	
	/// Search area limit for start of pattern.
	pub search_area_limit_for_start_of_pattern: u16,
	
	/// Pattern.
	pub pattern: Box<[u8]>,
	
	/// Look for pattern after the previous item's offset.
	pub look_for_pattern_after_the_previous_item_offset: bool,
	
	/// Relative offset.
	pub relative_offset: bool,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_raw,
}

custom_deserialize!
{
	RawSpecification,
	0 => offset,
	1 => search_area_limit_for_start_of_pattern,
	2 => pattern,
	3 => look_for_pattern_after_the_previous_item_offset,
	4 => relative_offset,
}

impl Clone for RawSpecification
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
			look_for_pattern_after_the_previous_item_offset: self.look_for_pattern_after_the_previous_item_offset,
			relative_offset: self.relative_offset,
		}
	}
}

impl PartialOrd for RawSpecification
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for RawSpecification
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.offset.cmp(&rhs.offset).then_with(|| self.search_area_limit_for_start_of_pattern.cmp(&rhs.search_area_limit_for_start_of_pattern)).then_with(|| self.pattern.cmp(&rhs.pattern)).then_with(|| self.look_for_pattern_after_the_previous_item_offset.cmp(&rhs.look_for_pattern_after_the_previous_item_offset)).then_with(|| self.relative_offset.cmp(&rhs.relative_offset))
	}
}

impl PartialEq for RawSpecification
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.offset == rhs.offset && self.search_area_limit_for_start_of_pattern == rhs.search_area_limit_for_start_of_pattern && self.pattern == rhs.pattern && self.look_for_pattern_after_the_previous_item_offset == rhs.look_for_pattern_after_the_previous_item_offset && self.relative_offset == rhs.relative_offset
	}
}

impl Eq for RawSpecification
{
}

impl Hash for RawSpecification
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.offset.hash(hasher);
		self.search_area_limit_for_start_of_pattern.hash(hasher);
		self.pattern.hash(hasher);
		self.look_for_pattern_after_the_previous_item_offset.hash(hasher);
		self.relative_offset.hash(hasher)
	}
}

impl MaskedPattern for RawSpecification
{
	type Type = rte_flow_item_raw;
}

impl Specification for RawSpecification
{
	const DpdkFlowType: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_RAW;
	
	type Mask = RawMask;
	
	#[inline(always)]
	fn dpdk_specification(&self) -> &<Self as MaskedPattern>::Type
	{
		&self.cached
	}
}

impl RawSpecification
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(offset: i32, search_area_limit_for_start_of_pattern: u16, mut pattern: Box<[u8]>, look_for_pattern_after_the_previous_item_offset: bool, relative_offset: bool) -> Self
	{
		debug_assert_ne!(pattern.len(), 0, "empty patterns are useless");
		debug_assert!(pattern.len() <= ::std::u16::MAX as usize, "pattern length '{}' exceeds ::std::u16::MAX '{}'", pattern.len(), ::std::u16::MAX);
		
		const Reserved: u32 = 0;
		
		let relative = if look_for_pattern_after_the_previous_item_offset
		{
			1
		}
		else
		{
			0
		};
		
		let search = if relative_offset
		{
			1
		}
		else
		{
			0
		};
		
		Self
		{
			cached: rte_flow_item_raw
			{
				bitfield_1: rte_flow_item_raw::newbitfield_1(relative, search, Reserved),
				offset,
				limit: search_area_limit_for_start_of_pattern,
				length: pattern.len() as u16,
				pattern: pattern.as_mut_ptr(),
			},
			offset,
			search_area_limit_for_start_of_pattern,
			pattern,
			look_for_pattern_after_the_previous_item_offset,
			relative_offset,
		}
	}
}
