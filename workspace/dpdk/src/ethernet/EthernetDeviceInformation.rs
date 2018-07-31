// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


trait EthernetDeviceInformation
{
	// ETH_RSS_L2_PAYLOAD is rarely supported and ETH_RSS_PORT is almost unused.
	const DesiredReceiveSideScalingHashProtocols: u64 = ETH_RSS_IP | ETH_RSS_UDP | ETH_RSS_TCP;
	
	/// Returns driver name after fixing buggy maximum receive queue information.
	#[inline(always)]
	fn driver_name_and_fix_buggy_information<'a>(&'a mut self) -> &'a CStr
	{
		let this = self.mutable_reference();
		
		let driver_name = unsafe { CStr::from_ptr(this.driver_name) }.to_str();
		
		let possibly_buggy_max_rx_queues = this.max_rx_queues;
		this.max_rx_queues = match driver_name
		{
			Some("rte_ixgbe_pmd") => min(possibly_buggy_max_rx_queues, 16),
			Some("rte_ixgbevf_pmd") => min(possibly_buggy_max_rx_queues, 4),
			Some("rte_i40e_pmd") => min(possibly_buggy_max_rx_queues, 64),
			Some("rte_i40evf_pmd") => min(possibly_buggy_max_rx_queues, 16),
			_ => possibly_buggy_max_rx_queues,
		};
		
		let some_drivers_use_nonsense_values_that_exceed_super_jumbo_frame_sizes = min(self.max_rx_pktlen, EthernetFrameLength::MaximumIncludingCyclicRedundancyCheckWithJumboFrames.into());
		self.max_rx_pktlen = some_drivers_use_nonsense_values_that_exceed_super_jumbo_frame_sizes;
		
		driver_name
	}
	
	// ARP, etc traffic gets pushed to reta index 0.
	// so we might want to load the queue at reta index 0 slightly lower, or even take it out of the reta table.
	
	/// Creates a redirection table.
	///
	/// Returns the table and the table's size (which is 64x larger than the length of the table).
	#[inline(always)]
	fn redirection_table(&self, number_of_receive_side_scaling_queues: u16, first_receive_side_scaling_queue: ReceiveQueueIdentifer) -> (Vec<rte_eth_rss_reta_entry64>, u16)
	{
		const GroupSizeU16: u16 = RTE_RETA_GROUP_SIZE as u16;
		
		let redirection_table_size = self.reference().reta_size;
		debug_assert_ne!(redirection_table_size, 0, "Call is_receive_side_scaling_is_unavailable() first");
		debug_assert!(redirection_table_size.is_power_of_two(), "Redirection table (RETA) size is not a power of two");
		debug_assert_eq!(redirection_table_size % GroupSizeU16, 0, "redirection_table_size '{}' is not a multiple of RTE_RETA_GROUP_SIZE '{}'", redirection_table_size, RTE_RETA_GROUP_SIZE);
		debug_assert!(number_of_receive_side_scaling_queues <= redirection_table_size, "number_of_receive_side_scaling_queues '{}' exceeds redirection table (RETA) size '{}'", number_of_receive_side_scaling_queues, redirection_table_size);
		
		let number_of_structs = (redirection_table_size / RTE_RETA_GROUP_SIZE) as usize;
		let mut redirection_table = Vec::with_capacity(number_of_structs);
		for _ in 0 .. number_of_structs
		{
			redirection_table.push
			(
				rte_eth_rss_reta_entry64
				{
					mask: ::std::u64::MAX,
					reta: unsafe { uninitialized() },
				}
			)
		}
		
		let number_of_whole_iterations = redirection_table_size / number_of_receive_side_scaling_queues;
		let remainder_to_assign = redirection_table_size % number_of_receive_side_scaling_queues;
		
		let mut redirection_table_index = 0;
		for iteration in 0 .. number_of_whole_iterations
		{
			for receive_queue_identifier in first_receive_side_scaling_queue.upto(number_of_receive_side_scaling_queues)
			{
				let rte_eth_rss_reta_entry64_index = (redirection_table_index / GroupSizeU16) as usize;
				let rte_eth_rss_reta_entry64_reta_index = (redirection_table_index % GroupSizeU16) as usize;
				redirection_table.get_mut(rte_eth_rss_reta_entry64_index).reta[rte_eth_rss_reta_entry64_reta_index] = receive_queue_identifier.into();
				redirection_table_index += 1;
			}
		}
		
		for receive_queue_identifier in first_receive_side_scaling_queue.upto(remainder_to_assign)
		{
			let rte_eth_rss_reta_entry64_index = (redirection_table_index / GroupSizeU16) as usize;
			let rte_eth_rss_reta_entry64_reta_index = (redirection_table_index % GroupSizeU16) as usize;
			redirection_table.get_mut(rte_eth_rss_reta_entry64_index).reta[rte_eth_rss_reta_entry64_reta_index] = receive_queue_identifier.into();
			redirection_table_index += 1;
		}
		
		(redirection_table, redirection_table_size)
	}
	
	/// Computes receive side scaling properties.
	///
	/// * Returns `Some(number_of_receive_side_scaling_queues, receive_side_scaling_hash_key)`.
	/// * Returns `None` if receive side scaling is not possible.
	///
	/// Call this only after `self.driver_name_and_fix_buggy_information()`.
	#[inline(always)]
	fn number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key(self, maximum_receive_side_scaling_queues: u16, receive_side_scaling_toeplitz_hash_function_key_data_strategy: &ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy) -> Option<(u16, Vec<u8>)>
	{
		debug_assert_ne!(maximum_receive_side_scaling_queues, 0, "maximum_receive_side_scaling_queues is zero");
		let this = self.reference();
		
		if self.is_receive_side_scaling_is_unavailable()
		{
			None
		}
		else
		{
			let number_of_receive_side_scaling_queues = min(maximum_receive_side_scaling_queues, this.max_rx_queues);
			
			let receive_side_scaling_hash_key = receive_side_scaling_toeplitz_hash_function_key_data_strategy.generate(this.hash_key_size, number_of_receive_side_scaling_queues);
			
			Some((number_of_receive_side_scaling_queues, receive_side_scaling_hash_key))
		}
	}
	
	#[inline(always)]
	fn number_of_transmit_queues(&self, maximum_transmit_queues: u16) -> u16
	{
		min(self.reference().max_tx_queues, maximum_transmit_queues)
	}
	
	/// Computes the maximum number of receive-transmit queue pairs.
	///
	/// Call this only after `self.driver_name_and_fix_buggy_information()`.
	#[inline(always)]
	fn maximum_receive_transmit_queue_pairs(self, available_cores: u16)
	{
		debug_assert_ne!(available_cores, 0, "available_cores is zero");
		
		let this = self.reference();
		
		debug_assert_ne!(this.max_rx_queues, 0, "Zero maximum receive queues");
		debug_assert_ne!(this.max_tx_queues, 0, "Zero maximum transmit queues");
		
		min(min(available_cores, this.max_rx_queues), this.max_tx_queues);
	}
	
	/// Is receive side scaling unavailable?
	#[inline(always)]
	fn is_receive_side_scaling_is_unavailable(self) -> bool
	{
		self.reference().flow_type_rss_offloads == 0 || self.reference().reta_size == 0
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn reference(&self) -> &rte_eth_dev_info;
	
	#[doc(hidden)]
	#[inline(always)]
	fn mutable_reference(&mut self) -> &mut rte_eth_dev_info;
}

impl EthernetDeviceInformation for rte_eth_dev_info
{
	#[inline(always)]
	fn reference(&self) -> &rte_eth_dev_info
	{
		self
	}
	
	#[inline(always)]
	fn mutable_reference(&mut self) -> &mut rte_eth_dev_info
	{
		self
	}
}
