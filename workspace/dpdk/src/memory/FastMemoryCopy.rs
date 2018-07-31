// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(all(target_arch = "x86", target_feature = "sse2"))] use ::std::arch::x86::
{
	__m128i,
	_mm_loadu_si128,
	_mm_store_si128,
	_mm_storeu_si128,
};
#[cfg(all(target_arch = "x86", target_feature = "sse2"))] use ::std::arch::x86::
{
	__m128i,
	_mm_loadu_si128,
	_mm_store_si128,
	_mm_storeu_si128,
};
#[cfg(all(target_arch = "x86_64", target_feature = "avx"))] use ::std::arch::x86_64::
{
	__m256i,
	_mm256_loadu_si256,
	_mm256_store_si256,
	_mm256_storeu_si256,
};
#[cfg(all(target_arch = "x86_64", target_feature = "avx"))] use ::std::arch::x86_64::
{
	__m256i,
	_mm256_loadu_si256,
	_mm256_store_si256,
	_mm256_storeu_si256,
};
#[cfg(all(target_arch = "x86_64", target_feature = "avxf"))] use ::std::arch::x86_64::
{
	__m512i,
	_mm512_loadu_si512,
	_mm512_store_si512,
	_mm512_storeu_si512,
};
#[cfg(all(target_arch = "x86_64", target_feature = "avxf"))] use ::std::arch::x86_64::
{
	__m512i,
	_mm512_loadu_si512,
	_mm512_store_si512,
	_mm512_storeu_si512,
};

// SSE4.1 : _mm_stream_load_si128 is better to prevent cache eviction.
// Does not appear to exist for 256 or 512.
// Must be 16-byte aligned.


/// Fast memory copy routines from DPDK.
pub trait FastMemoryCopy
{
	type Destination: FastMemoryCopyDestination;
	
	/// Copies 16 bytes to `destination` using SSE2 instructions.
	///
	/// * Source and destination must not overlap.
	/// * Source can be aligned or unaligned.
	/// * Destination can be aligned or unaligned.
	///
	/// Falls back to `copy_nonoverlapping`.
	#[inline(always)]
	unsafe fn copy_nonoverlapping_16_bytes_to_any_destination(self, destination: Self::Destination)
	{
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
		#[target_feature(enable = "sse2")]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			let a = _mm_loadu_si128(source as *const __m128i);
			_mm_storeu_si128(destination as *mut __m128i, a);
		}
		
		#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2")))]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			copy_nonoverlapping(source, destination, 16)
		}
		
		copy(self.as_constant_pointer(), destination.as_mutable_pointer())
	}
	
	/// Copies 16 bytes to `aligned_destination` using SSE2 instructions.
	///
	/// * Source and destination must not overlap.
	/// * Source can be aligned or unaligned.
	/// * Destination must be aligned on 16 byte boundary.
	///
	/// Falls back to `copy_nonoverlapping`.
	#[inline(always)]
	unsafe fn copy_nonoverlapping_16_bytes_to_aligned_destination(self, aligned_destination: Self::Destination)
	{
		debug_assert_eq!(aligned_destination.as_usize() % 16, 0, "aligned_destination is not 16-byte aligned");
		
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
		#[target_feature(enable = "sse2")]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			let a = _mm_loadu_si128(source as *const __m128i);
			_mm_store_si128(destination as *mut __m128i, a);
		}
		
		#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2")))]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			copy_nonoverlapping(source, destination, 16)
		}
		
		copy(self.as_constant_pointer(), aligned_destination.as_mutable_pointer())
	}
	
	/// Copies 32 bytes to `destination` using AVX instructions.
	///
	/// * Source and destination must not overlap.
	/// * Source can be aligned or unaligned.
	/// * Destination can be aligned or unaligned.
	///
	/// Falls back to `copy_nonoverlapping_16_bytes_to_any_destination` x2.
	#[inline(always)]
	unsafe fn copy_nonoverlapping_32_bytes_to_any_destination(self, destination: Self::Destination)
	{
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx"))]
		#[target_feature(enable = "avx")]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			let a = _mm256_loadu_si256(source as *const __m256i);
			_mm256_storeu_si256(destination as *mut __m256i, a);
		}
		
		#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx")))]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			source.to_memory().copy_nonoverlapping_16_bytes_to_any_destination(destination.to_memory());
			source.offset_16().copy_nonoverlapping_16_bytes_to_any_destination(destination.offset_16());
		}
		
		copy(self.as_constant_pointer(), destination.as_mutable_pointer())
	}
	
	/// Copies 32 bytes to `aligned_destination` using AVX instructions.
	///
	/// * Source and destination must not overlap.
	/// * Source can be aligned or unaligned.
	/// * Destination must be aligned on 32 byte boundary.
	///
	/// Falls back to `copy_nonoverlapping_16_bytes_to_aligned_destination` x2.
	#[inline(always)]
	unsafe fn copy_nonoverlapping_32_bytes_to_aligned_destination(self, aligned_destination: Self::Destination)
	{
		debug_assert_eq!(aligned_destination.as_usize() % 32, 0, "aligned_destination is not 32-byte aligned");
		
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx"))]
		#[target_feature(enable = "avx")]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			let a = _mm256_loadu_si256(source as *const __m256i);
			_mm256_store_si256(destination as *mut __m256i, a);
		}
		
		#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx")))]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			source.to_memory().copy_nonoverlapping_16_bytes_to_aligned_destination(destination.to_memory());
			source.offset_16().copy_nonoverlapping_16_bytes_to_aligned_destination(destination.offset_16());
		}
		
		copy(self.as_constant_pointer(), aligned_destination.as_mutable_pointer())
	}
	
	/// Copies 64 bytes to `destination` using AVX-512 instructions.
	///
	/// * Source and destination must not overlap.
	/// * Source can be aligned or unaligned.
	/// * Destination can be aligned or unaligned.
	///
	/// Falls back to `copy_nonoverlapping_32_bytes_to_any_destination` x2.
	#[inline(always)]
	unsafe fn copy_nonoverlapping_64_bytes_to_any_destination(self, destination: Self::Destination)
	{
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
		#[target_feature(enable = "avx512f")]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			let a = _mm512_loadu_si512(source as *const __m512i);
			_mm512_storeu_si512(destination as *mut __m512i, a);
		}
		
		#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f")))]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			source.to_memory().copy_nonoverlapping_32_bytes_to_aligned_destination(destination.to_memory());
			source.offset_32().copy_nonoverlapping_32_bytes_to_aligned_destination(destination.offset_32());
		}
		
		copy(self.as_constant_pointer(), destination.as_mutable_pointer())
	}
	
	/// Copies 64 bytes to `aligned_destination` using AVX-512 instructions.
	///
	/// * Source and destination must not overlap.
	/// * Source can be aligned or unaligned.
	/// * Destination must be aligned on 64 byte boundary.
	///
	/// Falls back to `copy_nonoverlapping_16_bytes_to_aligned_destination` x2.
	#[inline(always)]
	unsafe fn copy_nonoverlapping_64_bytes_to_aligned_destination(self, aligned_destination: Self::Destination)
	{
		debug_assert_eq!(aligned_destination.as_usize() % 64, 0, "aligned_destination is not 16-byte aligned");
		
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
		#[target_feature(enable = "avx512f")]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			let a = _mm512_loadu_si512(source as *const __m512i);
			_mm512_store_si512(destination as *mut __m512i, a);
		}
		
		#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f")))]
		#[inline(always)]
		unsafe fn copy(source: *const u8, destination: *mut u8)
		{
			source.to_memory().copy_nonoverlapping_32_bytes_to_aligned_destination(destination.to_memory());
			source.offset_32().copy_nonoverlapping_32_bytes_to_aligned_destination(destination.offset_32());
		}
		
		copy(self.as_constant_pointer(), aligned_destination.as_mutable_pointer())
	}
	
	/// Copies `length` bytes to `destination` using the best combination of instructions.
	///
	/// * Source and destination must not overlap.
	/// * Source can be aligned or unaligned.
	/// * Destination can be aligned or unaligned.
	///
	#[inline(always)]
	unsafe fn copy_nonoverlapping_bytes_from_source_to_destination(self, destination: Self::Destination, size: usize)
	{
		let mut source = self.as_usize();
		let mut destination = destination.as_usize();
		
		// Copy size < 16 bytes.
		if size < 16
		{
			if size & 0x01 != 0
			{
				* (destination as *mut u8) = * (source as *const u8);
				source += 1;
				destination += 1;
			}
			
			if size & 0x02 != 0
			{
				* (destination as *mut u16) = * (source as *const u16);
				source += 2;
				destination += 2;
			}
			
			if size & 0x04 != 0
			{
				* (destination as *mut u32) = * (source as *const u32);
				source += 4;
				destination += 4;
			}
			
			if size & 0x08 != 0
			{
				* (destination as *mut u64) = * (source as *const u64);
			}
			
			return
		}
		
		use self::AlignmentBitMask::*;
		
		// Copy 16 <= size <= 32 bytes.
		if size <= 32
		{
			// TODO: There is, in SSE4.1, an aligned 16 byte load intrinsic which does not cause cache eviction: _mm_stream_load_si128.
			
			if _128_bit.is_aligned(destination)
			{
				source.copy_nonoverlapping_16_bytes_to_aligned_destination(destination);
			}
			else
			{
				source.copy_nonoverlapping_16_bytes_to_any_destination(destination);
			}
			
			let remainder = size - 16;
			(source + remainder).copy_nonoverlapping_16_bytes_to_any_destination(destination + remainder);
			
			return
		}
		
		// Copy 32 < size <= 48 bytes.
		if size <= 48
		{
			if _256_bit.is_aligned(destination)
			{
				source.copy_nonoverlapping_32_bytes_to_aligned_destination(destination);
			}
			else
			{
				// NOTE: In theory, we could now test for 128-bit alignment and use copy_nonoverlapping_16_bytes_to_aligned_destination followed by copy_nonoverlapping_16_bytes_to_destination  but this costs more (a comparison branch and two copy instructions).
				
				source.copy_nonoverlapping_32_bytes_to_any_destination(destination);
			}
			
			let remainder = size - 32;
			(source + remainder).copy_nonoverlapping_16_bytes_to_any_destination(destination + remainder);
			
			return
		}
		
		// Copy 32 < size <= 64 bytes.
		if size <= 64
		{
			if _256_bit.is_aligned(destination)
			{
				source.copy_nonoverlapping_32_bytes_to_aligned_destination(destination);
			}
			else
			{
				// NOTE: In theory, we could now test for 128-bit alignment and use copy_nonoverlapping_16_bytes_to_aligned_destination followed by copy_nonoverlapping_16_bytes_to_destination  but this costs more (a comparison branch and two copy instructions).
				
				source.copy_nonoverlapping_32_bytes_to_any_destination(destination);
			}
			
			let remainder = size - 32;
			(source + remainder).copy_nonoverlapping_32_bytes_to_any_destination(destination + remainder);
			
			return
		}
		
		// Copy 64 < size <= 128 bytes.
		if size <= 128
		{
			if _512_bit.is_aligned(destination)
			{
				source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination);
			}
			else
			{
				// NOTE: In theory, we could now test for 256-bit or 128-bit alignment but the costs of the extra comparisons and multiple move instructions aren't worthwhile.
				
				source.copy_nonoverlapping_64_bytes_to_any_destination(destination);
			}
			
			let remainder = size - 64;
			
			if remainder <= 32
			{
				(source + remainder).copy_nonoverlapping_32_bytes_to_any_destination(destination + remainder);
			}
			else
			{
				(source + remainder).copy_nonoverlapping_64_bytes_to_any_destination(destination + remainder);
			}
			
			return
		}
		
		let mut size = size;
		
		// Above 256 bytes (AVX) and 512 bytes (AVX-512 Foundation) copies to aligned destinations are more efficient.
		if cfg!(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))
		{
			let is_destination_aligned = _512_bit.is_aligned(destination);
			
			if size <= 512
			{
				if is_destination_aligned
				{
					if size >= 256
					{
						source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination)
						(source + 64).copy_nonoverlapping_64_bytes_to_aligned_destination(destination + 64)
						(source + 128).copy_nonoverlapping_64_bytes_to_aligned_destination(destination + 128)
						(source + 192).copy_nonoverlapping_64_bytes_to_aligned_destination(destination + 192)
					}
					size -= 256;
					source += 256;
					destination += 256;
					
					if size >= 128
					{
						source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination)
						(source + 64).copy_nonoverlapping_64_bytes_to_aligned_destination(destination + 64)
					}
					size -= 128;
					source += 128;
					destination += 128;
					
					if size > 64
					{
						source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination);
						
						let remainder = size - 64;
						(source + remainder).copy_nonoverlapping_64_bytes_to_any_destination((destination + remainder));
						return
					}
				}
				else
				{
					if size >= 256
					{
						source.copy_nonoverlapping_64_bytes_to_any_destination(destination)
						(source + 64).copy_nonoverlapping_64_bytes_to_any_destination(destination + 64)
						(source + 128).copy_nonoverlapping_64_bytes_to_any_destination(destination + 128)
						(source + 192).copy_nonoverlapping_64_bytes_to_any_destination(destination + 192)
					}
					size -= 256;
					source += 256;
					destination += 256;
					
					if size >= 128
					{
						source.copy_nonoverlapping_64_bytes_to_any_destination(destination)
						(source + 64).copy_nonoverlapping_64_bytes_to_any_destination(destination + 64)
					}
					size -= 128;
					source += 128;
					destination += 128;
					
					if size > 64
					{
						source.copy_nonoverlapping_64_bytes_to_any_destination(destination);
						
						let remainder = size - 64;
						(source + remainder).copy_nonoverlapping_64_bytes_to_any_destination((destination + remainder));
						return
					}
				}
				
				if size > 0
				{
					let remainder = size - 64;
					(source + remainder).copy_nonoverlapping_64_bytes_to_any_destination((destination + remainder));
				}
				
				return
			}
			
			let need_to_align_destination = !is_destination_aligned;
			if need_to_align_destination
			{
				source.copy_nonoverlapping_64_bytes_to_any_destination(destination);
				
				let increment_to_round_up_to_64_byte_alignment = (32 - (destination & 31));
				source += increment_to_round_up_to_64_alignment;
				destination += increment_to_round_up_to_64_byte_alignment;
			}
			
			while size >= 512
			{
				let zmm0 = _mm512_loadu_si512(source as *const __m512i);
				let zmm1 = _mm512_loadu_si512((source + 64) as *const __m512i);
				let zmm2 = _mm512_loadu_si512((source + 128) as *const __m512i);
				let zmm3 = _mm512_loadu_si512((source + 192) as *const __m512i);
				let zmm4 = _mm512_loadu_si512((source + 256) as *const __m512i);
				let zmm5 = _mm512_loadu_si512((source + 320) as *const __m512i);
				let zmm6 = _mm512_loadu_si512((source + 384) as *const __m512i);
				let zmm7 = _mm512_loadu_si512((source + 448) as *const __m512i);
				
				_mm512_store_si512(destination as *mut __m512i, zmm0);
				_mm512_store_si512((destination + 64) as *mut __m512i, zmm1);
				_mm512_store_si512((destination + 128) as *mut __m512i, zmm2);
				_mm512_store_si512((destination + 192) as *mut __m512i, zmm3);
				_mm512_store_si512((destination + 256) as *mut __m512i, zmm4);
				_mm512_store_si512((destination + 320) as *mut __m512i, zmm5);
				_mm512_store_si512((destination + 384) as *mut __m512i, zmm6);
				_mm512_store_si512((destination + 448) as *mut __m512i, zmm7);
				
				size -= 512;
				source += 512;
				destination += 512;
			}
			
			while size >= 128
			{
				let zmm0 = _mm512_loadu_si512(source as *const __m512i);
				let zmm1 = _mm512_loadu_si512((source + 64) as *const __m512i);
				
				_mm512_store_si512(destination as *mut __m512i, zmm0);
				_mm512_store_si512((destination + 64) as *mut __m512i, zmm1);
				
				size -= 128;
				source += 128;
				destination += 128;
			}
			
			if size > 64
			{
				source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination);
				
				let remainder = size - 64;
				(source + remainder).copy_nonoverlapping_64_bytes_to_any_destination((destination + remainder));
				return
			}
			
			if size > 0
			{
				let remainder = size - 64;
				(source + remainder).copy_nonoverlapping_64_bytes_to_any_destination((destination + remainder));
			}
		}
		else if cfg!(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx"))
		{
			let is_destination_aligned = _256_bit.is_aligned(destination);
			
			if size <= 256
			{
				if is_destination_aligned
				{
					if size >= 128
					{
						source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination);
						(source + 64).copy_nonoverlapping_64_bytes_to_aligned_destination(destination + 64);
						size -= 128;
						source += 128;
						destination += 128;
					}
					
					if size >= 64
					{
						source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination);
						size -= 64;
						source += 64;
						destination += 64;
					}
					
					if size > 32
					{
						source.copy_nonoverlapping_32_bytes_to_aligned_destination();
						
						let remainder = size - 32;
						(source + remainder).copy_nonoverlapping_32_bytes_to_any_destination((destination + remainder));
						return
					}
				}
				else
				{
					if size >= 128
					{
						source.copy_nonoverlapping_64_bytes_to_any_destination(destination);
						(source + 64).copy_nonoverlapping_64_bytes_to_any_destination(destination + 64);
						size -= 128;
						source += 128;
						destination += 128;
					}
					
					if size >= 64
					{
						source.copy_nonoverlapping_64_bytes_to_any_destination(destination);
						size -= 64;
						source += 64;
						destination += 64;
					}
					
					if size > 32
					{
						source.copy_nonoverlapping_32_bytes_to_any_destination();
						
						let remainder = size - 32;
						(source + remainder).copy_nonoverlapping_32_bytes_to_any_destination((destination + remainder));
						return
					}
				}
				
				if size > 0
				{
					let remainder = size - 32;
					(source + remainder).copy_nonoverlapping_32_bytes_to_any_destination((destination + remainder));
				}
				
				return
			}
			
			let need_to_align_destination = !is_destination_aligned;
			if need_to_align_destination
			{
				source.copy_nonoverlapping_32_bytes_to_any_destination(destination);
				
				let increment_to_round_up_to_32_byte_alignment = (32 - (destination & 31));
				source += increment_to_round_up_to_32_byte_alignment;
				destination += increment_to_round_up_to_32_byte_alignment;
			}
			
			while size >= 128
			{
				let ymm0 = _mm256_loadu_si256(source as *const __m256i);
				let ymm1 = _mm256_loadu_si256((source + 32) as *const __m256i);
				let ymm2 = _mm256_loadu_si256((source + 64) as *const __m256i);
				let ymm3 = _mm256_loadu_si256((source + 96) as *const __m256i);
				
				_mm256_store_si256(destination as *mut __m256i, ymm0);
				_mm256_store_si256((destination + 32) as *mut __m256i, ymm1);
				_mm256_store_si256((destination + 64) as *mut __m256i, ymm2);
				_mm256_store_si256((destination + 96) as *mut __m256i, ymm3);
				
				size -= 128;
				source += 128;
				destination += 128;
			}
			
			if size >= 64
			{
				source.copy_nonoverlapping_64_bytes_to_aligned_destination(destination);
				size -= 64;
				source += 64;
				destination += 64;
			}
			
			if size > 32
			{
				source.copy_nonoverlapping_32_bytes_to_aligned_destination();
				
				let remainder = size - 32;
				(source + remainder).copy_nonoverlapping_32_bytes_to_any_destination((destination + remainder));
				return
			}
			
			if size > 0
			{
				let remainder = size - 32;
				(source + remainder).copy_nonoverlapping_32_bytes_to_any_destination((destination + remainder));
			}
		}
		else
		{
			copy_nonoverlapping(source as *const u8, destination as *const u8, size)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn as_usize(self) -> usize;
	
	#[doc(hidden)]
	#[inline(always)]
	fn as_constant_pointer(self) -> *const u8;
	
	#[doc(hidden)]
	#[inline(always)]
	fn increment(self, increment: usize) -> Self;
}

impl FastMemoryCopy for *const u8
{
	type Destination = *mut u8;
	
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		self as usize
	}
	
	#[inline(always)]
	fn as_constant_pointer(self) -> *const u8
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		self
	}
	
	#[inline(always)]
	fn increment(self, increment: usize) -> Self
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		unsafe { self.increment(increment as isize) }
	}
}

impl FastMemoryCopy for NonNull<u8>
{
	type Destination = Self;
	
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.as_ptr() as usize
	}
	
	#[inline(always)]
	fn as_constant_pointer(self) -> *const u8
	{
		self.as_ptr() as *const _
	}
	
	#[inline(always)]
	fn increment(self, increment: usize) -> Self
	{
		unsafe { NonNull::new_unchecked(self.as_ptr().increment(increment as isize)) }
	}
}

impl FastMemoryCopy for usize
{
	type Destination = Self;
	
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		debug_assert_ne!(self, 0, "null");
		
		self
	}
	
	#[inline(always)]
	fn as_constant_pointer(self) -> *const u8
	{
		debug_assert_ne!(self, 0, "null");
		
		self as *const u8
	}
	
	#[inline(always)]
	fn increment(self, increment: usize) -> Self
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		self + increment
	}
}
