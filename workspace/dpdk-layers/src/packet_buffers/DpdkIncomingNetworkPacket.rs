// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Wraps a non-null DPDK packet buffer, `rte_mbuf`.
///
/// Note that without segmented (chained) buffers, `pkt_len` is always the same as `data_len`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkIncomingNetworkPacket(NonNull<rte_mbuf>);

impl From<NonNull<rte_mbuf>> for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn from(value: NonNull<rte_mbuf>) -> Self
	{
		DpdkIncomingNetworkPacket(value)
	}
}

impl TryFrom<*const rte_mbuf> for DpdkIncomingNetworkPacket
{
	type Error = ();
	
	#[inline(always)]
	fn from(value: *const rte_mbuf) -> Result<Self, Self::Error>
	{
		if value.is_null()
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(value as *mut rte_mbuf) })
		}
	}
}

impl TryFrom<*mut rte_mbuf> for DpdkIncomingNetworkPacket
{
	type Error = ();
	
	#[inline(always)]
	fn from(value: *mut rte_mbuf) -> Result<Self, Self::Error>
	{
		if value.is_null()
		{
			Err(())
		}
		else
		{
			Ok(DpdkIncomingNetworkPacket(unsafe { NonNull::new_unchecked(value) }))
		}
	}
}

impl<'a> Into<&'a rte_mbuf> for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn into(self) -> &'a rte_mbuf
	{
		self.0.reference()
	}
}

impl<'a> Into<&'a mut2 rte_mbuf> for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn into(self) -> &'a mut rte_mbuf
	{
		self.0.mutable_reference()
	}
}

impl Into<NonNull<rte_mbuf>> for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn into(self) -> NonNull<rte_mbuf>
	{
		self.0
	}
}

impl Into<*mut rte_mbuf> for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn into(self) -> *mut rte_mbuf
	{
		self.as_ptr()
	}
}

impl Into<*const rte_mbuf> for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn into(self) -> *const rte_mbuf
	{
		self.as_ptr() as *const _
	}
}

impl IncomingNetworkPacket for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn free_direct_contiguous_packet(self)
	{
		self.raw_free()
	}
	
	#[inline(always)]
	fn packet_length_if_contiguous(self) -> u16
	{
		self.debug_assert_is_contiguous();
		
		self.data_length()
	}
	
	#[inline(always)]
	fn offset_into_data<T>(self, offset: usize) -> NonNull<T>
	{
		debug_assert!(offset + size_of::<T>() <= (self.packet_length_if_contiguous() as usize), "offset with size of T exceeds packet length");
		
		let packet = { self.reference() };
		let pointer = ((packet.buf_addr as usize) + (self.segment_buffer_reserved_head_room() as usize) + offset) as *mut T;
		unsafe { NonNull::new_unchecked(pointer) }
	}
	
	#[inline(always)]
	fn hardware_offload_layer_2_packet_type(self) -> HardwareOffloadLayer2PacketType
	{
		use self::HardwareOffloadLayer2PacketType::*;
		use self::HardwareOffloadCategorisedLayer2PacketType::*;
		
		// Only bits 3:0 (0x0F) are significant.
		match self.hardware_offload_packet_type() & RTE_PTYPE_L2_MASK
		{
			RTE_PTYPE_UNKNOWN => Unknown,
			
			RTE_PTYPE_L2_ETHER => Ethernet(None),
			
			RTE_PTYPE_L2_ETHER_TIMESYNC => Ethernet(Some(Ieee1588TimeSync)),
			
			RTE_PTYPE_L2_ETHER_ARP => Ethernet(Some(AddressResolutionProtocol)),
			
			RTE_PTYPE_L2_ETHER_LLDP => Ethernet(Some(LinkLayerDiscoveryProtocol)),
			
			RTE_PTYPE_L2_ETHER_NSH => Ethernet(Some(NetworkServiceHeader)),
			
			RTE_PTYPE_L2_ETHER_VLAN => Ethernet(Some(VirtualLan)),
			
			RTE_PTYPE_L2_ETHER_QINQ => Ethernet(Some(QinQVirtualLan)),
			
			RTE_PTYPE_L2_ETHER_PPPOE => Ethernet(Some(PPPoE)),
			
			_ => Other
		}
	}
	
	#[inline(always)]
	fn hardware_offload_layer_3_packet_type(self) -> HardwareOffloadLayer3PacketType
	{
		use self::HardwareOffloadLayer3PacketType::*;
		use self::HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType::*;
		
		// Only bits 7:4 (0xF0) are significant.
		match self.hardware_offload_packet_type() & RTE_PTYPE_L3_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_L3_IPV4 => InternetProtocolVersion4(NotPresent),
			
			RTE_PTYPE_L3_IPV4_EXT => InternetProtocolVersion4(Present),
			
			RTE_PTYPE_L3_IPV4_EXT_UNKNOWN => InternetProtocolVersion4(PresentAndUnrecognised),
			
			RTE_PTYPE_L3_IPV6 => InternetProtocolVersion6(NotPresent),
			
			RTE_PTYPE_L3_IPV6_EXT => InternetProtocolVersion6(Present),
			
			RTE_PTYPE_L3_IPV6_EXT_UNKNOWN => InternetProtocolVersion6(PresentAndUnrecognised),
			
			_ => Other
		}
	}
	
	#[inline(always)]
	fn hardware_offload_layer_4_packet_type(self) -> HardwareOffloadLayer4PacketType
	{
		use self::HardwareOffloadLayer4PacketType::*;
		
		// Only bits 11:8 (0x0F00) are significant.
		match self.hardware_offload_packet_type() & RTE_PTYPE_L4_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_L4_TCP => TransmissionControlProtocol,
			
			RTE_PTYPE_L4_UDP => UserDatagramProtocol,
			
			RTE_PTYPE_L4_FRAG => Fragmented,
			
			RTE_PTYPE_L4_SCTP => StreamControlTransmissionProtocol,
			
			RTE_PTYPE_L4_ICMP => InternetControlMessageProtocol,
			
			RTE_PTYPE_L4_NONFRAG => OtherNotAFragment,
			
			_ => Other
		}
	}
	
	#[inline(always)]
	fn hardware_offload_tunnel_packet_type(self) -> HardwareOffloadTunnelPacketType
	{
		use self::HardwareOffloadTunnelPacketType::*;
		
		// Only bits 15:12 (0xF000) are significant.
		match self.hardware_offload_packet_type() & RTE_PTYPE_TUNNEL_MASK
		{
			0 => Uncategorised,
			
			RTE_PTYPE_TUNNEL_IP => InternetProtocol,
			
			RTE_PTYPE_TUNNEL_GRE => GenericRoutingEncapsulation,
			
			RTE_PTYPE_TUNNEL_VXLAN => VirtualExtensibleLocalAreaNetwork,
			
			RTE_PTYPE_TUNNEL_NVGRE => NetworkVirtualizationUsingGenericRoutingEncapsulation,
			
			RTE_PTYPE_TUNNEL_GENEVE => GenericNetworkVirtualizationEncapsulation,
			
			RTE_PTYPE_TUNNEL_GRENAT => TeredoOrGenericRoutingEncapsulationOrVirtualExtensibleLocalAreaNetwork,
			
			RTE_PTYPE_TUNNEL_GTPC => GprsTunnelingProtocolControl,
			
			RTE_PTYPE_TUNNEL_GTPU => GprsTunnelingProtocolUserData,
			
			RTE_PTYPE_TUNNEL_ESP => InternetProtocolEncapsulatingSecurityPayload,
			
			RTE_PTYPE_TUNNEL_L2TP => Layer2TunnelingProtocol,
			
			RTE_PTYPE_TUNNEL_VXLAN_GPE => VirtualExtensibleLocalAreaNetworkGenericProtocolExtension,
			
			RTE_PTYPE_TUNNEL_MPLS_IN_UDP => MultiprotocolLabelSwitchingInGenericRoutingEncapsulation,
			
			RTE_PTYPE_TUNNEL_MPLS_IN_GRE => MultiprotocolLabelSwitchingInUserDatagramProtocol,
			
			_ => Other
		}
	}
	
	#[inline(always)]
	fn hardware_offload_tunnel_inner_layer_2_packet_type(self) -> HardwareOffloadLayer2PacketType
	{
		use self::HardwareOffloadLayer2PacketType::*;
		use self::HardwareOffloadCategorisedLayer2PacketType::*;
		
		// Only bits 19:16 are significant.
		match self.hardware_offload_packet_type() & RTE_PTYPE_INNER_L2_MASK
		{
			0 => Unknown,
			
			RTE_PTYPE_INNER_L2_ETHER => Ethernet(None),
			
			RTE_PTYPE_INNER_L2_ETHER_VLAN => Ethernet(Some(VirtualLan)),
			
			RTE_PTYPE_INNER_L2_ETHER_QINQ => Ethernet(Some(QinQVirtualLan)),
			
			_ => Ethernet(Some(Other))
		}
	}
	
	#[inline(always)]
	fn hardware_offload_tunnel_inner_layer_3_packet_type(self) -> HardwareOffloadLayer3PacketType
	{
		use self::HardwareOffloadLayer3PacketType::*;
		use self::HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType::*;
		
		// Only bits 23:20 are significant.
		match self.hardware_offload_packet_type() & RTE_PTYPE_INNER_L3_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_INNER_L3_IPV4 => InternetProtocolVersion4(NotPresent),
			
			RTE_PTYPE_INNER_L3_IPV4_EXT => InternetProtocolVersion4(Present),
			
			RTE_PTYPE_INNER_L3_IPV4_EXT_UNKNOWN => InternetProtocolVersion4(PresentAndUnrecognised),
			
			RTE_PTYPE_INNER_L3_IPV6 => InternetProtocolVersion6(NotPresent),
			
			RTE_PTYPE_INNER_L3_IPV6_EXT => InternetProtocolVersion6(Present),
			
			RTE_PTYPE_INNER_L3_IPV6_EXT_UNKNOWN => InternetProtocolVersion6(PresentAndUnrecognised),
			
			_ => Other,
		}
	}
	
	#[inline(always)]
	fn hardware_offload_tunnel_inner_layer_4_packet_type(self) -> HardwareOffloadLayer4PacketType
	{
		use self::HardwareOffloadLayer4PacketType::*;
		
		// Only bits 27:24 are significant.
		match self.hardware_offload_packet_type() & RTE_PTYPE_INNER_L4_MASK
		{
			0 => UncategorisedOrAbsent,
			
			RTE_PTYPE_INNER_L4_TCP => TransmissionControlProtocol,
			
			RTE_PTYPE_INNER_L4_UDP => UserDatagramProtocol,
			
			RTE_PTYPE_INNER_L4_FRAG => Fragmented,
			
			RTE_PTYPE_INNER_L4_SCTP => StreamControlTransmissionProtocol,
			
			RTE_PTYPE_INNER_L4_ICMP => InternetControlMessageProtocol,
			
			RTE_PTYPE_INNER_L4_NONFRAG => OtherNotAFragment,
			
			_ => Other
		}
	}
	
	#[inline(always)]
	fn hardware_offload_internet_protocol_version_4_check_sum_status(self) -> HardwareOffloadCheckSumStatus
	{
		self.receive_offload_flags().internet_protocol_version_4_check_sum_status()
	}
	
	#[inline(always)]
	fn hardware_offload_layer_4_check_sum_status(self) -> HardwareOffloadCheckSumStatus
	{
		self.receive_offload_flags().layer_4_check_sum_status()
	}
	
	#[inline(always)]
	fn hardware_offload_categorisation_indicates_an_unwanted_packet(self) -> bool
	{
		self.receive_offload_flags().hardware_offloading_categorisation_indicates_an_unwanted_packet()
	}
}

impl EthernetIncomingNetworkPacket for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn was_vlan_tag_control_information_stripped(self) -> bool
	{
		self.receive_offload_flags().was_vlan_tag_control_information_stripped()
	}
	
	#[inline(always)]
	fn stripped_vlan_tag_control_information(self) -> TagControlInformation
	{
		TagControlInformation(NetworkEndianU16::from_network_endian(self.reference().vlan_tci))
	}
	
	#[inline(always)]
	fn was_vlan_qinq_tag_control_information_stripped(self) -> bool
	{
		self.receive_offload_flags().was_vlan_qinq_tag_control_information_stripped()
	}
	
	#[inline(always)]
	fn stripped_vlan_qinq_tag_control_information(self) -> (TagControlInformation, TagControlInformation)
	{
		(TagControlInformation(NetworkEndianU16::from_network_endian(self.reference().vlan_tci_outer)), self.stripped_vlan_tag_control_information())
	}
}

impl PrintInformation for DpdkIncomingNetworkPacket
{
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		const FirstNBytesOfPacketData: usize = 0;
		unsafe { rte_pktmbuf_dump(stderr as *mut FILE, self.as_ptr(), FirstNBytesOfPacketData) }
	}
}

impl DpdkIncomingNetworkPacket
{
	/// Private data alignment.
	pub(crate) const AlignmentOfPrivateData: usize = RTE_MBUF_PRIV_ALIGN as usize;
	
	/// Normally 128 bytes, but a configuration value for DPDK.
	///
	/// Equivalent to `RTE_PKTMBUF_HEADROOM`.
	pub(crate) const HeadRoom: u16 = RTE_PKTMBUF_HEADROOM as u16;
	
	/// Some NICs need at least a 2KB buffer to receive a standard Ethernet frame without splitting it into multiple segments.
	///
	/// Equivalent to `RTE_MBUF_DEFAULT_DATAROOM`.
	pub(crate) const DefaultDataRoom: u16 = RTE_MBUF_DEFAULT_DATAROOM as u16;
	
	/// Some NICs need at least a 2KB buffer to receive a standard Ethernet frame without splitting it into multiple segments.
	///
	/// For PacketBuffers used for receive or transmit, this is the minimal recommended buffer length.
	///
	/// Equivalent to `RTE_MBUF_DEFAULT_BUF_SIZE`.
	pub(crate) const DefaultBufferLength: u16 = Self::buffer_length(Self::DefaultDataRoom);
	
	/// Maximum number of segment buffers in a packet buffer.
	pub(crate) const MaximumNumberOfSegmentBuffers: u16 = RTE_MBUF_MAX_NB_SEGS as u16;
	
	/// Creates a new wrapper.
	#[inline(always)]
	pub(crate) fn from_possibly_null_rte_mbuf(value: *mut rte_mbuf) -> Option<Self>
	{
		if value.is_null()
		{
			None
		}
		else
		{
			Some(DpdkIncomingNetworkPacket(unsafe { NonNull::new_unchecked(value) }))
		}
	}
	
	/// Checks if this packet is contiguous.
	#[inline(always)]
	fn debug_assert_is_contiguous(self)
	{
		debug_assert!(self.is_contiguous(), "Inbound packets should be contiguous; scatter receive (`offloads::DEV_RX_OFFLOAD_SCATTER`) is not supported. To make sure not packets are dropped by poll-mode drivers (PMDs) in this mode, the value of `data_room_size` passed to rte_pktmbuf_pool_create() is at least Self::data_room_size_for_packet_buffer_pool()");
	}
	
	/// A contiguous packet has only one segment, ie there is not a chain of segments.
	///
	/// Opposite of `is_segmented()`.
	#[inline(always)]
	fn is_contiguous(self) -> bool
	{
		self.number_of_segments() == 1
	}
	
	/// A segmented packet has more than one segment, ie there is a chain of segments.
	///
	/// Opposite of `is_contiguous()`.
	#[inline(always)]
	fn is_segmented(self) -> bool
	{
		self.number_of_segments() != 1
	}
	
	/// Number of segments.
	#[inline(always)]
	fn number_of_segments(self) -> u16
	{
		let number_of_segments = self.reference().nb_segs;
		debug_assert_ne!(number_of_segments, 0, "No segments!");
		number_of_segments
	}
	
	#[inline(always)]
	fn receive_offload_flags(self) -> PacketReceiveOffloadFeaturesFlags
	{
		unsafe { transmute(self.offload_flags()) }
	}
	
	#[inline(always)]
	fn offload_flags(self) -> u64
	{
		self.reference().ol_flags
	}
	
	/// Put packet back into its original packet buffer pool.
	///
	/// The caller must ensure that the mbuf is direct and properly reinitialized (`refcnt=1`, `next=NULL`, `nb_segs=1`), as done by `self.pre_free_segment()`.
	///
	/// This function should be used with care, when optimization is required.
	///
	/// For standard needs, prefer `self.free()` or `self.free_segment()`.
	#[inline(always)]
	fn raw_free(self)
	{
		self.debug_assert_is_contiguous();
		
		self.packet_buffer_pool_packet_allocated_from().put(self)
	}
	
	/// Parent packet buffer pool that allocated this packet.
	#[inline(always)]
	fn packet_buffer_pool_packet_allocated_from(self) -> PacketBufferPool
	{
		PacketBufferPool(unsafe { NonNull::new_unchecked(self.reference().pool) })
	}
	
	#[inline(always)]
	fn reference<'a>(self) -> &'a rte_mbuf
	{
		unsafe { & * self.as_ptr() }
	}
	
	#[inline(always)]
	fn mutable_reference<'a>(self) -> &'a mut rte_mbuf
	{
		unsafe { &mut * self.as_ptr() }
	}
	
	#[inline(always)]
	fn as_ptr(self) -> *mut rte_mbuf
	{
		self.0.as_ptr()
	}
	
	/// Constant function calculates buffer length from data room length.
	#[inline(always)]
	const fn buffer_length(data_room_length: u16) -> u16
	{
		data_room_length + Self::HeadRoom
	}
	
	/// Data length.
	///
	/// Amount of data 'payload' in segment buffer, always equal to or less than `segment_buffer_length()`.
	///
	/// Is equivalent to `self.segment_buffer_length() - self.segment_buffer_reserved_head_room() - self.segment_buffer_tail_room()`.
	///
	/// Also known as `data_len`.
	#[inline(always)]
	fn data_length(self) -> u16
	{
		self.reference().data_len
	}
	
	/// Packet length.
	///
	/// Is the sum of the `data_length()` of all segments.
	///
	/// Also known as `pkt_len`.
	#[inline(always)]
	fn length(self) -> u32
	{
		self.reference().pkt_len
	}
	
	/// Segment buffer length.
	///
	/// Also known as `buf_len`.
	///
	/// Size of this buffer.
	#[inline(always)]
	fn segment_buffer_length(self) -> u16
	{
		self.reference().buf_len
	}
	
	/// Head room.
	///
	/// The length of the part at the start of the segment buffer that is reserved for header data.
	///
	/// The actual data 'payload' starts after this offset in the segment buffer.
	#[inline(always)]
	fn segment_buffer_reserved_head_room(self) -> u16
	{
		self.reference().data_off
	}
	
	/// Tail room.
	///
	/// The amount of space (unused bytes) at the end of the segment buffer in this packet that could be used for data 'payload'.
	#[inline(always)]
	fn segment_buffer_tail_room(self) -> u16
	{
		let packet = self.reference();
		let tail_offset = self.segment_buffer_reserved_head_room() + self.data_length();
		self.segment_buffer_length() - tail_offset
	}
	
	/// Size of the application private data.
	///
	/// If this is an indirect PacketBuffer, it is the size of the parent direct PacketBuffer's application private data.
	#[inline(always)]
	fn private_size(self) -> u16
	{
		(self.reference()).priv_size
	}

//	/// Value of `data_room_size` passed to `rte_pktmbuf_pool_create()` to ensure that segmentation of receive packets is not needed and packet drops do not occur.
//	///
//	/// Use this value to avoid the need to specify `offloads::DEV_RX_OFFLOAD_SCATTER` for poll-mode drivers (PMDs).
//	#[inline(always)]
//	fn data_room_size_for_packet_buffer_pool(maximum_transmission_unit_size: MaximumTransmissionUnitSize) -> u16
//	{
//		maximum_transmission_unit_size.to_data_room_size_for_packet_buffer_pool()
//	}
//
//	/// Destroy this packet and return the memory it uses to its packet buffer pool (PacketBufferPool).
//	#[inline(always)]
//	fn free(self)
//	{
//		unsafe { rust_rte_pktmbuf_free(self.as_ptr()) };
//	}
//
//	/// Set this packet to be ignored.
//	#[inline(always)]
//	fn ignore(self)
//	{
//		*self.mutable_reference()._3.packet_type.as_mut() = RTE_PTYPE_UNKNOWN
//	}
}

impl DpdkIncomingNetworkPacket
{
	/// Returns the IO address that points to the start of the data in the packet.
	///
	/// Implements `rte_pktmbuf_iova`.
	#[inline(always)]
	pub fn io_virtual_address(self) -> rte_iova_t
	{
		self.io_virtual_address_offset(0)
	}
	
	/// Returns the IO address that points to an offset of the data in the packet.
	///
	/// Implements `rte_pktmbuf_iova_offset`.
	#[inline(always)]
	pub fn io_virtual_address_offset(self, offset: u64) -> rte_iova_t
	{
		let packet = self.reference();
		((*packet._1.buf_iova.as_ref()) + (self.segment_buffer_reserved_head_room() as u64) + offset) as rte_iova_t
	}
	
	/// Returns an error on overflow.
	#[inline(always)]
	pub fn chain_append_tail(&self, tail: Self) -> Result<(), ()>
	{
		Self::chain_together(self, tail)
	}
	
	/// Returns an error on overflow.
	#[inline(always)]
	pub fn chain_prepend_head(&self, head: Self) -> Result<(), ()>
	{
		Self::chain_together(head, self)
	}
	
	/// Chain together.
	#[inline(always)]
	pub fn chain_together(head: Self, tail: Self) -> Result<(), ()>
	{
		let result = unsafe { rust_rte_pktmbuf_chain(head.as_ptr(), tail.as_ptr()) };
		if likely!(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				NegativeE::EOVERFLOW => false,
				
				_ => panic!("Unexpected error code '{}' from rust_rte_rte_pktmbuf_chain()", result),
			}
		}
	}
	
	/// User data as a (possibly null) pointer.
	#[inline(always)]
	pub fn user_data_as_pointer<T>(self) -> *mut T
	{
		self.reference()._4.userdata as *mut T
	}
	
	/// User data as 8 native endian bytes.
	///
	/// Used for instance by the `rte_security` library.
	#[inline(always)]
	pub fn user_data_raw(self) -> u64
	{
		self.reference()._4.udata64
	}

	/// Current reference count.
	#[inline(always)]
	pub fn reference_count(self) -> u16
{
	self.reference().refcnt
}
	
	/// Adjust reference count by delta for all segments.
	#[inline(always)]
	pub fn adjust_reference_count_for_all_segments(self, delta: i16)
	{
		let mut m = self.as_ptr();
		while
		{
			unsafe { rust_rte_mbuf_refcnt_update(m, delta) };
			m = unsafe { & * m }.next;
			m.is_not_null()
		}
		{
		}
	}
	
	/// Optimized routine that only works on direct, contiguous packets with a reference count of 1.
	#[inline(always)]
	pub fn free_direct_contiguous_packet(self)
	{
		self.raw_free()
	}
	
	/// Put packet back into its original packet buffer pool.
	///
	/// The caller must ensure that the mbuf is direct and properly reinitialized (`refcnt=1`, `next=NULL`, `nb_segs=1`), as done by `self.pre_free_segment()`.
	///
	/// This function should be used with care, when optimization is required.
	///
	/// For standard needs, prefer `self.free()` or `self.free_segment()`.
	#[inline(always)]
	pub fn raw_free(self)
	{
		debug_assert_ne!(self.is_indirect_attached_packet_buffer(), "This is an indirect packet");
		self.debug_assert_is_contiguous();
		debug_assert_eq!(self.reference_count(), 1, "Has a reference count which is not 1");
		
		self.packet_buffer_pool_packet_allocated_from().put(self.as_ptr())
	}
	
	/// Decreases reference counter and unlinks a mbuf segment.
	///
	/// This function does the same than a free, except that it does not return the segment to its packet buffer pool.
	///
	/// It decreases the reference counter, and if it reaches 0, it is detached from its parent for an indirect mbuf.
	///
	/// Returns Some(self) if is the last reference, which can be recycled of freed. Otherwise returns None if the reference count is not zero.
	#[inline(always)]
	pub fn pre_free_segment(self) -> Option<PacketBuffer>
	{
		let result = unsafe { rust_rte_pktmbuf_prefree_seg(self.as_ptr()) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(result) })
		}
	}
	
	/// Free a segment of a packet into its original packet buffer pool.
	///
	/// Does so without parsing other segments in the case of chained buffers.
	#[inline(always)]
	pub fn free_segment(self)
	{
		unsafe { rust_rte_pktmbuf_free_seg(self.as_ptr()) }
	}
	
	/// Next segment.
	#[inline(always)]
	pub fn next_segment(self) -> Option<Self>
	{
		let next = self.reference().next;
		if next.is_null()
		{
			None
		}
		else
		{
			Some(DpdkIncomingNetworkPacket(unsafe { NonNull::new_unchecked(next) }))
		}
	}
	
	/// Last segment.
	#[inline(always)]
	pub fn last_segment(self) -> Option<Self>
	{
		let last = unsafe { rust_rte_pktmbuf_lastseg(self.as_ptr()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkIncomingNetworkPacket(unsafe { NonNull::new_unchecked(last) }))
		}
	}
	
	/// Finds a segment at the offset given.
	///
	/// Returns the found segment and the reduced offset, ie as if offset from found segment.
	///
	/// Panics if `offset + buffer.len() > self.length()`.
	#[inline(always)]
	pub fn find_segment_at_offset(self, mut offset: u32) -> (NonNull<rte_mbuf>, u32)
	{
		let mut segment = self;
		while offset >= segment.data_length()
		{
			offset -= segment.data_length();
			segment = segment.next_segment().expect("Number of segments x each segment length != packet length; violation of expected packet state");
		}
		(segment, offset)
	}
	
	/// Writes (copies) `buffer.len()` data bytes into the packet from `buffer`, regardless of how many segments it has starting at the packet at `offset`.
	///
	/// Panics if `offset + buffer.len() > self.length()`.
	///
	/// `copy_from` must not overlap with the packet's data.
	#[inline(always)]
	pub fn write_even_if_non_contiguous(self, offset: u32, copy_from: &[u8])
	{
		let length = copy_from.len() as u32;
		
		debug_assert!(offset + length <= self.length(), "offset '{}' + copy_from.len() '{}' exceeds packet length '{}'", offset, length, self.length());
		
		if offset + length <= (self.data_length() as u32)
		{
			let destination = self.offset_into_data(self, offset);
			unsafe { rust_rte_memcpy(destination, copy_from.as_ptr() as *const c_void, length as usize) };
			return
		}
		
		let (mut segment, mut offset) = self.find_segment_at_offset(offset);
		
		if offset + length <= (segment.data_length() as u32)
		{
			let destination = segment.offset_into_data(offset);
			unsafe { rust_rte_memcpy(destination, copy_from.as_ptr() as *const c_void, length as usize) };
			return
		}
		
		let mut copy_from_offset: u32 = 0;
		let mut remaining_length = length;
		while remaining_length > 0
		{
			let length_of_this_copy =
			{
				let maximum_copy_length = (segment.data_length() as u32) - offset;
				
				if maximum_copy_length > remaining_length
				{
					remaining_length
				}
				else
				{
					maximum_copy_length
				}
			};
			
			{
				let destination = segment.offset_into_data(offset as usize);
				let copy_from_pointer = unsafe { copy_from.get_unchecked(copy_from_offset) };
				unsafe { rust_rte_memcpy(destination, copy_from_pointer as *const c_void, length_of_this_copy as usize) };
			}
			
			offset = 0;
			
			segment = segment.next_segment();
			remaining_length -= length_of_this_copy;
			copy_from_offset += length_of_this_copy;
		}
		
		Ok(())
	}
	
	/// This function moves the data into the first segment if there is enough tail room.
	///
	/// In effect, compaction to try to make a Packet Buffer contiguous.
	///
	/// The subsequent segments are unchained and freed.
	#[inline(always)]
	pub fn linearize(self)
	{
		unsafe { rust_rte_pktmbuf_linearize(self.as_ptr()) }
	}
	
	/// Packet (*not TCP*) sequence number.
	///
	/// Used for re-ordering out-of-order packets, typically when packets are being received by multiple threads.
	/// In this case, the sequence number can be a global atomically incremented counter.
	///
	/// See `ReorderingBuffer`.
	#[inline(always)]
	pub fn sequence_number(self) -> u32
	{
		self.reference().seqn
	}
	
	/// Packet (*not TCP*) sequence number.
	///
	/// Used for re-ordering out-of-order packets, typically when packets are being received by multiple threads.
	/// In this case, the sequence number can be a global atomically incremented counter.
	///
	/// See `ReorderBuffer`.
	#[inline(always)]
	pub fn set_sequence_number(self, sequence_number: u32)
	{
		self.mutable_reference().seqn = sequence_number
	}
	
	/// Clone.
	///
	/// Allocates the clone from `packet_buffer_pool`.
	#[inline(always)]
	pub fn clone(&self, packet_buffer_pool: PacketBufferPool) -> Result<Self, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_clone(self.as_ptr(), packet_buffer_pool.as_ptr()) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(DpdkIncomingNetworkPacket(unsafe { NonNull::new_unchecked(result) }))
		}
	}
	
	/// Reset the fields of a packet to their default values; allows re-use.
	///
	/// The packet must have only one segment.
	#[inline(always)]
	pub fn reset(self)
	{
		unsafe { rust_rte_pktmbuf_reset(self.as_ptr()) }
	}
	
	/// Resets head room length to the minimum of `Self::HeadRoom` and `self.segment_buffer_length()`.
	///
	/// The packet must have only one segment.
	///
	/// Does not move any data at all, so all data (eg headers) will be invalid after this.
	///
	/// Returns the new segment buffer reserved head room.
	#[inline(always)]
	pub fn reset_segment_buffer_reserved_head_room(self) -> u16
	{
		let head_room = min(Self::HeadRoom, self.segment_buffer_length());
		self.mutable_reference().data_off = head_room;
		
		head_room
	}
	
	/// Attach packet `attach_as_indirect` to this one.
	///
	/// After attachment we refer to the packet buffer we attached as 'indirect', while we refer to the mbuf we attached to as 'direct'.
	///
	/// The direct mbuf's reference counter is incremented.
	///
	/// Currently the following are not supported:-
	/// * `attach_as_indirect` is already indirectly attached.
	/// * `attach_as_indirect` is used by someone else (its reference counter is greater then 1).
	#[inline(always)]
	pub fn attach(self, attach_as_indirect: Self)
	{
		unsafe { rust_rte_pktmbuf_attach(attach_as_indirect.as_ptr(), self.as_ptr()) }
	}
	
	/// Detach an indirect packet.
	///
	/// * restore original mbuf address and length values.
	/// * reset pktmbuf data and data_len to their default values.
	/// * decrement the direct mbuf's reference counter.
	///
	/// When the reference counter becomes 0, the direct mbuf is freed.
	///
	/// All other fields of the given packet will be left intact.
	#[inline(always)]
	pub fn detach_indirect_packet(self)
	{
		unsafe { rust_rte_pktmbuf_detach(self.as_ptr()) }
	}
	
	/// Prepend `length` bytes to the packet and return a pointer to the start address of the added data.
	///
	/// If there is not enough head room in the first segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	pub fn prepend(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_prepend(self.as_ptr(), length) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result as *mut u8) })
		}
	}
	
	/// Append `length` bytes to the packet and return a pointer to the start address of the added data.
	///
	/// If the `length` is greater than the length of the last segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	pub fn append(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_append(self.as_ptr(), length) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result as *mut u8) })
		}
	}
	
	/// Remove `length` bytes at the beginning of a packet and return a pointer to the start address of the new data area.
	///
   /// If the `length` is greater than the length of the first segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	pub fn remove(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_adj(self.as_ptr(), length) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result as *mut u8) })
		}
	}
	
	/// Remove `length` bytes of data at the end of the packet.
	///
	/// If the `length` is greater than the length of the last segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	pub fn trim(self, length: u16) -> Result<(), ()>
	{
		let result = unsafe { rust_rte_pktmbuf_trim(self.as_ptr(), length) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if result == -1
		{
			Err(())
		}
		else
		{
			panic!("Unexpected result")
		}
	}
	
	/// Validate that the packet's fields are correctly set for transmit offload.
	#[inline(always)]
	pub fn validate_transmit_offload(self) -> Result<(), PosixErrorNumber>
	{
		let result = unsafe { rust_rte_validate_tx_offload(self.as_ptr() as *const _) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result < 0)
		{
			Err(-result)
		}
		else
		{
			panic!("Invalid result '{}' from rust_rte_validate_tx_offload()", result)
		}
	}
	
	/// Is the hash a receive side scaling hash?
	#[inline(always)]
	pub fn has_receive_side_scaling_hash(self) -> bool
	{
		self.has_offload_flag(PKT_RX_RSS_HASH)
	}
	
	/// Receive side scaling hash.
	///
	/// Only valid if `self.has_receive_side_scaling_hash()` is true.
	#[inline(always)]
	pub fn hash_as_receive_side_scaling_hash(self) -> u32
	{
		* self.reference().rss.as_ref()
	}
	
	/// Is the hash a flow director filter identifier?
	#[inline(always)]
	pub fn has_flow_director_filter_identifier(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR)
	}
	
	/// Flow director filter identifier.
	#[inline(always)]
	pub fn hash_as_flow_director_filter_identifier(self) -> u32
	{
		let flow_director = self.reference().fdir.as_ref();
		flow_director.hi
	}
	
	/// Is the hash a flow director hash and filter identifier?
	#[inline(always)]
	pub fn has_flow_director_hash_and_filter_identifier(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR_ID)
	}
	
	/// Flow director hash and filter identifier.
	#[inline(always)]
	pub fn hash_as_flow_director_hash_and_filter_identifier(self) -> (u16, u16)
	{
		let flow_director = self.reference().fdir.as_ref();
		let hash_and_identifier = flow_director._1._1.as_ref();
		(hash_and_identifier.hash, hash_and_identifier.id)
	}
	
	/// Is the hash a flow director flexible bytes?
	#[inline(always)]
	pub fn has_flow_director_flexible_bytes_high_and_low(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR_FLX)
	}
	
	/// Flow director flexible bytes.
	#[inline(always)]
	pub fn hash_as_flow_director_flexible_bytes_high_and_low(self) -> (u32, u32)
	{
		let flow_director = self.reference().fdir.as_ref();
		(flow_director.hi, flow_director._1.lo)
	}
	
	/// Hierarchical Scheduler flexible bytes.
	#[inline(always)]
	pub fn hash_as_hierarchical_scheduler_bytes_high_and_low(self) -> (u32, u32)
	{
		let scheduler = * self.reference().sched;
		(scheduler.hi, scheduler.lo)
	}
	
	/// User define tags.
	///
	/// See `rte_distributor_process()`.
	#[inline(always)]
	pub fn hash_as_user_defined_tags(self) -> u32
	{
		* self.reference().usr.as_ref()
	}

	/// Raw hardware packet type.
	#[inline(always)]
	fn hardware_offload_packet_type(self) -> u32
	{
		unsafe { *self.reference()._3.packet_type.as_ref() }
	}

	/// Returns a Layer 2 name for this hardware packet type.
	///
	/// If this is a tunneled packet, then this is known as the Outer Layer 2 name.
	///
	/// * All names start `L2_`.
	/// * If unknown, name will be `L2_UNKNOWN`; this occurs for invalid packet type flags.
	/// * If known but not further categorised, name will be `L2_ETHER`.
	/// * If the hardware identified a particular EtherType, then the name will be one of:-
	///   * `L2_ETHER_TIMESYNC`
	///   * `L2_ETHER_ARP`
	///   * `L2_ETHER_LLDP`
	///   * `L2_ETHER_NSH`
	///   * `L2_ETHER_VLAN`
	///   * `L2_ETHER_QINQ`
	///   * `L2_ETHER_PPPOE`
	#[inline(always)]
	pub fn hardware_offload_layer_2_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l2_name(self.hardware_packet_type())) }
	}

	/// Returns a Layer 3 name for this packet type.
	///
	/// If this is a tunneled packet, then this is known as the Outer Layer 3 name.
	///
	/// * All names start `L3_`.
	/// * If unknown, name will be `L3_UNKNOWN`; this occurs for invalid packet type flags.
	/// * If known, name will start with either `L3_IPV6` or `L3_IPV6`.
	/// * Other names are:-
	///   * `L3_IPV4`
	///   * `L3_IPV4_EXT`
	///   * `L3_IPV4_EXT_UNKNOWN`
	///   * `L3_IPV6`
	///   * `L3_IPV6_EXT`
	///   * `L3_IPV6_EXT_UNKNOWN`
	#[inline(always)]
	fn hardware_offload_layer_3_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l3_name(self.hardware_packet_type())) }
	}

	/// Returns a Layer 4 name for this packet type.
	///
	/// If this is a tunneled packet, then this is known as the Outer Layer 4 name.
	///
	/// * All names start `L4_`.
	/// * If unknown or not a layer 4 packet, name will be `L4_UNKNOWN`; this also occurs for invalid packet type flags.
	/// * Other names are:-
	///   * `L4_ICMP`
	///   * `L4_UDP`
	///   * `L4_TCP`
	///   * `L4_SCTP`
	///   * `L4_FRAG`
	///   * `L4_NONFRAG`
	#[inline(always)]
	fn hardware_offload_layer_4_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.hardware_packet_type())) }
	}

	/// Returns a tunnel name for this packet type.
	///
	/// * All names start `TUNNEL_`.
	/// * If unknown or not a tunnel, name will be `TUNNEL_UNKNOWN`; this also occurs for invalid packet type flags.
	/// * Other names are:-
	///   * `TUNNEL_IP`
	///   * `TUNNEL_GRE`
	///   * `TUNNEL_VXLAN`
	///   * `TUNNEL_NVGRE`
	///   * `TUNNEL_GENEVE`
	///   * `TUNNEL_GRENAT`
	///   * `TUNNEL_GTPC`
	///   * `TUNNEL_GTPU`
	///   * `TUNNEL_ESP`
	///   * `TUNNEL_L2TP`
	///   * `TUNNEL_VXLAN_GPE`
	///   * `TUNNEL_MPLS_IN_UDP`
	///   * `TUNNEL_MPLS_IN_GRE`
	///
	#[inline(always)]
	fn hardware_offload_tunnel_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_tunnel_name(self.hardware_packet_type())) }
	}

	/// Returns a Tunnel Inner Layer 2 name for this hardware packet type.
	///
	/// * All names start `INNER_L2_`.
	/// * If unknown, name will be `INNER_L2_UNKNOWN`; this occurs for invalid packet type flags.
	/// * If known but not further categorised, name will be `INNER_L2_ETHER`.
	/// * If the hardware identified a particular EtherType, then the name will be one of:-
	///   * `INNER_L2_ETHER_VLAN`
	///   * `INNER_L2_ETHER_QINQ`
	#[inline(always)]
	fn hardware_offload_tunnel_inner_layer_2_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_inner_l2_name(self.hardware_packet_type())) }
	}

	/// Returns a Tunnel Inner Layer 3 name for this packet type.
	///
	/// * All names start `INNER_L3_`.
	/// * If unknown, name will be `INNER_L3_UNKNOWN`; this occurs for invalid packet type flags.
	/// * Other names are:-
	///   * `INNER_L3_IPV4`
	///   * `INNER_L3_IPV4_EXT`
	///   * `INNER_L3_IPV4_EXT_UNKNOWN`
	///   * `INNER_L3_IPV6`
	///   * `INNER_L3_IPV6_EXT`
	///   * `INNER_L3_IPV6_EXT_UNKNOWN`
	#[inline(always)]
	fn hardware_offload_tunnel_inner_layer_3_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_inner_l3_name(self.hardware_packet_type())) }
	}

	/// Returns a Tunnel Inner Layer 4 name for this packet type.
	///
	/// * All names start `INNER_L4_`.
	/// * If unknown or not a layer 4 packet, name will be `INNER_L4_UNKNOWN`; this also occurs for invalid packet type flags.
	/// * Other names are:-
	///   * `INNER_L4_ICMP`
	///   * `INNER_L4_UDP`
	///   * `INNER_L4_TCP`
	///   * `INNER_L4_SCTP`
	///   * `INNER_L4_FRAG`
	///   * `INNER_L4_NONFRAG`
	#[inline(always)]
	fn hardware_offload_tunnel_inner_layer_4_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.hardware_packet_type())) }
	}
}
