// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a doubly linked list, with fragments stored left-to-right, where 'left' are offsets closer to zero.
///
/// As an optimization, fragments are checked right-to-left (ie highest fragment first).
/// If fragments are received in-order (which is often the case) or slightly out-of-order, then this is likely to create a substantial performance gain.
/// However, this design has its worst performance if a flow of packets, say crafted by an attacker, go from rightmost to leftmost, as the linked list will be walked numerous times.
struct InternetProtocolVersion6FragmentList
{
	fragmentation_reassembly_started_at_timestamp_in_milliseconds: u64,
	leftmost: FragmentListNode,
	rightmost: FragmentListNode,
	has_end_fragment: bool,
	fragmented_data_in_bytes: usize,
	first_fragment_details: Option<InternetProtocolVersion6FirstFragmentDetails>,
	invariant_flow_label: u32,
	invariant_differentiated_service_code_point: DifferentiatedServiceCodePoint,
	current_knowledge_of_explicit_congestion_notification: ExplicitCongestionNotification,
	was_congestion_encountered: bool,
}

impl InternetProtocolVersion6FragmentList
{
	const MaximumFragmentReassemblyTimeInMilliseconds: u64 = 30 * 1_000;
	
	#[inline(always)]
	pub(crate) fn new(fragmentation_reassembly_started_at_timestamp_in_milliseconds: u64, internet_protocol_version_6_packet_header: &InternetProtocolVersion6PacketHeader, packet_buffer: PacketBuffer, offset: u16, length: u16, fragmented_payload_offset: u16, has_more_flag_set: bool, fragment_details: InternetProtocolVersion6FirstFragmentDetails) -> Result<Self, ()>
	{
		if length == 0
		{
			return Err(())
		}
		
		let is_first_fragment = offset == 0;
		let is_last_fragment = !has_more_flag_set;
		
		if is_first_fragment && is_last_fragment
		{
			return Err(())
		}
		
		let (differentiated_service_code_point, explicit_congestion_notification) = internet_protocol_version_6_packet_header.traffic_class();
		
		let fragment_list_node_to_insert = FragmentListNode::new(packet_buffer, offset, length, fragmented_payload_offset);
		
		Ok
		(
			Self
			{
				leftmost: fragment_list_node_to_insert,
				rightmost: fragment_list_node_to_insert,
				has_end_fragment: is_last_fragment,
				fragmented_data_in_bytes: length as usize,
				first_fragment_details: if is_first_fragment
				{
					Some(fragment_details)
				}
				else
				{
					None
				},
				invariant_flow_label: internet_protocol_version_6_packet_header.flow_label(),
				invariant_differentiated_service_code_point: differentiated_service_code_point,
				current_knowledge_of_explicit_congestion_notification: explicit_congestion_notification,
				was_congestion_encountered: explicit_congestion_notification.congestion_encountered(),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn expired(&self, now_timestamp_in_milliseconds: u64) -> bool
	{
		self.fragmentation_reassembly_started_at_timestamp_in_milliseconds + MaximumFragmentReassemblyTimeInMilliseconds >= now_timestamp_in_milliseconds
	}
	
	#[inline(always)]
	pub(crate) fn process_fragment(&mut self, internet_protocol_version_6_packet_header: &InternetProtocolVersion6PacketHeader, packet_buffer: PacketBuffer, offset: u16, length: u16, fragmented_payload_offset: u16, has_more_flag_set: bool, fragment_details: InternetProtocolVersion6FirstFragmentDetails) -> Result<Option<PacketBuffer>, ()>
	{
		debug_assert!(offset as usize + length as usize <= ::std::u16::MAX as usize, "offset + length exceeds 65,535");
		
		// A DPDK packet has a maximum data_len of an u16, but DPDK needs a 2Kb head room; ethernet headers need 14 bytes, IPv6 header needs 40 bytes and various extensions headers are possible.
		const RoughMaximum: usize = 60 * 1024;
		
		if self.fragmented_data_in_bytes + (length as usize) > RoughMaximum
		{
			return Err(())
		}
		
		self.check_flow_label_matches(internet_protocol_version_6_packet_header)?;
		
		self.check_differentiated_service_code_point_and_explicit_congestion_notification_matches(internet_protocol_version_6_packet_header)?;
		
		let fragment_list_node_to_insert = FragmentListNode::new(packet_buffer, offset, length, fragmented_payload_offset);
		
		let is_last_fragment = !has_more_flag_set;
		
		if !self.first_fragment(fragment_list_node_to_insert, is_last_fragment, fragment_details)?
		{
			if !self.end_fragment(fragment_list_node_to_insert, is_last_fragment)?
			{
				self.insert_node_walking_from_right_to_left(fragment_list_node_to_insert)?;
			}
		}
		
		self.fragmented_data_in_bytes += length as usize;
		
		if self.complete()
		{
			let reassembled_packet_buffer = self.reassemble(packet_buffer_pool)?;
			Ok(Some(reassembled_packet_buffer))
		}
		else
		{
			Ok(None)
		}
	}
	
	#[inline(always)]
	fn reassemble(&self, packet_buffer_pool: PacketBufferPool) -> Result<PacketBuffer, ()>
	{
		let reassembled_packet_buffer = packet_buffer_pool.allocate()?;
		
		let available_space_for_data = reassembled_packet_buffer.segment_buffer_tail_room() as usize;
		
		let first_fragment_details = self.first_fragment_details.unwrap();
		
		let header_to = reassembled_packet_buffer.offset_into_data::<u8>(0).as_ptr();
		
		let (mut to, available_space_for_data) =
			{
				let header_from = self.leftmost.0.offset_into_data::<u8>(first_fragment_details.offset_of_ethernet_packet_header).as_ptr().as_ptr() as *const u8;
				let header_bytes_to_copy = size_of::<EthernetPacketHeader>() + size_of::<InternetProtocolVersion6PacketHeader>() + first_fragment_details.per_fragment_extension_headers_length;
				debug_assert!(available_space_for_data >= header_bytes_to_copy, "packet buffer to copy fragments into does not have enough header space");
				unsafe { copy_nonoverlapping(header_from, header_to, header_bytes_to_copy) };
				(unsafe { header_to.offset(header_bytes_to_copy as isize) }, available_space_for_data - header_bytes_to_copy)
			};
		
		if unlikely!(available_space_for_data < (self.fragmented_data_in_bytes as usize))
		{
			return Err(())
		}
		
		let mut reassembled_payload_length = first_fragment_details.per_fragment_extension_headers_length;
		let mut next = Some(self.leftmost);
		while next.is_some()
		{
			let offset = next.fields().fragmented_payload_offset as usize;
			let fragmented_paylod_data_from = next.0.offset_into_data::<u8>(offset).as_ptr() as *const u8;
			let fragmented_paylod_data_length = next.fields().length as usize;
			unsafe { copy_nonoverlapping(fragmented_paylod_data_from, to, fragmented_paylod_data_length) }
			
			to = unsafe { to.offset(fragmented_paylod_data_length as isize) };
			reassembled_payload_length += fragmented_paylod_data_length;
			
			next = next.right();
		}
		
		let internet_protocol_version_6_packet_header = unsafe { &mut * reassembled_packet_buffer.offset_into_data::<InternetProtocolVersion6PacketHeader>(size_of::<EthernetPacketHeader>() + size_of::<InternetProtocolVersion6PacketHeader>()).as_ptr() };
		internet_protocol_version_6_packet_header.set_payload_length_including_extension_headers(reassembled_payload_length);
		internet_protocol_version_6_packet_header.set_explicit_congestion_notification(if self.was_congestion_encountered
		{
			ExplicitCongestionNotification::CongestionEncountered
		}
		else
		{
			self.current_knowledge_of_explicit_congestion_notification
		});
		let next_header_to_change_pointer = unsafe { (internet_protocol_version_6_packet_header as *mut u8).offset(first_fragment_details.offset_of_next_header_to_change_relative_to_start_of_internet_protocol_version_6_packet_header as isize) };
		unsafe { *next_header_to_change_pointer = first_fragment_details.next_header };
		
		let packet_length = (size_of::<EthernetPacketHeader>() + size_of::<InternetProtocolVersion6PacketHeader>() + reassembled_payload_length) as u16;
		reassembled_packet_buffer.mutable_reference().pkt_len = packet_length;
		reassembled_packet_buffer.mutable_reference().data_len = packet_length;
		
		Ok(reassembled_packet_buffer)
	}
	
	#[inline(always)]
	fn check_flow_label_matches(&self, internet_protocol_version_6_packet_header: &InternetProtocolVersion6PacketHeader) -> Result<(), ()>
	{
		if likely!(internet_protocol_version_6_packet_header.flow_label() == self.invariant_flow_label)
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn check_differentiated_service_code_point_and_explicit_congestion_notification_matches(&mut self, internet_protocol_version_6_packet_header: &InternetProtocolVersion6PacketHeader) -> Result<(), ()>
	{
		let (differentiated_service_code_point, explicit_congestion_notification) = internet_protocol_version_6_packet_header.traffic_class();
		
		if unlikely!(differentiated_service_code_point != self.invariant_differentiated_service_code_point)
		{
			return Err(())
		}
		
		use self::ExplicitCongestionNotification::*;
		
		match (self.current_knowledge_of_explicit_congestion_notification, explicit_congestion_notification)
		{
			(NotCapableTransport, NotCapableTransport) => (),
			(CapableTransportEctZero, CapableTransportEctZero) => (),
			(CapableTransportEctOne, CapableTransportEctOne) => (),
			(CongestionEncountered, CongestionEncountered) => (),
			
			(CapableTransportEctZero, CongestionEncountered) =>
			{
				self.was_congestion_encountered = true
			}
		
			(CapableTransportEctOne, CongestionEncountered) =>
			{
				self.was_congestion_encountered = true
			}
			
			(CongestionEncountered, CapableTransportEctZero) =>
			{
				self.current_knowledge_of_explicit_congestion_notification = CapableTransportEctZero
			}
			
			(CongestionEncountered, CapableTransportEctOne) =>
			{
				self.current_knowledge_of_explicit_congestion_notification = CapableTransportEctOne
			}
			
			_ => return Err(())
		}
	}
	
	#[inline(always)]
	fn first_fragment(&mut self, fragment_list_node_to_insert: FragmentListNode, is_last_fragment: bool, fragment_details: InternetProtocolVersion6FirstFragmentDetails) -> Result<(), ()>
	{
		if fragment_list_node_to_insert.is_first_fragment()
		{
			let already_has_first_fragment = self.first_fragment_details.is_some();
			
			if unlikely!(already_has_first_fragment)
			{
				return Err(())
			}
			
			if unlikely!(is_last_fragment)
			{
				return Err(())
			}
			
			self.first_fragment_details = fragment_details;
			
			fragment_list_node_to_insert.set_right(self.leftmost);
			self.leftmost.set_left(fragment_list_node_to_insert);
			self.leftmost = fragment_list_node_to_insert;
			
			Ok(true)
		}
		
		Ok(false)
	}
	
	#[inline(always)]
	fn end_fragment(&mut self, fragment_list_node_to_insert: FragmentListNode, is_last_fragment: bool) -> Result<bool, ()>
	{
		if is_end_fragment
		{
			let duplicate_end_fragment = self.has_end_fragment;
			if unlikely!(duplicate_end_fragment)
			{
				return Err(())
			}
			self.has_end_fragment = true;
			
			fragment_list_node_to_insert.set_left(self.rightmost);
			self.rightmost.set_right(fragment_list_node_to_insert);
			self.rightmost = fragment_list_node_to_insert;
			
			Ok(true)
			
		}
		Ok(false)
	}
	
	#[inline(always)]
	fn insert_node_walking_from_right_to_left(&mut self, fragment_list_node_to_insert: FragmentListNode) -> Result<(), ()>
	{
		let mut current = self.rightmost;
		loop
		{
			if current.is_to_left_of(fragment_list_node_to_insert)?
			{
				let right_of_current = current.right();
				if let Some(right_of_current) = right_of_current
				{
					fragment_list_node_to_insert.set_right(right_of_current);
					right_of_current.set_left(fragment_list_node_to_insert);
				}
				else
				{
					self.rightmost = fragment_list_node_to_insert;
				}
				current.set_right(fragment_list_node_to_insert);
				fragment_list_node_to_insert.set_left(current);
			}
			else
			{
				let left_of_current = current.left();
				if let Some(left_of_current) = left_of_current
				{
					current = left_of_current;
					continue
				}
				else
				{
					fragment_list_node_to_insert.set_right(current);
					current.set_left(fragment_list_node_to_insert);
					
					self.leftmost = fragment_list_node_to_insert;
					
					return Ok(())
				}
			}
		}
	}
	
	#[inline(always)]
	fn complete(&self) -> bool
	{
		if self.has_end_fragment
		{
			self.fragmented_data_in_bytes == self.rightmost.unwrap().fields().end() as usize
		}
		else
		{
			false
		}
	}
}
