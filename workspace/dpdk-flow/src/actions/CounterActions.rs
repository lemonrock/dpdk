// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Applies the `COUNT` action for each entry.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct CounterActions(BTreeMap<CounterIdentifier, CounterSharing>);

impl CounterActions
{
	#[inline(always)]
	pub(crate) fn len(&self) -> usize
	{
		self.0.len()
	}
	
	#[inline(always)]
	pub(crate) fn rte_flow_actions(&self, drop_prevention: &mut Vec<Box<Any>>, actions: &mut Vec<rte_flow_action>)
	{
		for (counter_identifier, counter_sharing) in self.0.iter()
		{
			actions.push
			(
				rte_flow_action
				{
					type_: rte_flow_action_type::RTE_FLOW_ACTION_TYPE_COUNT,
					conf:
					{
						box_configuration
						(
							drop_prevention,
							Self::rte_flow_action_count(*counter_identifier, counter_sharing)
						)
					}
				}
			)
		}
	}
	
	#[inline(always)]
	pub(crate) fn rte_flow_action_count(counter_identifier: CounterIdentifier, counter_sharing: &CounterSharing) -> rte_flow_action_count
	{
		const Reserved: u32 = 0;
		
		rte_flow_action_count
		{
			bitfield_1: rte_flow_action_count::newbitfield_1(counter_sharing.to_bitfield_value(), Reserved),
			id: counter_identifier,
		}
	}
	
	#[inline(always)]
	pub(crate) fn to_hash_map(&self) -> HashMap<CounterIdentifier, CounterSharing>
	{
		let mut hash_map = HashMap::with_capacity(self.len());
		for (key, value) in self.0.iter()
		{
			hash_map.insert(*key, *value);
		}
		hash_map
	}
}
