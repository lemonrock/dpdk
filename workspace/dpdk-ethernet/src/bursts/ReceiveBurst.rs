// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A helper struct to reduce the need for slightly expensive indirect look ups.
///
/// Obtain from an `EthernetPort`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ReceiveBurst
{
	receive_burst_function_pointer: eth_rx_burst_t,
	receive_queue: NonNull<c_void>,
	receive_memory_pool: NonNull<rte_mempool>, // use this when working with fragments?
	maximum_number_of_packets_which_can_be_received_at_once: usize,
}

impl ReceiveBurst
{
	/// Use this as the upper limit for `maximum_capacity()` of a `receive_packets_into`.
	#[inline(always)]
	pub fn maximum_number_of_packets_which_can_be_received_at_once(&self) -> usize
	{
		self.maximum_number_of_packets_which_can_be_received_at_once
	}
	
	/// Receives packets.
	///
	/// Returns number received, which can be zero.
	///
	/// If zero is returned repeatedly, then one should check the device link status (ie is it up)!
	///
	/// Will panic in debug mode if `number_of_potential_packets` (`receive_packets_into.maximum_capacity() - receive_packets_into.length()`) is zero.
	///
	/// A typical `number_of_potential_packets` should be 16, 32 or 512. An upper limit on this is:-
	///
	/// * `::std::u16::MAX`.
	/// * The `nb_desc` of `rte_eth_rxq_info`.
	#[inline(always)]
	pub fn receive_packets_in_a_burst<A: NonNullUnifiedArrayVecAndVec<rte_mbuf>>(&self, receive_packets_into: &mut A) -> usize
	{
		let (pointer, number_of_potential_packets) = receive_packets_into.from_ffi_data_u16();
		
		let number_of_packets_received = unsafe { (self.receive_burst_function_pointer)(self.receive_queue.as_ptr(), pointer, number_of_potential_packets) };
		debug_assert!(number_of_packets_received <= number_of_potential_packets, "number_of_packets_received '{}' exceeds number_of_potential_packets_u16 '{}'", number_of_packets_received, number_of_potential_packets);
		
		let number_of_packets_received = number_of_packets_received as usize;
		let length = receive_packets_into.length();
		receive_packets_into.set_length(length + number_of_packets_received);
		number_of_packets_received
	}
	
	#[inline(always)]
	pub(crate) fn new(ethernet_port_identifier: EthernetPortIdentifier, ethernet_device_capabilities: &EthernetDeviceCapabilities, queue_identifier: ReceiveQueueIdentifier) -> ReceiveBurst
	{
		let receive_queue_information = ethernet_port_identifier.receive_queue_information(queue_identifier);
		debug_assert_eq!(receive_queue_information.scattered_rx, 0, "Packet receive scatter (ie multiple segment, non-contiguous packets) is not supported but this queue has it enabled");
		debug_assert!(!receive_queue_information.mp.is_null(), "Packet receive queue memory pool is null");
		
		let ethernet_device_mutable: &'static mut rte_eth_dev = ethernet_port_identifier.ethernet_device_mutable();
		
		if cfg!(debug_assertions)
		{
			let callbacks = &mut ethernet_device_mutable.post_rx_burst_cbs;
			let mut index = 0;
			for callback in callbacks.iter()
			{
				debug_assert!(callback.is_null(), "Post-process callback after receive burst at index '{}' is not null", index);
				index +=1;
			}
		}
		
		ReceiveBurst
		{
			receive_burst_function_pointer: ethernet_device_mutable.rx_pkt_burst,
			receive_queue: ethernet_port_identifier.receive_queue(queue_identifier),
			maximum_number_of_packets_which_can_be_received_at_once: ethernet_device_capabilities.receive_burst_maximum_packets(),
			receive_memory_pool: unsafe { NonNull::new_unchecked(receive_queue_information.mp) },
		}
	}
}
