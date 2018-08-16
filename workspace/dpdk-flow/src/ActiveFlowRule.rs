// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A flow rule which is active, ie in use.
///
/// When dropped, the flow rule will no longer be active.
#[derive(Debug)]
pub struct ActiveFlowRule
{
	ethernet_port_identifier: EthernetPortIdentifier,
	counter_actions: HashMap<CounterIdentifier, CounterSharing>,
	reference: NonNull<rte_flow>,
}

impl Drop for ActiveFlowRule
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rte_flow_destroy(self.ethernet_port_identifier(), self.as_ptr(), null_mut()) };
	}
}

impl ActiveFlowRule
{
	/// Tries to drop this rule; returns an error if it does so.
	#[inline(always)]
	pub fn try_to_drop(self) -> Result<(), (Self, (i32, rte_flow_error))>
	{
		let mut error = unsafe { zeroed() };
		let result = unsafe { rte_flow_destroy(self.ethernet_port_identifier(), self.as_ptr(), &mut error) };
		
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
	
	/// As of DPDK 18.05, only Mellanox supports querying counters.
	///
	/// Returns Err(0) if no counter identifier found.
	#[inline(always)]
	pub fn query_counters(&self, counter_identifier: CounterIdentifier, reset_counters_after_retrieving_current_values: bool) -> Result<rte_flow_query_count, u32>
	{
		let rte_flow_action_count = CounterActions::rte_flow_action_count
		(
			counter_identifier,
			match self.counter_actions.get(&counter_identifier)
			{
				None => return Err(0),
				Some(counter_sharing) => counter_sharing,
			}
		);
		
		let action = rte_flow_action
		{
			type_: rte_flow_action_type::RTE_FLOW_ACTION_TYPE_COUNT,
			conf: &rte_flow_action_count as *const rte_flow_action_count as *const c_void,
		};
		
		let mut data: rte_flow_query_count = unsafe { zeroed() };
		if reset_counters_after_retrieving_current_values
		{
			data.set_reset(1);
		}
		
		let result = unsafe { rte_flow_query(self.ethernet_port_identifier(), self.as_ptr(), &action, &mut data as *mut rte_flow_query_count as *mut c_void, null_mut()) };
		if likely!(result == 0)
		{
			Ok(data)
		}
		else if likely!(result < 0)
		{
			Err((-result) as u32)
		}
		else
		{
			panic!("Invalid return code '{}'", result);
		}
	}
	
	#[inline(always)]
	fn ethernet_port_identifier(&self) -> u16
	{
		self.ethernet_port_identifier.into()
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut rte_flow
	{
		self.reference.as_ptr()
	}
}
