// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Should the function running on the current logical core terminate?
pub struct ShouldFunctionTerminate(AtomicBool);

unsafe impl Send for ShouldFunctionTerminate
{
}

unsafe impl Sync for ShouldFunctionTerminate
{
}

impl ShouldFunctionTerminate
{
	const Sleepiness: Duration = Duration::from_millis(10);
	
	#[inline(always)]
	pub fn should_terminate(&self) -> bool
	{
		self.0.load(Ordering::Relaxed)
	}
	
	#[inline(always)]
	pub fn sleep_and_check_should_terminate(&self) -> bool
	{
		sleep(Self::Sleepiness);
		self.should_terminate()
	}
}
