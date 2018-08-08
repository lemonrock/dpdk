// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A flow item that matches raw data using a pattern.
#[derive(Debug)]
#[repr(transparent)]
pub struct RawPacketMatcher
{
	underlying: rte_flow_item_raw,
}

impl Drop for RawPacketMatcher
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.pattern_is_not_null()
		{
			drop(self.pattern_as_boxed_slice())
		}
	}
}

impl Clone for RawPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		let mut clone = RawPacketMatcher
		{
			underlying: generic_clone(&self.underlying),
		};
		
		if clone.pattern_is_not_null()
		{
			clone.underlying.pattern = Self::pattern_to_raw(self.pattern_cloned())
		}
		
		clone
	}
}

impl PartialEq for RawPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		generic_equals(&self.underlying, &rhs.underlying)
	}
}

impl Eq for RawPacketMatcher
{
}

impl PartialOrd for RawPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for RawPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		generic_compare(&self.underlying, &rhs.underlying)
	}
}

impl Hash for RawPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		generic_hash::<H, _>(self, hasher)
	}
}

impl PacketMatcher for RawPacketMatcher
{
	type DpdkType = rte_flow_item_raw;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_RAW;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_raw_mask }
	}
}

impl RawPacketMatcher
{
	/// * `offset` can be absolute or relative; DPDK documentation is a little difficult to flow.
	/// * `pattern` can not be longer than 65,535 bytes and must not be empty (zero, 0).
	#[inline(always)]
	pub fn new(look_for_pattern_after_the_previous_item_offset: bool, search_pattern_from_offset: bool, offset: i32, search_area_limit_for_start_of_pattern: u16, pattern: Box<[u8]>) -> Self
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
		
		let search = if search_pattern_from_offset
		{
			1
		}
		else
		{
			0
		};
		
		let this = Self
		{
			underlying: rte_flow_item_raw
			{
				bitfield_1: rte_flow_item_raw::newbitfield_1(relative, search, Reserved),
				offset,
				limit: search_area_limit_for_start_of_pattern,
				length: pattern.len() as u16,
				pattern: Self::pattern_to_raw(pattern),
			}
		};
		
		this
	}
	
	#[inline(always)]
	fn pattern_is_not_null(&self) -> bool
	{
		!self.underlying.pattern.is_null()
	}
	
	#[inline(always)]
	fn pattern_to_raw(mut pattern: Box<[u8]>) -> *const u8
	{
		let pointer = pattern.as_mut_ptr();
		forget(pattern);
		pointer as *const _
	}
	
	#[inline(always)]
	fn pattern_as_boxed_slice(&self) -> Box<[u8]>
	{
		unsafe
		{
			let slice = from_raw_parts_mut(self.underlying.pattern as *mut _, self.underlying.length as usize);
			Box::from_raw(slice)
		}
	}
	
	#[inline(always)]
	fn pattern_cloned(&self) -> Box<[u8]>
	{
		let pattern = self.pattern_as_boxed_slice();
		let clone = pattern.clone();
		forget(pattern);
		clone
	}
}
