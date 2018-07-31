// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Alignment bit mask.
#[repr(usize)]
pub enum AlignmentBitMask
{
	/// AVX-512 Foundation.
	_512_bit = (1 << 6) - 1,
	
	/// AVX.
	_256_bit = (1 << 5) - 1,
	
	/// SSE2.
	_128_bit = (1 << 4) - 1,
}

impl AlignmentBitMask
{
	/// Is this `memory` aligned with this bit mask?
	#[inline(always)]
	pub fn is_aligned(self, pointer: usize) -> bool
	{
		pointer & (self as usize) == 0
	}
	
	/// Is this `memory` unaligned with this bit mask?
	#[inline(always)]
	pub fn is_unaligned(self, pointer: usize) -> bool
	{
		pointer & (self as usize) != 0
	}
	
	/// Returns the preferred alignment for this CPU.
	///
	/// Determined at compile time.
	#[inline(always)]
	pub fn preferred_alignment() -> Self
	{
		use self::AlignmentBitMask::*;
		
		if cfg!(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))
		{
			_512_bit
		}
		else if cfg!(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx"))
		{
			_256_bit
		}
		else
		{
			_128_bit
		}
	}
}
