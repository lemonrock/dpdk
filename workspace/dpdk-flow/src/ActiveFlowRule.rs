// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A flow rule which is active, ie in use.
///
/// When dropped, the flow rule will no longer be active.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ActiveFlowRule
{
	port_identifier: u16,
	reference: NonNull<rte_flow>,
}

impl Drop for ActiveFlowRule
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rte_flow_destroy(self.port_identifier, self.reference.as_ptr(), null_mut()) };
	}
}

impl ActiveFlowRule
{
	/// Tries to drop this rule; returns an error if it does so.
	#[inline(always)]
	pub fn try_to_drop(self) -> Result<(), (Self, (i32, rte_flow_error))>
	{
		let mut error = unsafe { zeroed() };
		let result = unsafe { rte_flow_destroy(self.port_identifier, self.reference.as_ptr(), &mut error) };
		
		if likely!(result == 0)
		{
			forget(self);
			Ok(())
		}
		else
		{
			Err((self, (LogicalCore::current_logical_core_error_number(), error)))
		}
	}
}
