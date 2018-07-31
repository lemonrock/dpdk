// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Hertz (Hz).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Hertz(u64);

impl From<u64> for Hertz
{
	#[inline(always)]
	fn from(frequency: u64) -> Self
	{
		Hertz(frequency)
	}
}

impl Into<u64> for Hertz
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Hertz
{
	/// Number of cycles in one second (measured frequency) in hertz for the HPET timer.
	///
	/// Preferred to TSC.
	#[inline(always)]
	pub fn number_of_cycles_in_one_second_for_the_hpet_timer() -> Self
	{
		Hertz(unsafe { rte_get_hpet_cycles() })
	}
	
	/// Number of cycles in one second (measured frequency) in hertz for the TSC timer.
	#[inline(always)]
	pub fn number_of_cycles_in_one_second_for_the_tsc_timer() -> Self
	{
		Hertz(unsafe { rte_get_tsc_hz() })
	}
	
	/// Number of cycles in one second (measured frequency) in hertz for the default timer.
	#[inline(always)]
	pub fn number_of_cycles_in_one_second_for_the_default_timer() -> Self
	{
		Hertz(unsafe { rust_rte_get_timer_hz() })
	}
}
