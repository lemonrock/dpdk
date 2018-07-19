// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Wraps a non-null DPDK packet buffer, `rte_mbuf`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PacketBuffer(NonNull<rte_mbuf>);

impl From<NonNull<rte_mbuf>> for PacketBuffer
{
	#[inline(always)]
	fn from(value: NonNull<rte_mbuf>) -> Self
	{
		PacketBuffer(value)
	}
}

impl Into<NonNull<rte_mbuf>> for PacketBuffer
{
	#[inline(always)]
	fn into(self) -> NonNull<rte_mbuf>
	{
		self.0
	}
}

impl Into<*mut rte_mbuf> for PacketBuffer
{
	#[inline(always)]
	fn into(self) -> *mut rte_mbuf
	{
		self.as_ptr()
	}
}

impl PacketBuffer
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
			Some(PacketBuffer(unsafe { NonNull::new_unchecked(value) }))
		}
	}
	
	/// Packet length if contiguous.
	///
	/// Same as `data_length()`.
	#[inline(always)]
	pub(crate) fn packet_length_if_contiguous(self) -> u16
	{
		self.debug_assert_is_contiguous();
		
		self.data_length()
	}
	
	/// Packet length less ethernet header.
	#[inline(always)]
	pub(crate) fn packet_length_if_contiguous_less_ethernet_packet_header(self) -> u16
	{
		self.packet_length_if_contiguous() - EthernetPacketHeader::SizeU16
	}
	
	/// Is too short to be an ethernet packet?
	#[inline(always)]
	pub(crate) fn is_too_short_to_be_an_ethernet_packet(self) -> bool
	{
		self.packet_length_if_contiguous() < EthernetPacketHeader::SizeU16
	}
	
	/// Is too short to be an IEEE 802.1Q Virtual LAN packet?
	#[inline(always)]
	pub(crate) fn is_too_short_to_be_a_vlan_ethernet_packet(self) -> bool
	{
		const Overhead: u16 = VirtualLanPacketHeader::IEEE_802_1Q_SizeU16;
		
		self.packet_length_if_contiguous() < (EthernetPacketHeader::SizeU16 + Overhead)
	}
	
	/// Is too short to be an IEEE 802.1ad QinQ Virtual LAN packet?
	#[inline(always)]
	pub(crate) fn is_too_short_to_be_a_qinq_vlan_ethernet_packet(self) -> bool
	{
		const Overhead: u16 = VirtualLanPacketHeader::IEEE_802_1ad_SizeU16 + VirtualLanPacketHeader::IEEE_802_1Q_SizeU16;
		
		self.packet_length_if_contiguous() < (EthernetPacketHeader::SizeU16 + Overhead)
	}
	
	/// Needs to be set so that `reassemble_fragmented_internet_protocol_version_4_packet()` or `reassemble_fragmented_internet_protocol_version_6_packet()` work correctly.
	#[inline(always)]
	pub(crate) fn set_layer_2_header_length(self, length: u16)
	{
		self.mutable_reference()._5._1.set_l2_len(length as u64)
	}
	
	/// Needs to be set so that `reassemble_fragmented_internet_protocol_version_4_packet()` or `reassemble_fragmented_internet_protocol_version_6_packet()` work correctly.
	#[inline(always)]
	pub(crate) fn set_layer_3_header_length(self, length: u16)
	{
		self.mutable_reference()._5._1.set_l3_len(length as u64)
	}
	
	/// Ethernet packet.
	///
	/// No checking of data length is made; be careful dereferencing this value.
	/// Call one of `is_too_short_to_be_an_ethernet_packet()`, `is_too_short_to_be_a_vlan_ethernet_packet()` or `is_too_short_to_be_a_qinq_vlan_ethernet_packet()` first.
	#[inline(always)]
	pub(crate) fn ethernet_packet<'a>(self) -> &'a EthernetPacket
	{
		self.offset_into_data_reference::<'a, EthernetPacket>(0)
	}
	
	/// Optimized routine that only works on direct, contiguous packets with a reference count of 1.
	#[inline(always)]
	pub(crate) fn free_direct_contiguous_packet(self)
	{
		self.raw_free()
	}
	
	/// Was VLAN tag control information (TCI) stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
	#[inline(always)]
	pub(crate) fn was_vlan_tag_control_information_stripped(self) -> bool
	{
		self.has_offload_flags(PKT_RX_VLAN_STRIPPED)
	}
	
	/// Stripped VLAN tag control information (TCI).
	#[inline(always)]
	pub(crate) fn stripped_vlan_tag_control_information(self) -> TagControlInformation
	{
		TagControlInformation(NetworkByteOrderEndianU16::from_network_byte_order_value(self.reference().vlan_tci))
	}
	
	/// Was VLAN QinQ tag control information (TCI) stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
	#[inline(always)]
	pub(crate) fn was_vlan_qinq_tag_control_information_stripped(self) -> bool
	{
		self.has_offload_flags(PKT_RX_QINQ_STRIPPED)
	}
	
	/// Stripped VLAN QinQ tag control information (TCI) (outer and inner).
	#[inline(always)]
	pub(crate) fn stripped_vlan_qinq_tag_control_information(self) -> (TagControlInformation, TagControlInformation)
	{
		(TagControlInformation(NetworkByteOrderEndianU16::from_network_byte_order_value(self.reference().vlan_tci_outer)), self.stripped_vlan_tag_control_information())
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
	fn has_offload_flags(self, flags: u64) -> bool
	{
		(self.offload_flags() & flags) != 0
	}
	
	#[inline(always)]
	fn offload_flags(self) -> u64
	{
		self.reference().ol_flags
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
	
	/// Implementation of DPDK `rte_pktmbuf_mtod` and `rte_pktmbuf_mtod_offset`.
	///
	/// Compare with `io_virtual_address_offset()`.
	#[inline(always)]
	fn offset_into_data_reference<'a, T: 'a>(self, offset: usize) -> &'a T
	{
		unsafe { & * (self.offset_into_data::<T>(offset).as_ptr() as *const T) }
	}
	
	/// Implementation of DPDK `rte_pktmbuf_mtod` and `rte_pktmbuf_mtod_offset`.
	///
	/// Compare with `io_virtual_address_offset()`.
	#[inline(always)]
	fn offset_into_data<T>(self, offset: usize) -> NonNull<T>
	{
		let packet = { self.reference() };
		let pointer = ((packet.buf_addr as usize) + (self.segment_buffer_reserved_head_room() as usize) + offset) as *mut T;
		unsafe { NonNull::new_unchecked(pointer) }
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
	
	/// Constant function calculates buffer length.
	#[inline(always)]
	const fn buffer_length(data_room_length: u16) -> u16
	{
		data_room_length + Self::HeadRoom
	}











//
//
//	/// Value of `data_room_size` passed to `rte_pktmbuf_pool_create()` to ensure that segmentation of receive packets is not needed and packet drops do not occur.
//	///
//	/// Use this value to avoid the need to specify `offloads::DEV_RX_OFFLOAD_SCATTER` for poll-mode drivers (PMDs).
//	#[inline(always)]
//	fn data_room_size_for_packet_buffer_pool(maximum_transmission_unit_size: MaximumTransmissionUnitSize) -> u16
//	{
//		maximum_transmission_unit_size.to_data_room_size_for_packet_buffer_pool()
//	}
//
//

//
//	/// Set this packet to be ignored.
//	#[inline(always)]
//	fn ignore(self)
//	{
//		*self.mutable_reference()._3.packet_type.as_mut() = RTE_PTYPE_UNKNOWN
//	}
//
//	/// Raw hardware packet type.
//	#[inline(always)]
//	fn hardware_packet_type(self) -> u32
//	{
//		*self.reference()._3.packet_type.as_ref()
//	}
//
//	/// Layer 2 hardware packet type.
//	#[inline(always)]
//	fn layer_2_hardware_packet_type(self) -> Layer2PacketType
//	{
//		Layer2PacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
//	}
//
//	/// Returns a Layer 2 name for this hardware packet type.
//	///
//	/// If this is a tunneled packet, then this is known as the Outer Layer 2 name.
//	///
//	/// * All names start `L2_`.
//	/// * If unknown, name will be `L2_UNKNOWN`; this occurs for invalid packet type flags.
//	/// * If known but not further categorised, name will be `L2_ETHER`.
//	/// * If the hardware identified a particular EtherType, then the name will be one of:-
//	///   * `L2_ETHER_TIMESYNC`
//	///   * `L2_ETHER_ARP`
//	///   * `L2_ETHER_LLDP`
//	///   * `L2_ETHER_NSH`
//	///   * `L2_ETHER_VLAN`
//	///   * `L2_ETHER_QINQ`
//	///   * `L2_ETHER_PPPOE`
//	#[inline(always)]
//	fn layer_2_hardware_packet_type_name(self) -> &'static CStr
//	{
//		unsafe { CStr::from_ptr(rte_get_ptype_l2_name(self.hardware_packet_type())) }
//	}
//
//	/// Layer 3 hardware packet type.
//	///
//	/// See also `self.layer_3_hardware_packet_type_is_internet_protocol_version_4()` and `self.layer_3_hardware_packet_type_is_internet_protocol_version_6()` for a short-cut approach that.
//	#[inline(always)]
//	fn layer_3_hardware_packet_type(self) -> Layer3PacketType
//	{
//		Layer3PacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
//	}
//
//	/// Returns a Layer 3 name for this packet type.
//	///
//	/// If this is a tunneled packet, then this is known as the Outer Layer 3 name.
//	///
//	/// * All names start `L3_`.
//	/// * If unknown, name will be `L3_UNKNOWN`; this occurs for invalid packet type flags.
//	/// * If known, name will start with either `L3_IPV6` or `L3_IPV6`.
//	/// * Other names are:-
//	///   * `L3_IPV4`
//	///   * `L3_IPV4_EXT`
//	///   * `L3_IPV4_EXT_UNKNOWN`
//	///   * `L3_IPV6`
//	///   * `L3_IPV6_EXT`
//	///   * `L3_IPV6_EXT_UNKNOWN`
//	#[inline(always)]
//	fn layer_3_hardware_packet_type_name(self) -> &'static CStr
//	{
//		unsafe { CStr::from_ptr(rte_get_ptype_l3_name(self.hardware_packet_type())) }
//	}
//
//	/// Equivalent to `RTE_ETH_IS_IPV4_HDR`.
//	#[inline(always)]
//	fn layer_3_hardware_packet_type_is_internet_protocol_version_4(self) -> bool
//	{
//		self.hardware_packet_type() & RTE_PTYPE_L3_IPV4 != 0
//	}
//
//	/// Equivalent to `RTE_ETH_IS_IPV6_HDR`.
//	#[inline(always)]
//	fn layer_3_hardware_packet_type_is_internet_protocol_version_6(self) -> bool
//	{
//		self.hardware_packet_type() & RTE_PTYPE_L3_IPV6 != 0
//	}
//
//	/// Layer 4 hardware packet type.
//	#[inline(always)]
//	fn layer_4_hardware_packet_type(self) -> Layer4PacketType
//	{
//		Layer4PacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
//	}
//
//	/// Returns a Layer 4 name for this packet type.
//	///
//	/// If this is a tunneled packet, then this is known as the Outer Layer 4 name.
//	///
//	/// * All names start `L4_`.
//	/// * If unknown or not a layer 4 packet, name will be `L4_UNKNOWN`; this also occurs for invalid packet type flags.
//	/// * Other names are:-
//	///   * `L4_ICMP`
//	///   * `L4_UDP`
//	///   * `L4_TCP`
//	///   * `L4_SCTP`
//	///   * `L4_FRAG`
//	///   * `L4_NONFRAG`
//	#[inline(always)]
//	fn layer_4_hardware_packet_type_name(self) -> &'static CStr
//	{
//		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.hardware_packet_type())) }
//	}
//
//	/// Is this packet encapsulated in a tunnel?
//	///
//	/// In which case, the inner layers are where the data is going to.
//	#[inline(always)]
//	fn is_encapsulated_in_a_tunnel_and_has_inner_layers(self) -> bool
//	{
//		self.hardware_packet_type() & RTE_PTYPE_TUNNEL_MASK == RTE_PTYPE_TUNNEL_MASK
//	}
//
//	#[inline(always)]
//	fn tunnel_hardware_packet_type(self) -> TunnelPacketType
//	{
//		TunnelPacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
//	}
//
//	/// Returns a tunnel name for this packet type.
//	///
//	/// * All names start `TUNNEL_`.
//	/// * If unknown or not a tunnel, name will be `TUNNEL_UNKNOWN`; this also occurs for invalid packet type flags.
//	/// * Other names are:-
//	///   * `TUNNEL_IP`
//	///   * `TUNNEL_GRE`
//	///   * `TUNNEL_VXLAN`
//	///   * `TUNNEL_NVGRE`
//	///   * `TUNNEL_GENEVE`
//	///   * `TUNNEL_GRENAT`
//	///   * `TUNNEL_GTPC`
//	///   * `TUNNEL_GTPU`
//	///   * `TUNNEL_ESP`
//	///   * `TUNNEL_L2TP`
//	#[inline(always)]
//	fn tunnel_hardware_packet_type_name(self) -> &'static CStr
//	{
//		unsafe { CStr::from_ptr(rte_get_ptype_tunnel_name(self.hardware_packet_type())) }
//	}
//
//	/// Tunnel Inner Layer 2 hardware packet type.
//	#[inline(always)]
//	fn tunnel_inner_layer_2_hardware_packet_type(self) -> Layer2PacketType
//	{
//		Layer2PacketType::inner_layer_2_for_tunnel_from_packet_buffer_packet_type(self.hardware_packet_type())
//	}
//
//	/// Returns a Tunnel Inner Layer 2 name for this hardware packet type.
//	///
//	/// * All names start `INNER_L2_`.
//	/// * If unknown, name will be `INNER_L2_UNKNOWN`; this occurs for invalid packet type flags.
//	/// * If known but not further categorised, name will be `INNER_L2_ETHER`.
//	/// * If the hardware identified a particular EtherType, then the name will be one of:-
//	///   * `INNER_L2_ETHER_VLAN`
//	///   * `INNER_L2_ETHER_QINQ`
//	#[inline(always)]
//	fn tunnel_inner_layer_2_hardware_packet_type_name(self) -> &'static CStr
//	{
//		unsafe { CStr::from_ptr(rte_get_ptype_inner_l2_name(self.hardware_packet_type())) }
//	}
//
//	/// Tunnel Inner Layer 3 hardware packet type.
//	#[inline(always)]
//	fn tunnel_inner_layer_3_hardware_packet_type(self) -> Layer3PacketType
//	{
//		Layer3PacketType::inner_layer_3_for_tunnel_from_packet_buffer_packet_type(self.hardware_packet_type())
//	}
//
//	/// Returns a Tunnel Inner Layer 3 name for this packet type.
//	///
//	/// * All names start `INNER_L3_`.
//	/// * If unknown, name will be `INNER_L3_UNKNOWN`; this occurs for invalid packet type flags.
//	/// * Other names are:-
//	///   * `INNER_L3_IPV4`
//	///   * `INNER_L3_IPV4_EXT`
//	///   * `INNER_L3_IPV4_EXT_UNKNOWN`
//	///   * `INNER_L3_IPV6`
//	///   * `INNER_L3_IPV6_EXT`
//	///   * `INNER_L3_IPV6_EXT_UNKNOWN`
//	#[inline(always)]
//	fn tunnel_inner_layer_3_hardware_packet_type_name(self) -> &'static CStr
//	{
//		unsafe { CStr::from_ptr(rte_get_ptype_inner_l3_name(self.hardware_packet_type())) }
//	}
//
//	/// Tunnel Inner Layer 4 hardware packet type.
//	#[inline(always)]
//	fn tunnel_inner_layer_4_hardware_packet_type(self) -> Layer4PacketType
//	{
//		Layer4PacketType::inner_layer_4_for_tunnel_from_packet_buffer_packet_type(self.hardware_packet_type())
//	}
//
//	/// Returns a Tunnel Inner Layer 4 name for this packet type.
//	///
//	/// * All names start `INNER_L4_`.
//	/// * If unknown or not a layer 4 packet, name will be `INNER_L4_UNKNOWN`; this also occurs for invalid packet type flags.
//	/// * Other names are:-
//	///   * `INNER_L4_ICMP`
//	///   * `INNER_L4_UDP`
//	///   * `INNER_L4_TCP`
//	///   * `INNER_L4_SCTP`
//	///   * `INNER_L4_FRAG`
//	///   * `INNER_L4_NONFRAG`
//	#[inline(always)]
//	fn tunnel_inner_layer_4_hardware_packet_type_name(self) -> &'static CStr
//	{
//		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.hardware_packet_type())) }
//	}
//
//	/// Destroy this packet and return the memory it uses to its packet buffer pool (PacketBufferPool).
//	#[inline(always)]
//	fn free(self)
//	{
//		unsafe { rust_rte_pktmbuf_free(self.as_ptr()) };
//	}
//
//
//
//	/// Was IEEE1588 (802.1AS) timestamp stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
//	///
//	/// IEEE1588 timestamps are part of the Precision Time Protocol (PTP) (EtherType 0x88F7).
//	///
//	/// For code examples using PTP to adjust the Linux kernel's clock, see in DPDK `examples/ptpclient/ptpclient.c`, particularly `parse_ptp_frames()`.
//	#[inline(always)]
//	fn was_ieee1588_timestamp_stripped(self) -> bool
//	{
//		self.has_offload_flag(PKT_RX_TIMESTAMP)
//	}
//
//	/// Stripped IEEE1588 (802.1AS) timestamp.
//	///
//	/// IEEE1588 timestamps are part of the Precision Time Protocol (PTP) (EtherType 0x88F7).
//	///
//	/// The unit and time reference are not normalized but are always the same for a given (ethernet) port.
//	#[inline(always)]
//	fn stripped_ieee1588_timestamp_information(self) -> u64
//	{
//		self.reference().timestamp
//	}
//
//	/// IEEE1588 (802.1AS) flags.
//	///
//	/// Slow to obtain as not likely to be cached.
//	#[inline(always)]
//	fn timesync_flags(self) -> u16
//	{
//		(self.reference()).timesync
//	}
//
//	/// Is this an indirectly attached packet buffer?
//	#[inline(always)]
//	fn is_indirect_attached_packet_buffer(self) -> bool
//	{
//		self.has_offload_flag(IND_ATTACHED_MBUF)
//	}
//
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
		(self.reference()).data_off
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
}
