// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalCoresActive(LogicalCoresActiveArray<bool>, u8);

impl Default for LogicalCoresActive
{
	fn default() -> Self
	{
		let mut value = [false; Maximum];
		value[0] = true;
		LogicalCoresActive(LogicalCoresActiveArray(value), 1)
	}
}

impl Serialize for LogicalCoresActive
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let values = self.asLinuxString();
		serializer.serialize_str(&values)
	}
}

impl Deserialize for LogicalCoresActive
{
	fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error>
	{
		struct FromString;
		
		impl Visitor for FromString
		{
			type Value = LogicalCoresActive;
			
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				formatter.write_str("A string in Linux cpu set format")
			}
			
			fn visit_str<E: ::serde::de::Error>(self, value: &str) -> Result<Self::Value, E>
			{
				match Self::Value::parse(value)
				{
					Ok(value) => Ok(value),
					Err(_) => Err(E::custom("Could not parse string")),
				}
			}
		}
		
		deserializer.deserialize(FromString)
	}
}

impl Active for LogicalCoresActive
{
	type T = LogicalCore;
	
	const Maximum: usize = Maximum;
	
	#[inline(always)]
	fn constructor(index: usize) -> Self::T
	{
		LogicalCore(index as u32)
	}
	
	#[inline(always)]
	fn count(&self) -> usize
	{
		self.1 as usize
	}
	
	#[inline(always)]
	fn none() -> Self
	{
		LogicalCoresActive(LogicalCoresActiveArray([false; Maximum]), 0)
	}
	
	#[inline(always)]
	fn all() -> Self
	{
		LogicalCoresActive(LogicalCoresActiveArray([true; Maximum]), Maximum as u8)
	}
	
	#[inline(always)]
	fn value(&self, index: usize) -> bool
	{
		debug_assert!(index < Self::Maximum, "index '{}' is not less than Maximum '{}'", index, Self::Maximum);
		
		(self.0).0[index]
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
		
		(self.0).0[index] = toValue;
	}
}

impl LogicalCoresActive
{
	pub fn as_hexadecimal_core_mask_c_string(&self) -> CString
	{
		let mut setBits = 0;
		for index in 0..Maximum
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
