// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A busy-poll implementation
pub trait BusyPollBehaviour: Sized
{
	/// Executed once before the busy-poll loop begins.
	///
	/// Will be executed inside the logical core, thread and NUMA node that will always run `execute()` and `finish()`: useful for setting up initial thread or core-local state.
	#[inline(always)]
	fn start(&mut self);
	
	/// Executed repeatedly, once for each execution of the busy poll loop.
	///
	/// Will be executed inside the logical core, thread and NUMA node that will always run `start()` and `finish()`.
	#[inline(always)]
	fn execute(&mut self);
	
	/// Executed once after the busy-poll loop finished, but not if a panic was raised.
	///
	/// Will be executed inside the logical core, thread and NUMA node that will always run `start()` and `execute()`.
	#[inline(always)]
	fn finish(&mut self);
	
	#[doc(hidden)]
	#[inline(always)]
	fn execute_code_on_slave(self, slave_logical_core: LogicalCore, should_function_terminate: &Arc<ShouldFunctionTerminate>)
	{
		let function_to_execute_on_slave = Box::new(BusyPollingLogicalCoreFunction::new(self, should_function_terminate));
		slave_logical_core.execute_code_on_slave(function_to_execute_on_slave).expect("Should not be busy at this time")
	}
}
