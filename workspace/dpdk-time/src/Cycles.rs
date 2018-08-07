// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Cycles.
///
/// Multiply by `Hertz` to get a time duration in seconds.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct Cycles(u64);

impl From<u64> for Cycles
{
	#[inline(always)]
	fn from(cycles: u64) -> Self
	{
		Cycles(cycles)
	}
}

impl Into<u64> for Cycles
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Default for Cycles
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::AroundTenMillisecondsAt2GigaHertzSuitableForATimerProgressEngine
	}
}

impl Sub for Cycles
{
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output
	{
		Cycles(self.0 - rhs.0)
	}
}

impl Cycles
{
	/// A sensible amount to use with a TimerProgressEngine.
	pub const AroundTenMillisecondsAt2GigaHertzSuitableForATimerProgressEngine: Cycles = Cycles(20_000_000);
	
	/// Current HPET cycles since boot.
	///
	/// Preferred to RDTSC / TSC.
	#[inline(always)]
	pub fn current_hpet_cycles_since_boot() -> Self
	{
		Cycles(unsafe { rte_get_hpet_cycles() })
	}
	
	/// Current RDTSC cycles since boot.
	#[inline(always)]
	pub fn current_rdtsc_cycles_since_boot() -> Cycles
	{
		Cycles(unsafe { rust_rte_rdtsc() })
	}
	
	/// Current RDTSC cycles sicne boot more precisely but slightly more slowly than `current_rdtsc`.
	///
	/// Uses a memory barrier internally.
	#[inline(always)]
	pub fn current_rdtsc_cycles_since_boot_precisely_but_slightly_slowly() -> Cycles
	{
		Cycles(unsafe { rust_rte_rdtsc_precise() })
	}
	
	/// Current TSC cycles since boot.
	///
	/// Same as `current_rdtsc_since_boot` inside DPDK at the moment (last checked for DPDK v18.02).
	#[inline(always)]
	pub fn current_tsc_cycles_since_boot() -> Self
	{
		Cycles(unsafe { rust_rte_get_tsc_cycles() })
	}
	
	/// Current TSC cycles since boot for the default timer.
	///
	/// Same as `current_rdtsc_since_boot` inside DPDK at the moment (last checked for DPDK v18.02).
	#[inline(always)]
	pub fn current_tsc_cycles_since_boot_for_the_default_timer() -> Self
	{
		Cycles(unsafe { rust_rte_get_timer_cycles() })
	}
}
