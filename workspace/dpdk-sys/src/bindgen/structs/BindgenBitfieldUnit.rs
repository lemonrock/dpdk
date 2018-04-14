// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindgenBitfieldUnit<Storage, Align>
where Storage: AsRef<[u8]> + AsMut<[u8]>
{
	storage: Storage,
	align: [Align; 0],
}

impl<Storage, Align> BindgenBitfieldUnit<Storage, Align>
where Storage: AsRef<[u8]> + AsMut<[u8]>
{
	
	#[inline(always)]
	pub fn new(storage: Storage) -> Self
	{
		Self {
			storage,
			align: [],
		}
	}
	
	#[inline(always)]
	pub fn get_bit(&self, index: usize) -> bool
	{
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = self.storage.as_ref()[byte_index];
		let bit_index = index % 8;
		let mask = 1 << bit_index;
		byte & mask == mask
	}
	
	#[inline(always)]
	pub fn set_bit(&mut self, index: usize, val: bool)
	{
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = &mut self.storage.as_mut()[byte_index];
		let bit_index = index % 8;
		let mask = 1 << bit_index;
		if val
		{
			*byte |= mask;
		}
		else
		{
			*byte &= !mask;
		}
	}
	
	#[inline(always)]
	pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64
	{
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
		let mut val = 0;
		for i in 0 .. (bit_width as usize)
		{
			if self.get_bit(i + bit_offset)
			{
				val |= 1 << i;
			}
		}
		val
	}
	
	#[inline(always)]
	pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64)
	{
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
		for i in 0 .. (bit_width as usize)
		{
			let mask = 1 << i;
			let val_bit_is_set = val & mask == mask;
			self.set_bit(i + bit_offset, val_bit_is_set);
		}
	}
}
