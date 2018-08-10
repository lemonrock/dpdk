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
	
	/*
	 * Definition of a single action.
	 *
	 * A list of actions is terminated by a END action.
	 *
	 * For simple actions without a configuration structure, conf remains NULL.
	 */
	struct rte_flow_action {
enum rte_flow_action_type type; /**< Action type. */
const void *conf; /**< Pointer to action configuration structure. */
};
	
	
	
	
	int
	rte_flow_validate(uint16_t port_id,
	const struct rte_flow_attr *attr,
	const struct rte_flow_item pattern[],
	const struct rte_flow_action actions[],
struct rte_flow_error *error);
	
	int
	rte_flow_flush(uint16_t port_id,
	struct rte_flow_error *error);

int
rte_flow_query(uint16_t port_id,
struct rte_flow *flow,
	const struct rte_flow_action *action,
void *data,
struct rte_flow_error *error);

int
rte_flow_isolate(uint16_t port_id, int set, struct rte_flow_error *error);
}
