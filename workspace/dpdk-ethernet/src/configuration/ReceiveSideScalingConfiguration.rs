// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive Side Scaling (RSS) configuration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ReceiveSideScalingConfiguration
{
	/// Number of receive queues to use for receive side scaling.
	pub number_of_receive_queues: ReceiveNumberOfQueues,
	
	/// Hash function key data strategy.
	///
	/// ie How to generate the key data bytes.
	pub hash_function_key_data_strategy: HashFunctionKeyDataStrategy,
	
	/// Redirection table strategy, ie how to direct different hash results to receive queues.
	pub redirection_table_strategy: RedirectionTableStategy,
}

impl ReceiveSideScalingConfiguration
{
	pub(crate) fn create<'a>(this: Option<&'a Self>, ethernet_device_capabilities: &EthernetDeviceCapabilities, receive_queue_configurations: &[ReceiveQueueConfiguration]) -> (rte_eth_rx_mq_mode, rte_eth_rss_conf, Option<ReceiveSideScalingHashKey<'a>>, Option<RedirectionTable>)
	{
		match this
		{
			None => Self::no_receive_side_scaling(),
			Some(this) => this.create_internal(ethernet_device_capabilities, receive_queue_configurations)
		}
	}
	
	#[inline(always)]
	fn create_internal<'a>(&'a self, ethernet_device_capabilities: &EthernetDeviceCapabilities, receive_queue_configurations: &[ReceiveQueueConfiguration]) -> (rte_eth_rx_mq_mode, rte_eth_rss_conf, Option<ReceiveSideScalingHashKey<'a>>, Option<RedirectionTable>)
	{
		assert!(receive_queue_configurations.len() >= self.number_of_receive_queues.into(), "Not enough receive queue configurations for receive side scaling");
		
		if self.number_of_receive_queues.is_zero()
		{
			Self::no_receive_side_scaling()
		}
		else
		{
			match self.hash_function_key_data_strategy.create(ethernet_device_capabilities, self.number_of_receive_queues)
			{
				None => Self::no_receive_side_scaling(),
				Some(mut receive_side_scaling_hash_key) => match self.redirection_table_strategy.create(ethernet_device_capabilities, self.number_of_receive_queues).expect("If there is a RSS key size")
				{
					None => Self::no_receive_side_scaling(),
					Some(redirection_table) =>
					{
						let rss_conf =
						{
							let (pointer, length) = receive_side_scaling_hash_key.pointer_and_length();
							rte_eth_rss_conf
							{
								rss_key: pointer,
								rss_key_len: length,
								rss_hf: ethernet_device_capabilities.receive_side_scaling_offload_flow().bits(),
							}
						};
						
						(rte_eth_rx_mq_mode::ETH_MQ_RX_RSS, rss_conf, Some(receive_side_scaling_hash_key), Some(redirection_table))
					}
				}
			}
		}
	}
	
	#[inline(always)]
	fn no_receive_side_scaling<'a>() -> (rte_eth_rx_mq_mode, rte_eth_rss_conf, Option<ReceiveSideScalingHashKey<'a>>, Option<RedirectionTable>)
	{
		(rte_eth_rx_mq_mode::ETH_MQ_RX_NONE, unsafe { zeroed() }, None, None)
	}
}
