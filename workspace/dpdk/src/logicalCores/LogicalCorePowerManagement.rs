// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const MaximumNumberOfFrequencies: usize = RTE_MAX_LCORE_FREQS;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalCorePowerManagement
{
	logicalCore: LogicalCore,
	frequencies: Option<ArrayVec<[u32; MaximumNumberOfFrequencies]>>,
	numberOfFrequencies: u8,
}

impl Drop for LogicalCorePowerManagement
{
	#[inline(always)]
	fn drop(&mut self)
	{
		match unsafe { ::dpdk_sys::rte_power_exit(self.logicalCoreAsU32()) }
		{
			0 => (),
			
			x if x.is_negative() => (),
			
			illegal @ _ => panic!("rte_power_exit() returned an invalid positive return code of '{}'", illegal),
		}
	}
}

impl LogicalCorePowerManagement
{
	#[inline(always)]
	pub fn initialise(logicalCore: LogicalCore) -> Option<LogicalCorePowerManagement>
	{
		debug_assert!(!logicalCore.isAny(), "logicalCore can not be any");
				
		let logicalCoreAsU32 = logicalCore.as_u32();
		match unsafe { ::dpdk_sys::rte_power_init(logicalCoreAsU32) }
		{
			0 =>
			{
				let frequencies = Self::getFrequencies(logicalCoreAsU32);

				let length = if let Some(ref frequencies) = frequencies
				{
					frequencies.len() as u8
				}
				else
				{
					0
				};
				
				Some
				(
					LogicalCorePowerManagement
					{
						logicalCore: logicalCore,
						frequencies: frequencies,
						numberOfFrequencies: length,
					}
				)
			},
			
			negative if negative < 0 => None,
			
			illegal @ _ => panic!("Function rte_power_init() returned an unexpected result '{}'", illegal),
		}
	}
	
	#[inline(always)]
	pub fn numberOfFrequencies(&self) -> u8
	{
		self.numberOfFrequencies
	}
	
	#[inline(always)]
	pub fn getFrequency(&self, index: u8) -> Option<u32>
	{
		if let Some(ref frequencies) = self.frequencies
		{
			frequencies.get(index as usize).map(|value| {*value})
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn getCurrentFrequencyIndex(&self) -> Option<u8>
	{
		if let Some(functionPointer) = unsafe { ::dpdk_sys::rte_power_get_freq }
		{
			match functionPointer(self.logicalCoreAsU32())
			{
				0xFFFFFFFF => None,
				
				index if index <= self.numberOfFrequencies as u32 => Some(index as u8),
				
				index @ _ => panic!("rte_power_get_freq() returned an index '{}' which equals or exceeds numberOfFrequencies '{}'", index, self.numberOfFrequencies),
			}
		}
		else
		{
			None
		}
	}
		
	#[inline(always)]
	pub fn setCurrentFrequencyIndex(&self, index: u8) -> Option<bool>
	{
		debug_assert!(index < self.numberOfFrequencies, "index '{}' equals or exceeds numberOfFrequencies '{}'", index, self.numberOfFrequencies);
		
		if let Some(functionPointer) = unsafe { ::dpdk_sys::rte_power_set_freq }
		{
			Self::matchResult("rte_power_set_freq", functionPointer(self.logicalCoreAsU32(), index as u32))
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn increaseFrequency(&self) -> Option<bool>
	{
		self.executeFrequencyChangeFunctionPointer("rte_power_freq_up", unsafe { ::dpdk_sys::rte_power_freq_up })
	}
	
	#[inline(always)]
	pub fn decreaseFrequency(&self) -> Option<bool>
	{
		self.executeFrequencyChangeFunctionPointer("rte_power_freq_down", unsafe { ::dpdk_sys::rte_power_freq_down })
	}
	
	#[inline(always)]
	pub fn setFrequencyToMaximum(&self) -> Option<bool>
	{
		self.executeFrequencyChangeFunctionPointer("rte_power_freq_max", unsafe { ::dpdk_sys::rte_power_freq_max })
	}
	
	#[inline(always)]
	pub fn setFrequencyToMinimum(&self) -> Option<bool>
	{
		self.executeFrequencyChangeFunctionPointer("rte_power_freq_min", unsafe { ::dpdk_sys::rte_power_freq_min })
	}
	
	#[inline(always)]
	fn executeFrequencyChangeFunctionPointer(&self, functionName: &str, functionPointer: rte_power_freq_change_t) -> Option<bool>
	{
		if let Some(functionPointer) = functionPointer
		{
			Self::matchResult(functionName, functionPointer(self.logicalCoreAsU32()))
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn matchResult(functionName: &str, result: i32) -> Option<bool>
	{
		match result
		{
			1 => Some(true),
			
			0 => Some(false),
			
			negative if negative < 0 => None,
			
			illegal @ _ => panic!("Function {}() returned an unexpected result '{}'", functionName, illegal),
		}
	}
	
	#[inline(always)]
	fn getFrequencies(logicalCoreAsU32: u32) -> Option<ArrayVec<[u32; MaximumNumberOfFrequencies]>>
	{
		if let Some(functionPointer) = unsafe { ::dpdk_sys::rte_power_freqs }
		{
			let mut frequencies: ArrayVec<[u32; MaximumNumberOfFrequencies]> = ArrayVec::new();
			let actualNumber = (unsafe { functionPointer(logicalCoreAsU32, frequencies.as_mut_ptr(), MaximumNumberOfFrequencies as u32) }) as usize;
			if actualNumber < MaximumNumberOfFrequencies
			{
				unsafe { frequencies.set_len(actualNumber) };
				Some(frequencies)
			}
			else
			{
				panic!("rte_power_freqs() returned '{}', which equals or exceeds MaximumNumberOfFrequencies '{}'", actualNumber, MaximumNumberOfFrequencies);
			}
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn logicalCoreAsU32(&self) -> u32
	{
		self.logicalCore.as_u32()
	}
}
