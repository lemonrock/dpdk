// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A timer progress engine.
///
/// Only one is needed.
///
/// Not thread safe.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerProgressEngine
{
	period: Cycles,
	previous_rdtsc_cycles_since_boot: Cycles,
}

impl PrintAllInformation for TimerProgressEngine
{
	#[inline(always)]
	fn print_information_to_stream(stream: *mut FILE)
	{
		unsafe { rte_timer_dump_stats(stream) };
	}
}

impl TimerProgressEngine
{
	/// Calling the underlying timer check code in progress() is quite expensive, so use a large value of cycles, eg `Cycles::AroundTenMillisecondsAt2GigaHertzSuitableForATimerProgressEngine`.
	#[inline(always)]
	pub fn new(period: Cycles) -> Self
	{
		Self
		{
			period,
			previous_rdtsc_cycles_since_boot: Cycles::current_rdtsc_cycles_since_boot(),
		}
	}
	
	/// This causes all expired timers to fire.
	///
	/// This is expensive, so we only actually call the underlying logic if more than `timer_resolution_cycles` has elapsed.
	#[inline(always)]
	pub fn progress(&mut self)
	{
		let current_rdtsc_cycles_since_boot = Cycles::current_rdtsc_cycles_since_boot();
		if current_rdtsc_cycles_since_boot - self.previous_rdtsc_cycles_since_boot > self.period
		{
			unsafe { rte_timer_manage() };
			self.previous_rdtsc_cycles_since_boot = current_rdtsc_cycles_since_boot;
		}
	}
}
