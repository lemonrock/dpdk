// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumaSocketsActive([bool; NumaNode::MaximumNumaSockets], u8);

impl Default for NumaSocketsActive
{
	fn default() -> Self
	{
		let mut value = [false; NumaNode::MaximumNumaSockets];
		value[0] = true;
		return NumaSocketsActive(value, 1);
	}
}

impl Active for NumaSocketsActive
{
	type T = NumaSocketId;
	
	const Maximum: usize = NumaNode::MaximumNumaSockets;
	
	#[inline(always)]
	fn constructor(index: usize) -> Self::T
	{
		NumaSocketId::fromU32(index as u32).unwrap()
	}
	
	#[inline(always)]
	fn count(&self) -> usize
	{
		self.1 as usize
	}
	
	#[inline(always)]
	fn none() -> Self
	{
		NumaSocketsActive([false; NumaNode::MaximumNumaSockets], 0)
	}
	
	#[inline(always)]
	fn all() -> Self
	{
		NumaSocketsActive([true; NumaNode::MaximumNumaSockets], NumaNode::MaximumNumaSockets as u8)
	}
	
	#[inline(always)]
	fn value(&self, index: usize) -> bool
	{
		debug_assert!(index < Self::Maximum, "index '{}' is not less than Maximum '{}'", index, Self::Maximum);
		
		(self.0)[index]
	}
	
	#[inline(always)]
	fn set(&mut self, index: usize, toValue: bool)
	{
		debug_assert!(index < Self::Maximum, "index '{}' is not less than Maximum '{}'", index, Self::Maximum);
		
		if toValue
		{
			self.1 += 1
		}
		else
		{
			self.1 -= 1
		}
		
		(self.0)[index] = toValue;
	}
}


impl NumaSocketsActive
{
	pub fn as_hexadecimal_core_mask_c_string(&self) -> CString
	{
		let mut setBits = 0;
		for index in 0..NumaNode::MaximumNumaSockets
		{
			if self.isEnabled(index)
			{
				setBits |= 1 << index
			}
		}
		
		debug_assert!(Self::Maximum <= 256 && Self::Maximum >= 16, "Change format string size parameter from 2 to something else, as Maximum '{}' is outside of the range expected", Self::Maximum);
		
		CString::new(format!("0x{:02}", setBits)).unwrap()
	}
}
