// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerManager
{
	timerResolutionCycles: u64,
	previousTsc: u64,
}

impl TimerManager
{
	pub const AroundTenMillisecondsAt2GigaHertz: u64 = 20000000;
	
	#[inline(always)]
	pub fn initialiseSubsystem()
	{
		unsafe { ::dpdk_sys::rte_timer_subsystem_init() }
	}
	
	/// Calling the underlying timer check code in runAllExpiredTimers() is quite expensive, so use a large value of cycles
	/// The default() checks every 10ms on a 2GHz CPU
	#[inline(always)]
	pub fn new(timerResolutionCycles: u64) -> TimerManager
	{
		TimerManager
		{
			timerResolutionCycles: timerResolutionCycles,
			previousTsc: getRdtsc(),
		}
	}
	
	#[inline(always)]
	pub fn runAllExpiredTimers(&mut self)
	{
		let currentTsc = getRdtsc();
		if currentTsc - self.previousTsc > self.timerResolutionCycles
		{
			unsafe { ::dpdk_sys::rte_timer_manage() };
			self.previousTsc = currentTsc;
		}
	}
	
	#[inline(always)]
	pub fn dumpStatisticsToStandardError()
	{
		unsafe { ::dpdk_sys::rte_timer_dump_stats(stderr as *mut FILE) }
	}
}

impl Default for TimerManager
{
	fn default() -> Self
	{
		TimerManager::new(Self::AroundTenMillisecondsAt2GigaHertz)
	}
}
