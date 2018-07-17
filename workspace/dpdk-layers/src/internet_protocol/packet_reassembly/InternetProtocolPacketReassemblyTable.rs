// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


///
///
///
///                                IMPORTANT NOTICE
///
///     This code uses the words 'death_row' and 'free' and seems quite tasteless and trivialising.
///     Be assured that the use of this language is solely to ensure correspondence with the
///     underlying logic in the DPDK library. By doing so, we allow a maintainer of this code to analyze
///     it effectively.
///
///     I, the original developer, find DPDK's choice of analogy deeply inappropriate and juvenile.
///
///       Raphael James Cohn
///         Original Developer
///         Skipton, 12th April 2017
///
///
///
///
pub struct InternetProtocolPacketReassemblyTable
{
	fragmentation_table: NonNull<rte_ip_frag_tbl>,
	death_row: rte_ip_frag_death_row,
}

impl PrintInformation for InternetProtocolPacketReassemblyTable
{
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { rte_ip_frag_table_statistics_dump(stream, self.handle() as *const _) }
	}
}

impl Drop for InternetProtocolPacketReassemblyTable
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rte_ip_frag_table_destroy(self.fragmentation_table.as_ptr()) };
		
		// Please see the IMPORTANT NOTICE above which explains why this language has been used.
		self.free_all_packets_on_death_row();
	}
}

impl InternetProtocolPacketReassemblyTable
{
	/// * `librte_port/rte_port_ras.c` uses 3.
	/// * `main.c` in the DPDK ip_reassembly example uses `PREFETCH_OFFSET`, 3.
	/// * Maximum is 256.
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used.
	const DeathRowPreFetchFactor: u32 = 3;
	
	/// Checks if an internet protocol (IP) version 4 packet is fragmented.
	#[inline(always)]
	pub fn is_internet_protocol_version_4_packet_fragmented(header: NonNull<ipv4_hdr>) -> bool
	{
		match unsafe { rust_rte_ipv4_frag_pkt_is_fragmented(header.as_ptr() as *const _) }
		{
			0 => false,
			1 => true,
			illegal @ _ => panic!("rust_rte_ipv4_frag_pkt_is_fragmented() returned illegal value '{}'", illegal),
		}
	}
	
	/// If an internet protocol (IP) version 6 packet is fragmented, gets the fragmentation header.
	///
	/// Current implementation is naive and only checks first extension header.
	///
	/// Returns null if no header is present.
	#[inline(always)]
	pub fn is_internet_protocol_version_6_packet_fragmented(header: NonNull<ipv6_hdr>) -> *mut ipv6_extension_fragment
	{
		unsafe { rust_rte_ipv6_frag_get_ipv6_fragment_header(header.as_ptr()) }
	}
	
	/// Number of Buckets:-
	///
	/// * `librte_port/rte_port_ras.c` uses `RTE_PORT_RAS_N_BUCKETS`, 4094.
	/// * `main.c` in the DPDK ip_reassembly example uses maximumFlowNumber, 1 - 65535, default value is 4096.
	/// * Documentation uses maximumFlowNumber + maximumFlowNumber / 4.
	/// * We use 4094.
	///
	/// Entries per Bucket:-
	///
	/// * `librte_port/rte_port_ras.c` uses `RTE_PORT_RAS_N_ENTRIES_PER_BUCKET`, 8.
	/// * `main.c` in the DPDK ip_reassembly example uses `IP_FRAG_TBL_BUCKET_ENTRIES`, 16.
	/// * We use 16.
	///
	/// MaximumFlowTimeToLive is calculated from maximum_time_to_live_in_cycles_for_each_fragmented_packet in all examples using the same calculation.
	///
	/// * `librte_port/rte_port_ras.c` uses maximum_flow_time_to_live of `MS_PER_S * 100`, ie 100 seconds.
	/// * `main.c` in the DPDK ip_reassembly example uses 1 second.
	/// * We use 100 seconds.
	#[inline(always)]
	pub fn defaultish(numa_node_choice: NumaNodeChoice) -> Self
	{
		const NumberOfBuckets: u16 = 4094;
		const EntriesPerBucket: PowerOfTwoSixteenBit = PowerOfTwoSixteenBit::_16;
		const MaximumFlowTimeToLive: Seconds = Seconds::from(100u64);
		Self::create(NumberOfBuckets, EntriesPerBucket, MaximumFlowTimeToLive, numa_node_choice)
	}
	
	/// `number_of_buckets` is really the maximum number of active flows, ie, of sets of fragmented packets.
	#[inline(always)]
	pub fn create(number_of_buckets: u16, entries_per_bucket: PowerOfTwoSixteenBit, maximum_flow_time_to_live: Seconds, numa_node_choice: NumaNodeChoice) -> Option<Self>
	{
		let number_of_cycles_in_one_second_for_the_tsc_timer = Hertz::number_of_cycles_in_one_second_for_the_tsc_timer().into();
		let number_of_buckets = number_of_buckets as u32;
		let entries_per_bucket = (entries_per_bucket as u16) as u32;
		// * `librte_port/rte_port_ras.c` uses `RTE_PORT_RAS_N_ENTRIES = RTE_PORT_RAS_N_BUCKETS * RTE_PORT_RAS_N_ENTRIES_PER_BUCKET`.
		// * `main.c` in the DPDK ip_reassembly example uses `maximumFlowNumber, 1 - 65535`, default value is `4096`.
		let maximum_entries = number_of_buckets * entries_per_bucket;
		let death_row = rte_ip_frag_death_row::default();
		
		debug_assert_ne!(number_of_buckets, 0, "number_of_buckets can not be zero");
		debug_assert!(!maximum_flow_time_to_live.is_zero(), "maximum_flow_time_to_live can not be zero");
		debug_assert_ne!(number_of_cycles_in_one_second_for_the_tsc_timer, 0, "number_of_cycles_in_one_second_for_the_tsc_timer can not be zero");
		debug_assert!(maximum_entries <= (number_of_buckets * entries_per_bucket), "maximum_entries should be less than or equal to number_of_buckets * entries_per_bucket");
		debug_assert!(Self::DeathRowPreFetchFactor as usize <= death_row.row.len(), "The DeathRowPreFetchFactor '{}' exceeds the maximum size of death_row.row.len() '{}'.");
		
		let maximum_time_to_live_in_cycles_for_each_fragmented_packet = number_of_cycles_in_one_second_for_the_tsc_timer * maximum_flow_time_to_live.into();

		let result = unsafe { rte_ip_frag_table_create(number_of_buckets, entries_per_bucket, maximum_entries, maximum_time_to_live_in_cycles_for_each_fragmented_packet, numa_node_choice.into()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					fragmentation_table: unsafe { NonNull::new_unchecked(result) },
					death_row,
				}
			)
		}
	}
	
	/// Reassembles an internet protocol (IP) version 4 packet.
	///
	/// At point of entry, `l2_len` and `l3_len` need to be set correctly.
	///
	/// `header_inside_incoming_fragmented_packet` must be be pointers into `incoming_fragmented_packet`.
	///
	/// Result may be null, the same packet or a different packet as `incoming_fragmented_packet`.
	///
	/// A null result implies that the `incoming_fragmented_packet` was a fragment and that not all fragments have been received.
	///
	/// `fragment_arrival_time_stamp_from_rdtsc` is expensive to compute. Use `Cycles::current_rdtsc_cycles_since_boot().into()`.
	#[inline(always)]
	pub fn reassemble_fragmented_internet_protocol_version_4_packet(&mut self, incoming_fragmented_packet: NonNull<rte_mbuf>, fragment_arrival_time_stamp_from_rdtsc: u64, header_inside_incoming_fragmented_packet: NonNull<ipv4_hdr>) -> *mut rte_mbuf
	{
		unsafe { rte_ipv4_frag_reassemble_packet(self.fragmentation_table.as_ptr(), &mut self.death_row, incoming_fragmented_packet, fragment_arrival_time_stamp_from_rdtsc, header_inside_incoming_fragmented_packet) }
	}
	
	/// Reassembles an internet protocol (IP) version 6 packet.
	///
	/// At point of entry, `l2_len` and `l3_len` need to be set correctly.
	///
	/// `header_inside_incoming_fragmented_packet` and `extension_header_fragment_inside_incoming_fragmented_packet` must be pointers into `incoming_fragmented_packet`.
	///
	/// Result may be null, the same packet or a different packet as `incoming_fragmented_packet`.
	///
	/// A null result implies that the `incoming_fragmented_packet` was a fragment and that not all fragments have been received.
	///
	/// `fragment_arrival_time_stamp_from_rdtsc` is expensive to compute. Use `Cycles::current_rdtsc_cycles_since_boot().into()`.
	#[inline(always)]
	pub fn reassemble_fragmented_internet_protocol_version_6_packet(&mut self, incoming_fragmented_packet: NonNull<rte_mbuf>, fragment_arrival_time_stamp_from_rdtsc: u64, header_inside_incoming_fragmented_packet: NonNull<ipv6_hdr>, extension_header_fragment_inside_incoming_fragmented_packet: NonNull<ipv6_extension_fragment>) -> *mut rte_mbuf
	{
		unsafe { rte_ipv6_frag_reassemble_packet(self.fragmentation_table.as_ptr(), &mut self.death_row, incoming_fragmented_packet, fragment_arrival_time_stamp_from_rdtsc, header_inside_incoming_fragmented_packet, extension_header_fragment_inside_incoming_fragmented_packet) }
	}
	
	/// Call this once after adding all fragments with either `reassemble_fragmented_internet_protocol_version_4_packet` or `reassemble_fragmented_internet_protocol_version_6_packet`.
	///
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used
	#[inline(always)]
	pub fn if_death_row_is_full_free_all_packets_on_death_row(&mut self)
	{
		if self.is_death_row_full()
		{
			self.free_all_packets_on_death_row()
		}
	}

	/// Please see the IMPORTANT NOTICE above which explains why this language has been used.
	#[inline(always)]
	fn is_death_row_full(&self) -> bool
	{
		self.death_row.cnt == self.death_row.row.len() as u32
	}

	/// Please see the IMPORTANT NOTICE above which explains why this language has been used.
	#[inline(always)]
	fn free_all_packets_on_death_row(&mut self)
	{
		unsafe { rte_ip_frag_free_death_row(&mut self.death_row, Self::DeathRowPreFetchFactor) };
	}
}

