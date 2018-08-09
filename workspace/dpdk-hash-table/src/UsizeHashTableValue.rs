// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait that values that fit in a HashTable need to implement.
///
/// Implementations are also possible for `Option<u16>` and `Option<u8>`, various tuples and the `::std::num::NonZero*` types, although these will need newtype wrappers.
pub trait UsizeHashTableValue: Sized
{
	/// Converts into a HashTable data-sized type.
	#[inline(always)]
	fn into_usize(self) -> usize;
	
	/// Converts from a HashTable a data-sized type.
	#[inline(always)]
	fn from_usize(value: usize) -> Self;
}

impl<T> UsizeHashTableValue for Box<T>
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		Box::into_raw(self) as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { Box::from_raw(value as *mut T) }
	}
}

impl<T> UsizeHashTableValue for Rc<T>
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		Rc::into_raw(self) as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { Rc::from_raw(value as *mut T) }
	}
}

impl<T> UsizeHashTableValue for Arc<T>
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		Arc::into_raw(self) as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { Arc::from_raw(value as *mut T) }
	}
}

impl<T> UsizeHashTableValue for *const T
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as *const T
	}
}

impl<T> UsizeHashTableValue for *mut T
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as *mut T
	}
}

impl<T> UsizeHashTableValue for NonNull<T>
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self.as_ptr() as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { NonNull::new_unchecked(value as *mut T) }
	}
}

impl<T> UsizeHashTableValue for Option<NonNull<T>>
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		match self
		{
			None => 0,
			Some(value) => value.as_ptr() as usize
		}
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		NonNull::new(value as *mut T)
	}
}

impl UsizeHashTableValue for usize
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value
	}
}

impl UsizeHashTableValue for isize
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as isize
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for u64
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as u64
	}
}

impl UsizeHashTableValue for u32
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as u32
	}
}

impl UsizeHashTableValue for u16
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as u16
	}
}

impl UsizeHashTableValue for u8
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as u8
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for i64
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as i64
	}
}

impl UsizeHashTableValue for i32
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as i32
	}
}

impl UsizeHashTableValue for i16
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as i16
	}
}

impl UsizeHashTableValue for i8
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as i8
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for f64
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self.to_bits() as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		f64::from_bits(value as u64)
	}
}

impl UsizeHashTableValue for f32
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self.to_bits() as usize
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		f32::from_bits(value as u32)
	}
}

#[repr(C)]
union UsizeHashTableValueConversion
{
	usize: usize,
	
	#[cfg(target_pointer_width = "64")]
	u64_1: [u64; 1],
	
	#[cfg(target_pointer_width = "64")]
	i64_1: [i64; 1],
	
	#[cfg(target_pointer_width = "64")]
	f64_1: [f64; 1],
	
	#[cfg(target_pointer_width = "64")]
	u32_2: [u32; 2],
	
	u32_1: [u32; 1],
	
	#[cfg(target_pointer_width = "64")]
	i32_2: [i32; 2],
	
	i32_1: [i32; 1],
	
	#[cfg(target_pointer_width = "64")]
	f32_2: [f32; 2],
	
	f32_1: [f32; 1],
	
	#[cfg(target_pointer_width = "64")]
	u16_4: [u16; 4],
	
	#[cfg(target_pointer_width = "64")]
	u16_3: [u16; 3],
	
	u16_2: [u16; 2],
	
	u16_1: [u16; 1],
	
	#[cfg(target_pointer_width = "64")]
	i16_4: [i16; 4],
	
	#[cfg(target_pointer_width = "64")]
	i16_3: [i16; 3],
	
	i16_2: [i16; 2],
	
	i16_1: [i16; 1],
	
	#[cfg(target_pointer_width = "64")]
	u8_8: [u8; 8],
	
	#[cfg(target_pointer_width = "64")]
	u8_7: [u8; 7],
	
	#[cfg(target_pointer_width = "64")]
	u8_6: [u8; 6],
	
	#[cfg(target_pointer_width = "64")]
	u8_5: [u8; 5],
	
	u8_4: [u8; 4],
	
	u8_3: [u8; 3],
	
	u8_2: [u8; 2],
	
	u8_1: [u8; 1],
	
	#[cfg(target_pointer_width = "64")]
	i8_8: [i8; 8],
	
	#[cfg(target_pointer_width = "64")]
	i8_7: [i8; 7],
	
	#[cfg(target_pointer_width = "64")]
	i8_6: [i8; 6],
	
	#[cfg(target_pointer_width = "64")]
	i8_5: [i8; 5],
	
	i8_4: [i8; 4],
	
	i8_3: [i8; 3],
	
	i8_2: [i8; 2],
	
	i8_1: [i8; 1],
	
	bool: bool,
}

impl UsizeHashTableValueConversion
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		unsafe { self.usize }
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		UsizeHashTableValueConversion
		{
			usize: value,
		}
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u64; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u64_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u64_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i64; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i64_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i64_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [f64; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { f64_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).f64_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u32; 2]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u32_2: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u32_2 }
	}
}

impl UsizeHashTableValue for [u32; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u32_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u32_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i32; 2]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i32_2: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i32_2 }
	}
}

impl UsizeHashTableValue for [i32; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i32_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i32_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [f32; 2]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { f32_2: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).f32_2 }
	}
}

impl UsizeHashTableValue for [f32; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { f32_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).f32_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u16; 4]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u16_4: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u16_4 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u16; 3]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u16_3: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u16_3 }
	}
}

impl UsizeHashTableValue for [u16; 2]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u16_2: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u16_2 }
	}
}

impl UsizeHashTableValue for [u16; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u16_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u16_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i16; 4]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i16_4: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i16_4 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i16; 3]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i16_3: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i16_3 }
	}
}

impl UsizeHashTableValue for [i16; 2]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i16_2: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i16_2 }
	}
}

impl UsizeHashTableValue for [i16; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i16_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i16_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u8; 8]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_8: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_8 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u8; 7]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_7: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_7 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u8; 6]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_6: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_6 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [u8; 5]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_5: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_5 }
	}
}

impl UsizeHashTableValue for [u8; 4]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_4: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_4 }
	}
}

impl UsizeHashTableValue for [u8; 3]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_3: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_3 }
	}
}

impl UsizeHashTableValue for [u8; 2]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_2: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_2 }
	}
}

impl UsizeHashTableValue for [u8; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { u8_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).u8_1 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i8; 8]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_8: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_8 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i8; 7]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_7: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_7 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i8; 6]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_6: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_6 }
	}
}

#[cfg(target_pointer_width = "64")]
impl UsizeHashTableValue for [i8; 5]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_5: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_5 }
	}
}

impl UsizeHashTableValue for [i8; 4]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_4: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_4 }
	}
}

impl UsizeHashTableValue for [i8; 3]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_3: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_3 }
	}
}

impl UsizeHashTableValue for [i8; 2]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_2: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_2 }
	}
}

impl UsizeHashTableValue for [i8; 1]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { i8_1: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).i8_1 }
	}
}

impl UsizeHashTableValue for ()
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		0
	}
	
	#[inline(always)]
	fn from_usize(_value: usize) -> Self
	{
		()
	}
}

impl UsizeHashTableValue for bool
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		UsizeHashTableValueConversion { bool: self }.into_usize()
	}
	
	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { UsizeHashTableValueConversion::from_usize(value).bool }
	}
}

impl<T> UsizeHashTableValue for [T; 0]
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		0
	}
	
	#[inline(always)]
	fn from_usize(_value: usize) -> Self
	{
		[]
	}
}
