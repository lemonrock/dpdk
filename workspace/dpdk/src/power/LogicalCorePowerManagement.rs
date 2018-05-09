// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Power Management for a logical core.
///
/// When dropped, power management ends.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalCorePowerManagement
{
	logical_core: LogicalCore,
	frequencies: ArrayVec<[u32; Self::MaximumNumberOfFrequencies]>,
}

impl Drop for LogicalCorePowerManagement
{
	#[inline(always)]
	fn drop(&mut self)
	{
		assert_eq!(unsafe { rte_power_exit(self.handle()) }, 0, "rte_power_exit failed for logical core '{:?}'", self.logical_core);
	}
}

impl LogicalCorePowerManagement
{
	/// Maximum number of frequencies.
	pub const MaximumNumberOfFrequencies: usize = RTE_MAX_LCORE_FREQS;
	
	/// Starts power management for a logical core.
	#[inline(always)]
	pub fn start(logical_core: LogicalCore) -> Result<Self, ()>
	{
		assert_eq!(unsafe { rte_power_init(logical_core.into()) }, 0, "rte_power_init failed for logical core '{:?}'", logical_core);
		
		if likely(unsafe { rte_power_init(logical_core.into()) } == 0)
		{
			let mut frequencies = ArrayVec::new();
			let length = (unsafe { rte_power_get_freqs })(logical_core.into(), frequencies.as_mut_ptr(), Self::MaximumNumberOfFrequencies as u32);
			unsafe { frequencies.set_len(length as u32) }
			
			Ok
			(
				Self
				{
					logical_core,
					frequencies
				}
			)
		}
		else
		{
			Err(())
		}
	}
	
	/// Obtains supported frequencies.
	#[inline(always)]
	pub fn supported_frequencies(&self) -> &[u32]
	{
		&self.frequencies[..]
	}
	
	/// Current frequency.
	#[inline(always)]
	pub fn current_frequency(&self) -> u32
	{
		self.frequencies.get(self.current_frequency_index())
	}
	
	/// Current frequency.
	#[inline(always)]
	pub fn current_frequency_index(&self) -> usize
	{
		(unsafe { rte_power_get_freq })(self.handle()) as usize
	}
	
	/// Increase frequency one step.
	#[inline(always)]
	pub fn increase_frequency(&self) -> bool
	{
		match (unsafe { rte_power_freq_up })(self.handle())
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_freq_up failed")
		}
	}
	
	/// Decrease frequency one step.
	#[inline(always)]
	pub fn decrease_frequency(&self) -> bool
	{
		match (unsafe { rte_power_freq_down })(self.handle())
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_freq_down failed")
		}
	}
	
	/// Set to minimum frequency.
	#[inline(always)]
	pub fn set_to_minimum_frequency(&self) -> bool
	{
		match (unsafe { rte_power_freq_min })(self.handle())
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_freq_min failed")
		}
	}
	
	/// Set to maximum frequency.
	#[inline(always)]
	pub fn set_to_maximum_frequency(&self) -> bool
	{
		match (unsafe { rte_power_freq_max })(self.handle())
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_freq_max failed")
		}
	}
	
	/// Set frequency.
	#[inline(always)]
	pub fn set_to_frequency_at_index(&self, index: usize) -> bool
	{
		debug_assert!(index < self.frequencies.len(), "index '{}' exceeds number of frequencies '{}", index, self.frequencies.len());
		
		match (unsafe { rte_power_set_freq })(self.handle(), index as u32)
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_set_freq failed")
		}
	}
	
	/// Enable turbo boost.
	#[inline(always)]
	pub fn enable_turbo_boost(&self) -> bool
	{
		match (unsafe { rte_power_freq_enable_turbo })(self.handle())
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_freq_enable_turbo failed")
		}
	}
	
	/// Disable turbo boost.
	#[inline(always)]
	pub fn disable_turbo_boost(&self) -> bool
	{
		match (unsafe { rte_power_freq_disable_turbo })(self.handle())
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_freq_disable_turbo failed")
		}
	}
	
	/// Is turbo boost enabled?
	#[inline(always)]
	pub fn is_turbo_boost_enabled(&self) -> bool
	{
		match (unsafe { rte_power_turbo_status })(self.handle())
		{
			1 => true,
			0 => false,
			_ => panic!("rte_power_turbo_status failed")
		}
	}
	
	#[inline(always)]
	fn handle(&self) -> u32
	{
		self.logical_core.into()
	}
}
