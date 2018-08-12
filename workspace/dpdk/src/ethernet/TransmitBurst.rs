// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A helper struct to reduce the need for slightly expensive indirect look ups.
///
/// Obtain from an `EthernetPort`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TransmitBurst
{
	transmit_prepare_function_pointer: unsafe extern "C" fn(txq: *mut c_void, tx_pkts: *mut *mut rte_mbuf, nb_pkts: u16) -> u16,
	transmit_burst_function_pointer: eth_tx_burst_t,
	transmit_queue: NonNull<c_void>,
	maximum_number_of_packets_which_can_be_transmitted_at_once: usize,
}

impl TransmitBurst
{
	/// Use this as the upper limit for `maximum_capacity()` of a `transmit_packets_from`.
	#[inline(always)]
	pub fn maximum_number_of_packets_which_can_be_transmitted_at_once(&self) -> usize
	{
		self.maximum_number_of_packets_which_can_be_transmitted_at_once
	}
	
	/// Transmits packets.
	///
	/// Returns number transmitted, which can be zero. Use this to compute `start_from_index` if not all packets were sent.
	///
	/// The number transmitted is equivalent to the number of 'transmit descriptors' available from the poll-mode driver (PMD)'s hardware.
	///
	/// There is no need to free any buffers transmitted; this is managed by the hardware (and how and when is controlled by `tx_free_thresh`).
	///
	/// If the PMD is `DEV_TX_OFFLOAD_MT_LOCKFREE` capable, multiple threads can invoke this function concurrently on the same tx queue without using a software lock. However, only the Cavium Octeon PMD supports this currently.
	///
	/// Will panic in debug mode if `number_of_potential_packets` (`transmit_packets_from.length()`) is zero.
	///
	/// A typical `length` should be 16, 32 or 512. An upper limit on this is:-
	///
	/// * `::std::u16::MAX`.
	/// * The `nb_desc` of `rte_eth_txq_info`.
	#[inline(always)]
	pub fn transmit_packets_in_a_burst<A: UnifiedArrayVecAndVec<NonNull<rte_mbuf>>>(&self, transmit_packets_from: &mut A, start_from_index: usize) -> usize
	{
		let (pointer, number_of_potential_packets) = transmit_packets_from.to_ffi_data_u16(start_from_index);
		
		let number_transmitted_u16 = (self.transmit_burst_function_pointer)(self.transmit_queue.as_ptr(), pointer, number_of_potential_packets);
		debug_assert!(number_transmitted_u16 <= number_of_potential_packets_u16, "number_transmitted_u16 '{}' exceeds number_of_potential_packets_u16 '{}'", number_transmitted_u16, number_of_potential_packets_u16);
		
		number_transmitted_u16 as usize
	}
	
	/// Repeatedly tries to transmit (busy loops).
	///
	/// Will try to prepare packets for hardware offload features before send; panics in debug mode if a packet is incorrect for offloading.
	///
	/// Will not send anything (does not panic) if there are no packets to send.
	///
	/// Finishes by truncating `transmit_packets_from`.
	///
	/// Panics if `start_from_index` exceeds `transmit_packets_from.length()`.
	#[inline(always)]
	pub fn transmit_packets_in_bursts_until_all_sent<A: UnifiedArrayVecAndVec<NonNull<rte_mbuf>>>(&self, transmit_packets_from: &mut A, mut start_from_index: usize)
	{
		debug_assert!(start_from_index <= transmit_packets_from.length(), "start_from_index '{}' is greater than transmit_packets_from.length() '{}'", start_from_index, transmit_packets_from.length());
		
		self.prepare(transmit_packets_from, start_from_index);
		
		while start_from_index < transmit_packets_from.length()
		{
			let number_transmitted = self.transmit_packets_in_a_burst::<A>(transmit_packets_from, start_from_index);
			start_from_index += number_transmitted;
		}
		
		transmit_packets_from.truncate_without_drop();
	}
	
	#[inline(always)]
	fn prepare<A: UnifiedArrayVecAndVec<NonNull<rte_mbuf>>>(&self, transmit_packets_from: &mut A, start_from_index: usize)
	{
		let (pointer, number_of_potential_packets) = transmit_packets_from.to_ffi_data_u16(start_from_index);
		
		let number_acceptable_u16 = (self.transmit_prepare_function_pointer)(self.transmit_queue, pointer, number_of_potential_packets);
		
		debug_assert_eq!(number_acceptable_u16, number_acceptable_u16, "A packet was not acceptable")
	}
	
	unsafe extern "C" fn prepare_is_unsupported(_txq: *mut c_void, _tx_pkts: *mut *mut rte_mbuf, nb_pkts: u16) -> u16
	{
		nb_pkts
	}
}
