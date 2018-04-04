// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::*;
use ::libc::timespec;


#[inline(always)]
pub fn getRdtsc() -> u64
{
	unsafe { rust_rte_rdtsc() }
}

#[inline(always)]
pub fn getRdtscPreciselyButSlightlySlowly() -> u64
{
	unsafe { rust_rte_rdtsc_precise() }
}

#[inline(always)]
pub fn getTheMeasuredFrequencyOfTheRdtscCounter() -> u64
{
	unsafe { ::dpdk_sys::rte_get_tsc_hz() }
}

#[inline(always)]
pub fn numberOfTscCyclesSinceBoot() -> u64
{
	unsafe { rust_rte_get_tsc_cycles() }
}

#[inline(always)]
pub fn numberOfCyclesSinceBootForTheDefaultTimer() -> u64
{
	unsafe { rust_rte_get_timer_cycles() }
}

#[inline(always)]
pub fn numberOfCyclesInOneSecondForTheDefaultTimer() -> u64
{
	unsafe { rust_rte_get_timer_hz() }
}

#[inline(always)]
pub fn blockAtLeastMicroseconds(microseconds: u32)
{
	unsafe { ::dpdk_sys::rte_delay_us_block(microseconds) }
}

#[inline(always)]
pub fn waitAtLeastMilliseconds(milliseconds: u32)
{
	unsafe { rust_rte_delay_ms(milliseconds) }
}

#[inline(always)]
pub fn timespecToNanoseconds(timespec: &timespec) -> u64
{
	unsafe { rust_rte_timespec_to_ns(timespec) }
}

#[inline(always)]
pub fn nanosecondsToTimespec(nanoseconds: u64) -> timespec
{
	unsafe { rust_rte_ns_to_timespec(nanoseconds) }
}
