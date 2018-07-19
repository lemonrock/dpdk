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

impl Display for InternetProtocolPacketReassemblyTable
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for InternetProtocolPacketReassemblyTable
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "InternetProtocolPacketReassemblyTable")
	}
}

impl PrintInformation for InternetProtocolPacketReassemblyTable
{
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { rte_ip_frag_table_statistics_dump(stream, self.fragmentation_table_pointer() as *const _) }
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
	
	/// * `maximum_number_of_packets_being_reassembled_at_any_one_time` can not be zero. A typical value is 4094.
	/// * `reassembly_timeout` can not be zero. RFC 1122 states it should be between 60 seconds and 120 seconds; Linux defaults to 30 seconds.
	/// * `entries_per_bucket` can not be zero and should be a power of 2; it is rounded up if not. A typical value is 8 or 16.
	#[inline(always)]
	pub fn create(maximum_number_of_packets_being_reassembled_at_any_one_time: u16, entries_per_bucket: u16, reassembly_timeout: Seconds, numa_node_choice: NumaNodeChoice) -> Result<Self, ()>
	{
		debug_assert_ne!(maximum_number_of_packets_being_reassembled_at_any_one_time, 0, "maximum_number_of_packets_being_reassembled_at_any_one_time can not be zero");
		
		let number_of_buckets = maximum_number_of_packets_being_reassembled_at_any_one_time as u32;
		let entries_per_bucket = (entries_per_bucket.checked_next_power_of_two().expect("entries_per_bucket is too large; it can not exceed 2^15")) as u32;
		
		let maximum_entries = number_of_buckets * entries_per_bucket;
		let death_row = rte_ip_frag_death_row::default();
		
		debug_assert!(maximum_entries <= (number_of_buckets * entries_per_bucket), "maximum_entries should be less than or equal to maximum_number_of_packets_being_reassembled_at_any_one_time * entries_per_bucket");
		debug_assert!(Self::DeathRowPreFetchFactor as usize <= death_row.row.len(), "The DeathRowPreFetchFactor '{}' exceeds the maximum size of death_row.row.len() '{}'.");
		
		let reassembly_timeout_cycles =
		{
			debug_assert_ne!(reassembly_timeout, Seconds::Zero, "reassembly_timeout can not be zero");
			debug_assert!(reassembly_timeout <= Seconds::TwoMinutes, "reassembly_timeout can not exceed 2 minutes");
			
			let number_of_cycles_in_one_second_for_the_tsc_timer = Hertz::number_of_cycles_in_one_second_for_the_tsc_timer().into();
			debug_assert_ne!(number_of_cycles_in_one_second_for_the_tsc_timer, 0, "number_of_cycles_in_one_second_for_the_tsc_timer can not be zero");
			number_of_cycles_in_one_second_for_the_tsc_timer * reassembly_timeout.into()
		};
		
		let result = unsafe { rte_ip_frag_table_create(number_of_buckets, entries_per_bucket, maximum_entries, reassembly_timeout_cycles, numa_node_choice.into()) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok
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
	/// The packet does not have to be fragmented; unfragmented packets are passed back.
	///
	/// At point of entry, `l2_len` and `l3_len` need to be set correctly.
	///
	/// `incoming_packet_internet_protocol_version_4_packet_header` must point into `incoming_packet`.
	///
	/// Result may be none, the same packet or a different packet as `incoming_packet`.
	///
	/// A none result implies that the `incoming_packet` was a fragment and that not all fragments have been received.
	///
	/// `incoming_packet_arrival_timestamp` is expensive to compute.
	///
	/// Use `Cycles::current_rdtsc_cycles_since_boot()` or a cached value.
	#[inline(always)]
	pub fn reassemble_fragmented_internet_protocol_version_4_packet(&mut self, incoming_packet: PacketBuffer, incoming_packet_arrival_timestamp: Cycles, incoming_packet_internet_protocol_version_4_packet_header: &mut InternetProtocolVersion4PacketHeader) -> Option<PacketBuffer>
	{
		if incoming_packet_internet_protocol_version_4_packet_header.is_fragmented()
		{
			let result = unsafe { rte_ipv4_frag_reassemble_packet(self.fragmentation_table_pointer(), self.death_row(), incoming_packet.as_ptr(), incoming_packet_arrival_timestamp.into(), incoming_packet_internet_protocol_version_4_packet_header.into()) };
			
			PacketBuffer::from_possibly_null_rte_mbuf(result)
		}
		else
		{
			Some(incoming_packet)
		}
	}
	
	/// Reassembles an internet protocol (IP) version 6 packet.
	///
	/// The packet does not have to be fragmented; unfragmented packets are passed back.
	///
	/// At point of entry, `l2_len` and `l3_len` need to be set correctly.
	///
	/// `incoming_packet_internet_protocol_version_6_packet_header` and `extension_header_fragment_inside_incoming_packet` must be pointers into `incoming_packet`.
	///
	/// Result may be none, the same packet or a different packet as `incoming_packet`.
	///
	/// A none result implies that the `incoming_packet` was a fragment and that not all fragments have been received.
	///
	/// `incoming_packet_arrival_timestamp` is expensive to compute.
	///
	/// Use `Cycles::current_rdtsc_cycles_since_boot()` or a cached value.
	#[inline(always)]
	pub fn reassemble_fragmented_internet_protocol_version_6_packet(&mut self, incoming_packet: PacketBuffer, incoming_packet_arrival_timestamp: Cycles, incoming_packet_internet_protocol_version_6_packet_header: &mut InternetProtocolVersion6PacketHeader) -> Option<PacketBuffer>
	{
		let extension_header_fragment_inside_incoming_packet = incoming_packet_internet_protocol_version_6_packet_header.is_fragmented();
		if extension_header_fragment_inside_incoming_packet.is_null()
		{
			return Some(incoming_packet)
		}
		
		let result = unsafe { rte_ipv6_frag_reassemble_packet(self.fragmentation_table_pointer(), self.death_row(), incoming_packet.as_ptr(), incoming_packet_arrival_timestamp.into(), incoming_packet_internet_protocol_version_6_packet_header.into(), extension_header_fragment_inside_incoming_packet as *mut _) };
		
		PacketBuffer::from_possibly_null_rte_mbuf(result)
	}
	
	/// Call this once after adding all fragments with either `reassemble_fragmented_internet_protocol_version_6_packet` or `reassemble_fragmented_internet_protocol_version_6_packet`.
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
	
	#[inline(always)]
	fn fragmentation_table_pointer(&self) -> *mut rte_ip_frag_tbl
	{
		self.fragmentation_table.as_ptr()
	}
	
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used
	fn death_row(&mut self) -> &mut rte_ip_frag_death_row
	{
		&mut self.death_row
	}
}
