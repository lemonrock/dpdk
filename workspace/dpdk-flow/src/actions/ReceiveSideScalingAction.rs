// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive side scaling (RSS) action.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ReceiveSideScalingAction
{
	/// Hash function algorithm to use.
	#[serde(default)]
	pub hash_function_algorithm: ReceiveSideScalingHashFunctionAlgorithm,
	
	/// What kind of receive-side scaling flow types to use?
	///
	/// Defaults to `ReceiveSideScalingOffloadFlow::default()`, which is currently `ReceiveSideScalingOffloadFlow::common_flags()` (`InternetProtocol`, `UserDatagramProtocol` and `TransmissionControlProtocol` including fragmented packets and extended header matches).
	#[serde(default)]
	pub types: ReceiveSideScalingOffloadFlow,
	
	/// How to calculate RSS hash.
	///
	/// Defaults to a value always supported.
	#[serde(default)]
	pub level: ReceiveSideScalingLevel,
	
	/// Hash function key strategy.
	#[serde(default)]
	pub hash_key_strategy: HashFunctionKeyDataStrategy,
	
	/// Must have at least one entry.
	///
	/// Limited in practice to the maximum number of supported queues, which may be as few as 2.
	///
	/// Defaults to 2 queues for zero-based indices 1 and 2 (implying the network card driver must support at least 3 queues).
	#[serde(default = "ReceiveSideScalingAction::receive_queue_indices_to_distribute_to_default")]
	pub receive_queue_indices_to_distribute_to: ArrayVec<[ReceiveQueueIdentifier; RTE_MAX_QUEUES_PER_PORT]>,
}

impl ReceiveSideScalingAction
{
	#[inline(always)]
	pub(crate) fn rte_flow_action(&self, ethernet_device_capabilities: &EthernetDeviceCapabilities, drop_prevention: &mut Vec<Box<Any>>) -> rte_flow_action
	{
		use self::rte_flow_action_type::*;
		
		assert!(self.receive_queue_indices_to_distribute_to.len() >= 2, "There must be at least 2 receive queues for receive side scaling");
		
		let (queue_pointer, queue_length) =
		{
			let limited_number_of_receive_queues = ethernet_device_capabilities.limit_number_of_receive_queues(self.receive_queue_indices_to_distribute_to.len());
			assert!(limited_number_of_receive_queues >= ReceiveNumberOfQueues::Two, "There must be at least 2 receive queues for receive side scaling after limiting for device limitations");
			
			let pointer = self.receive_queue_indices_to_distribute_to.as_ptr();
			
			(pointer, limited_number_of_receive_queues)
		};
		
		let (hash_key_pointer, hash_key_length) =
		{
			let mut hash_key = Box::new(self.hash_key_strategy.create(ethernet_device_capabilities, queue_length).unwrap());
			let pointer_and_length = hash_key.pointer_and_length();
			drop_prevention.push(hash_key);
			pointer_and_length
		};
		
		rte_flow_action
		{
			type_: RTE_FLOW_ACTION_TYPE_RSS,
			conf: box_configuration
			(
				drop_prevention,
				rte_flow_action_rss
				{
					func: self.hash_function_algorithm.into(),
					
					types: self.types.bits(),
					
					level: self.level.into(),
					
					key_len: hash_key_length as u32,
					
					key: hash_key_pointer as *const u8,
					
					queue_num: queue_length.into(),
					
					queue: queue_pointer as *const u16,
				}
			),
		}
	}
	
	#[inline(always)]
	fn receive_queue_indices_to_distribute_to_default() -> ArrayVec<[ReceiveQueueIdentifier; RTE_MAX_QUEUES_PER_PORT]>
	{
		let mut queues = ArrayVec::new();
		queues.push(ReceiveQueueIdentifier::from(1u8));
		queues.push(ReceiveQueueIdentifier::from(2u8));
		queues
	}
}
