// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A wrapper type for logical core functions which:-
///
/// * catches panics and causes termination.
/// * busy polls (loops) with a CPU pause.
///
/// It re-uses the definition of `ServiceFunction` for the body of the busy poll loop.
#[derive(Debug)]
pub struct BusyPollingLogicalCoreFunction<B: BusyPollBehaviour>
{
	busy_poll_behaviour: B,
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

impl<B: BusyPollBehaviour> SlaveLogicalCoreFunction for BusyPollingLogicalCoreFunction<B>
{
	#[inline(always)]
	fn execute(&mut self)
	{
		let success_or_failure = catch_unwind(AssertUnwindSafe(||
		{
			self.busy_poll_behaviour.start();
			
			while self.should_function_terminate.should_continue()
			{
				self.busy_poll_behaviour.execute();
				
				spin_loop_hint()
			}
			
			self.busy_poll_behaviour.finish();
		}));
		
		if let Err(panicked_with) = success_or_failure
		{
			self.should_function_terminate.we_panicked(panicked_with.as_ref())
		}
	}
}

impl<B: BusyPollBehaviour> BusyPollingLogicalCoreFunction<B>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(busy_poll_behaviour: B, should_function_terminate: &Arc<ShouldFunctionTerminate>) -> Self
	{
		Self
		{
			busy_poll_behaviour,
			should_function_terminate: should_function_terminate.clone(),
		}
	}
}
