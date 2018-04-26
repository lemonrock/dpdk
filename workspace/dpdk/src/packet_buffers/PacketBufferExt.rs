// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An extension trait which makes a `NonNull<rte_mbuf>` appear as a regular object, `PacketBuffer`.
pub trait PacketBufferExt: PrintInformation
{
	/// Private data alignment.
	const AlignmentOfPrivateData: usize = RTE_MBUF_PRIV_ALIGN as usize;
	
	/// Normally 128 bytes, but a configuration value for DPDK.
	///
	/// Equivalent to `RTE_PKTMBUF_HEADROOM`.
	const HeadRoom: u16 = RTE_PKTMBUF_HEADROOM as u16;
	
	/// Some NICs need at least a 2KB buffer to receive a standard Ethernet frame without splitting it into multiple segments.
	///
	/// Equivalent to `RTE_MBUF_DEFAULT_DATAROOM`.
	const DefaultDataRoom: u16 = RTE_MBUF_DEFAULT_DATAROOM as u16;
	
	/// Some NICs need at least a 2KB buffer to receive a standard Ethernet frame without splitting it into multiple segments.
	///
	/// For PacketBuffers used for receive or transmit, this is the minimal recommended buffer length.
	///
	/// Equivalent to `RTE_MBUF_DEFAULT_BUF_SIZE`.
	const DefaultBufferLength: u16 = buffer_length(Self::DefaultDataRoom);
	
	/// Maximum number of segment buffers in a packet buffer.
	const MaximumNumberOfSegmentBuffers: u16 = RTE_MBUF_MAX_NB_SEGS as u16;
	
	/// Value of `data_room_size` passed to `rte_pktmbuf_pool_create()` to ensure that segmentation of receive packets is not needed and packet drops do not occur.
	///
	/// Use this value to avoid the need to specify `offloads::DEV_RX_OFFLOAD_SCATTER` for poll-mode drivers (PMDs).
	#[inline(always)]
	fn data_room_size_for_packet_buffer_pool(maximum_transmission_unit_size: MaximumTransmissionUnitSize) -> u16
	{
		maximum_transmission_unit_size.to_data_room_size_for_packet_buffer_pool()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn reference<'a>(self) -> &'a rte_mbuf
	{
		unsafe { & * self.as_ptr() }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn mutable_reference<'a>(self) -> &'a mut rte_mbuf
	{
		unsafe { &mut * self.as_ptr() }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn has_offload_flag(self, flag: u64) -> bool
	{
		(self.offload_flags() & flag) == flag
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn offload_flags(self) -> u64
	{
		self.reference().offload_flags
	}
	
	/// Set this packet to be ignored.
	#[inline(always)]
	fn ignore(self)
	{
		*self.mutable_reference()._3.packet_type.as_mut() = RTE_PTYPE_UNKNOWN
	}
	
	/// Raw hardware packet type.
	#[inline(always)]
	fn hardware_packet_type(self) -> u32
	{
		 *self.reference()._3.packet_type.as_ref()
	}
	
	/// Layer 2 hardware packet type.
	#[inline(always)]
	fn layer_2_hardware_packet_type(self) -> Layer2PacketType
	{
		Layer2PacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
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
	fn layer_2_hardware_packet_type_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l2_name(self.hardware_packet_type())) }
	}
	
	/// Layer 3 hardware packet type.
	///
	/// See also `self.layer_3_hardware_packet_type_is_internet_protocol_version_4()` and `self.layer_3_hardware_packet_type_is_internet_protocol_version_6()` for a short-cut approach that.
	#[inline(always)]
	fn layer_3_hardware_packet_type(self) -> Layer3PacketType
	{
		Layer3PacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
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
	fn layer_3_hardware_packet_type_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l3_name(self.hardware_packet_type())) }
	}
	
	/// Equivalent to `RTE_ETH_IS_IPV4_HDR`.
	#[inline(always)]
	fn layer_3_hardware_packet_type_is_internet_protocol_version_4(self) -> bool
	{
		self.hardware_packet_type() & RTE_PTYPE_L3_IPV4 != 0
	}
	
	/// Equivalent to `RTE_ETH_IS_IPV6_HDR`.
	#[inline(always)]
	fn layer_3_hardware_packet_type_is_internet_protocol_version_6(self) -> bool
	{
		self.hardware_packet_type() & RTE_PTYPE_L3_IPV6 != 0
	}
	
	/// Layer 4 hardware packet type.
	#[inline(always)]
	fn layer_4_hardware_packet_type(self) -> Layer4PacketType
	{
		Layer4PacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
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
	fn layer_4_hardware_packet_type_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.hardware_packet_type())) }
	}
	
	/// Is this packet encapsulated in a tunnel?
	///
	/// In which case, the inner layers are where the data is going to.
	#[inline(always)]
	fn is_encapsulated_in_a_tunnel_and_has_inner_layers(self) -> bool
	{
		self.hardware_packet_type() & RTE_PTYPE_TUNNEL_MASK == RTE_PTYPE_TUNNEL_MASK
	}
	
	#[inline(always)]
	fn tunnel_hardware_packet_type(self) -> TunnelPacketType
	{
		TunnelPacketType::from_packet_buffer_packet_type(self.hardware_packet_type())
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
	#[inline(always)]
	fn tunnel_hardware_packet_type_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_tunnel_name(self.hardware_packet_type())) }
	}
	
	/// Tunnel Inner Layer 2 hardware packet type.
	#[inline(always)]
	fn tunnel_inner_layer_2_hardware_packet_type(self) -> Layer2PacketType
	{
		Layer2PacketType::inner_layer_2_for_tunnel_from_packet_buffer_packet_type(self.hardware_packet_type())
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
	fn tunnel_inner_layer_2_hardware_packet_type_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_inner_l2_name(self.hardware_packet_type())) }
	}
	
	/// Tunnel Inner Layer 3 hardware packet type.
	#[inline(always)]
	fn tunnel_inner_layer_3_hardware_packet_type(self) -> Layer3PacketType
	{
		Layer3PacketType::inner_layer_3_for_tunnel_from_packet_buffer_packet_type(self.hardware_packet_type())
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
	fn tunnel_inner_layer_3_hardware_packet_type_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_inner_l3_name(self.hardware_packet_type())) }
	}
	
	/// Tunnel Inner Layer 4 hardware packet type.
	#[inline(always)]
	fn tunnel_inner_layer_4_hardware_packet_type(self) -> Layer4PacketType
	{
		Layer4PacketType::inner_layer_4_for_tunnel_from_packet_buffer_packet_type(self.hardware_packet_type())
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
	fn tunnel_inner_layer_4_hardware_packet_type_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.hardware_packet_type())) }
	}
	
	/// Destroy this packet and return the memory it uses to its packet buffer pool (PacketBufferPool).
	#[inline(always)]
	fn free(self)
	{
		unsafe { rust_rte_pktmbuf_free(self.as_ptr()) };
	}
	
	/// Was VLAN QinQ tag control information (TCI) stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
	#[inline(always)]
	fn was_vlan_qinq_tag_control_information_stripped(self) -> bool
	{
		self.has_offload_flag(PKT_RX_QINQ_STRIPPED)
	}
	
	/// Stripped VLAN QinQ tag control information (TCI) (outer and inner).
	#[inline(always)]
	fn stripped_vlan_qinq_tag_control_information(self) -> (VirtualLanPacketTagControlInformation, VirtualLanPacketTagControlInformation)
	{
		(VirtualLanPacketTagControlInformation(NetworkByteOrderEndianU16::from_network_byte_order_value(self.reference().vlan_tci_outer)), self.stripped_vlan_tag_control_information())
	}
	
	/// Was VLAN tag control information (TCI) stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
	#[inline(always)]
	fn was_vlan_tag_control_information_stripped(self) -> bool
	{
		self.has_offload_flag(PKT_RX_VLAN_STRIPPED)
	}
	
	/// Stripped VLAN tag control information (TCI).
	#[inline(always)]
	fn stripped_vlan_tag_control_information(self) -> VirtualLanPacketTagControlInformation
	{
		VirtualLanPacketTagControlInformation(NetworkByteOrderEndianU16::from_network_byte_order_value(self.reference().vlan_tci))
	}
	
	/// Was IEEE1588 (802.1AS) timestamp stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
	///
	/// IEEE1588 timestamps are part of the Precision Time Protocol (PTP) (EtherType 0x88F7).
	///
	/// For code examples using PTP to adjust the Linux kernel's clock, see in DPDK `examples/ptpclient/ptpclient.c`, particularly `parse_ptp_frames()`.
	#[inline(always)]
	fn was_ieee1588_timestamp_stripped(self) -> bool
	{
		self.has_offload_flag(PKT_RX_TIMESTAMP)
	}
	
	/// Stripped IEEE1588 (802.1AS) timestamp.
	///
	/// IEEE1588 timestamps are part of the Precision Time Protocol (PTP) (EtherType 0x88F7).
	///
	/// The unit and time reference are not normalized but are always the same for a given (ethernet) port.
	#[inline(always)]
	fn stripped_ieee1588_timestamp_information(self) -> u64
	{
		self.reference().timestamp
	}
	
	/// IEEE1588 (802.1AS) flags.
	///
	/// Slow to obtain as not likely to be cached.
	#[inline(always)]
	fn timesync_flags(self) -> u16
	{
		(self.reference()).timesync
	}
	
	/// Is this an indirectly attached packet buffer?
	#[inline(always)]
	fn is_indirect_attached_packet_buffer(self) -> bool
	{
		self.has_offload_flag(IND_ATTACHED_MBUF)
	}
	
	/// Is this a control buffer (`CTRL_MBUF`)?
	#[inline(always)]
	fn is_this_a_control_buffer(self) -> bool
	{
		self.has_offload_flag(CTRL_MBUF_FLAG)
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
	
	/// Packet length if contiguous.
	///
	/// Same as `data_length()`.
	#[inline(always)]
	fn packet_length_if_contiguous(self) -> u16
	{
		self.debug_assert_is_contiguous();
		
		self.data_length()
	}
	
	/// Packet length less ethernet header.
	#[inline(always)]
	fn packet_length_if_contiguous_less_ethernet_packet_header(self) -> u16
	{
		self.packet_length_if_contiguous() - EthernetPacketHeader::SizeU16
	}
	
	/// Is too short to be an ethernet packet?
	#[inline(always)]
	fn is_too_short_to_be_an_ethernet_packet(self) -> bool
	{
		self.packet_length_if_contiguous() < EthernetPacketHeader::SizeU16
	}
	
	/// Is too short to be an IEEE 802.1Q Virtual LAN packet?
	#[inline(always)]
	fn is_too_short_to_be_a_vlan_ethernet_packet(self) -> bool
	{
		const Overhead: u16 = VirtualLanPacketHeader::VirtualLanPacketHeaderSizeU16;
		
		self.packet_length_if_contiguous() < (EthernetPacketHeader::SizeU16 + Overhead)
	}
	
	/// Is too short to be an IEEE 802.1ad QinQ Virtual LAN packet?
	#[inline(always)]
	fn is_too_short_to_be_a_qinq_vlan_ethernet_packet(self) -> bool
	{
		const Overhead: u16 = VirtualLanPacketHeader::QinQVirtualLanPacketHeaderSizeU16 + VirtualLanPacketHeader::VirtualLanPacketHeaderSizeU16;
		
		self.packet_length_if_contiguous() < (EthernetPacketHeader::SizeU16 + Overhead)
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
		self.segement_buffer_length() - tail_offset
	}
	
	/// Size of the application private data.
	///
	/// If this is an indirect PacketBuffer, it is the size of the parent direct PacketBuffer's application private data.
	#[inline(always)]
	fn private_size(self) -> u16
	{
		(self.reference()).priv_size
	}
	
	/// Implementation of DPDK `rte_pktmbuf_mtod`.
	///
	/// Compare with `io_virtual_address()`.
	#[inline(always)]
	fn start_of_data<T>(self) -> NonNull<T>
	{
		self.offset_into_data::<T>(0)
	}
	
	/// Implementation of DPDK `rte_pktmbuf_mtod_offset`.
	///
	/// Compare with `io_virtual_address_offset()`.
	#[inline(always)]
	fn offset_into_data<T>(self, offset: usize) -> NonNull<T>
	{
		let packet = { self.as_ref() };
		let pointer = ((packet.buf_addr as usize) + (self.segment_buffer_reserved_head_room() as usize) + offset) as *mut T;
		unsafe { NonNull::new_unchecked(pointer) }
	}
	
	#[inline(always)]
	fn ethernet_packet(self) -> NonNull<EthernetPacket>
	{
		self.start_of_data::<EthernetPacket>()
	}
	
	/// Pointer to internet protocol version 4 header (does not validate that it *is* such a header).
	#[inline(always)]
	fn internet_protocol_version_4_header(self) -> NonNull<ipv4_hdr>
	{
		Seems wrong - compare with self.ethernet_header()
		
		self.start_of_data::<ipv4_hdr>()
	}
	
	/// Pointer to internet protocol version 6 header (does not validate that it *is* such a header).
	#[inline(always)]
	fn internet_protocol_version_6_header<>(self) -> NonNull<ipv6_hdr>
	{
		Seems wrong (1)
		self.start_of_data::<ip6_hdr>()
		
		Seems wrong (2) - does not take into account VLAN or QinQ
		// And also needs to take into account whether VLAN or QinQ was stripped.
		let layer_2_length = size_of::<ether_hdr>();
		self.offset_into_data::<ip6_hdr>(layer_2_length as usize)
		
		
		Seems wrong (3) - headers must be documented as packed to generate correct instructions.
	
		Seems wrong (4) - we recalculate the offset into the data multiple times, which is SLOW.
	
		Seems wrong (5) - we do not take into account multiple segments.
	}
	
	
	
	
	
	
	
	
	
	
	
	
	/// Packet data length, ie packet length less header length.
	#[inline(always)]
	fn internet_protocol_version_4_packet_data_length(&self) -> usize
	{
		(self.length() as usize) - size_of::<ipv4_hdr>()
	}
	
	/// Layer 4 Protocol (TCP, UDP, etc) of Internet Protocol (IP) version 4 packet.
	#[inline(always)]
	fn internet_protocol_version_4_packet_layer_4_protocol(self) -> u8
	{
		let header = self.internet_protocol_version_4_header();
		unsafe { header.as_ref() }.next_proto_id
	}
	
	/// Layer 4 Protocol (TCP, UDP, etc) of Internet Protocol (IP) version 6 packet.
	#[inline(always)]
	fn internet_protocol_version_6_packet_layer_4_protocol(self) -> u8
	{
		let header = self.internet_protocol_version_6_header();
		unsafe { header.as_ref() }.proto
	}
	
	/// Source address of Internet Protocol (IP) version 4 packet.
	#[inline(always)]
	fn internet_protocol_version_4_packet_source_address(self) -> InternetProtocolVersion4HostAddress
	{
		let header = self.internet_protocol_version_4_header();
		InternetProtocolVersion4HostAddress::from_network_endian(unsafe { header.as_ref() }.src_addr)
	}
	
	/// Source address of Internet Protocol (IP) version 6 packet.
	#[inline(always)]
	fn internet_protocol_version_6_packet_source_address(self) -> InternetProtocolVersion6HostAddress
	{
		let header = self.internet_protocol_version_6_header();
		InternetProtocolVersion4HostAddress::from_octets(unsafe { header.as_ref() }.src_addr)
	}
	
	/// Destination address of Internet Protocol (IP) version 4 packet.
	#[inline(always)]
	fn internet_protocol_version_4_packet_destination_address(self) -> InternetProtocolVersion4HostAddress
	{
		let header = self.internet_protocol_version_4_header();
		InternetProtocolVersion4HostAddress::from_network_endian(unsafe { header.as_ref() }.dst_addr)
	}
	
	/// Destination address of Internet Protocol (IP) version 6 packet.
	#[inline(always)]
	fn internet_protocol_version_6_packet_destination_address(self) -> InternetProtocolVersion6HostAddress
	{
		let header = self.internet_protocol_version_6_header();
		InternetProtocolVersion4HostAddress::from_octets(unsafe { header.as_ref() }.dst_addr)
	}
	
	/// Hop limit of Internet Protocol (IP) version 4 packet.
	#[inline(always)]
	fn internet_protocol_version_4_packet_hop_limit(self) -> u8
	{
		let header = self.internet_protocol_version_4_header();
		unsafe { header.as_ref() }.time_to_live
	}
	
	/// Hop limit of Internet Protocol (IP) version 6 packet.
	#[inline(always)]
	fn internet_protocol_version_6_packet_hop_limit(self) -> u8
	{
		let header = self.internet_protocol_version_6_header();
		unsafe { header.as_ref() }.hop_limits
	}
	
	/// Fragment offset.
	#[inline(always)]
	fn internet_protocol_version_4_packet_fragment_offset(self) -> NetworkByteOrderEndianU16
	{
		let header = self.internet_protocol_version_4_header();
		NetworkByteOrderEndianU16::from_network_byte_order_value(unsafe { header.as_ref() }.fragment_offset)
	}
	
	/// Is the do not fragment (DF) flag set?
	#[inline(always)]
	fn internet_protocol_version_4_packet_has_do_not_fragment_flag_set(self) -> bool
	{
		#[cfg(target_endian = "big")] const DoNotFragmentFlag: u16 = 0x4000;
		#[cfg(target_endian = "little")] const DoNotFragmentFlag: u16 = 0x0040;
		
		let fragment_offset = self.internet_protocol_version_4_packet_fragment_offset();
		fragment_offset.to_network_byte_order_value() & DoNotFragmentFlag != 0
	}
	
	/// Packet data length, ie packet length less header length.
	///
	/// Does not take into account any extension headers.
	#[inline(always)]
	fn internet_protocol_version_6_packet_data_length(self) -> usize
	{
		(self.length() as usize) - size_of::<ipv6_hdr>()
	}
	
	/// If returns 1, then no need to fragment.
	///
	/// Use the value returned to size a Vec<>, or ArrayVec<>, used to handle fragmented packets.
	///
	/// Will never return 0.
	#[inline(always)]
	fn number_of_fragmented_internet_protocol_version_6_packets_rounded_up(self, maximum_transmission_unit_size_including_internet_protocol_version_6_header: MaximumTransmissionUnitSize) -> usize
	{
		let fragment_size = maximum_transmission_unit_size_including_internet_protocol_version_6_header.internet_protocol_version_6_fragment_size() as usize;
		debug_assert_ne!(fragment_size, 0, "fragment_size should never be zero");
		
		let packet_data_length = self.internet_protocol_version_6_packet_data_length();
		
		(packet_data_length + fragment_size - 1) / fragment_size
	}
	
	/// `self` is `packet_to_fragment`: will be freed if gets fragmented.
	///
	/// Optimized to avoid fragmenting if fragmenting not required; in this case, the `packet_to_fragment` is added to` fragments`.
	///
	/// Will resize fragments as appropriate.
	///
	/// TLDK fragments internally for UDP.
	///
	/// DPDK only supports `RTE_LIBRTE_IP_FRAG_MAX_FRAG`, which is typically 4.
	#[inline(always)]
	fn fragment_internet_protocol_version_6_packet(self, maximum_transmission_unit_size_including_internet_protocol_version_6_header: MaximumTransmissionUnitSize, direct_packets_pool: Non<rte_mempool>, indirect_packets_pool: Non<rte_mempool>, fragments: &mut Vec<Self>)
	{
		debug_assert_ne!(maximum_transmission_unit_size_including_internet_protocol_version_6_header, 0, "maximum_transmission_unit_size_including_internet_protocol_version_6_header can not be zero");
		debug_assert!(maximum_transmission_unit_size_including_internet_protocol_version_6_header.as_u16() as usize > size_of::<ipv6_hdr>(), "maximum_transmission_unit_size_including_internet_protocol_version_6_header '{}' is not greater than internet protocol version 6 header", maximum_transmission_unit_size_including_internet_protocol_version_6_header);
		
		if self.length() >= maximum_transmission_unit_size_including_internet_protocol_version_6_header.as_usize()
		{
			fragments.push(self);
			return
		}
		
		let original_length = fragments.len();
		let remaining_space = fragments.capacity() - original_length;
		let number_of_fragments_required = self.number_of_fragmented_internet_protocol_version_6_packets_rounded_up(maximum_transmission_unit_size_including_internet_protocol_version_6_header);
		
		if remaining_space < number_of_fragments_required
		{
			fragments.reserve(number_of_fragments_required - remaining_space);
		}
		
		let mut packets_out: *mut *mut rte_mbuf = unsafe { transmute(fragments.get_unchecked_mut(original_length)) };
		let result = unsafe { rte_ipv6_fragment_packet(self.as_ptr(), packets_out, number_of_fragments_required as u16, maximum_transmission_unit_size_including_internet_protocol_version_6_header.as_u16(), direct_packet_buffer_pool.as_ptr(), indirect_packet_buffer_pool.as_ptr()) };
		
		self.free();
		
		if likely(result > 1)
		{
			let number_of_fragments_added = result as usize;
			debug_assert_eq!(number_of_fragments_added, number_of_fragments_required, "Ourselves and DPDK disagree on number of fragments added");
			unsafe { fragments.set_len(original_length + number_of_fragments_required) }
			return
		}
		
		match result
		{
			1 => panic!("Should not have created one fragment"),
			0 => panic!("No fragments added"),
			NegativeE::EINVAL => panic!("buffer too small"),
			NegativeE::ENOMEM => panic!("cant allocate fragment packets"),
		}
	}
	
	/// Returns the IO address that points to the start of the data in the packet.
	///
	/// Implements `rte_pktmbuf_iova`.
	#[inline(always)]
	fn io_virtual_address(self) -> rte_iova_t
	{
		self.io_virtual_address_offset(0)
	}
	
	/// Returns the IO address that points to an offset of the data in the packet.
	///
	/// Implements `rte_pktmbuf_iova_offset`.
	#[inline(always)]
	fn io_virtual_address_offset(self, offset: u64) -> rte_iova_t
	{
		let packet = self.reference();
		((*packet._1.buf_iova.as_ref()) + (self.segment_buffer_reserved_head_room() as u64) + offset) as rte_iova_t
	}
	
	/// Returns an error on overflow.
	#[inline(always)]
	fn chain_append_tail(&self, tail: PacketBuffer) -> Result<(), ()>
	{
		Self::chain_together(self, tail)
	}
	
	/// Returns an error on overflow.
	#[inline(always)]
	fn chain_prepend_head(&self, head: PacketBuffer) -> Result<(), ()>
	{
		Self::chain_together(head, self)
	}
	
	/// Chain together.
	#[inline(always)]
	fn chain_together(head: PacketBuffer, tail: PacketBuffer) -> Result<(), ()>
	{
		let result = unsafe { rust_rte_pktmbuf_chain(head.as_ptr(), tail.as_ptr()) };
		if likely(result == 0)
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
	fn user_data_as_pointer<T>(self) -> *mut T
	{
		self.reference()._4.userdata as *mut T
	}
	
	/// User data as 8 native endian bytes.
	///
	/// Used for instance by the `rte_security` library.
	#[inline(always)]
	fn user_data_raw(self) -> u64
	{
		self.reference()._4.udata64
	}
	
	/// Parent packet buffer pool that allocated this packet.
	#[inline(always)]
	fn packet_buffer_pool_packet_allocated_from(self) -> PacketBufferPool
	{
		unsafe { NonNull::new_unchecked(self.as_ref().pool) }
	}
	
	/// Optimized routine that only works on direct, contiguous packets with a reference count of 1.
	#[inline(always)]
	fn free_direct_contiguous_packet(self)
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
	fn raw_free(self)
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
	fn pre_free_segment(self) -> Option<PacketBuffer>
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
	fn free_segment(self)
	{
		unsafe { rust_rte_pktmbuf_free_seg(self.as_ptr()) }
	}
	
	/// Next segment.
	#[inline(always)]
	fn next_segment(self) -> Option<NonNull<PacketBuffer>>
	{
		let next = self.reference().next;
		if next.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(next) })
		}
	}
	
	/// Last segment.
	#[inline(always)]
	fn last_segment(self) -> Option<NonNull<PacketBuffer>>
	{
		let result = unsafe { rust_rte_pktmbuf_lastseg(self.as_ptr()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(result) })
		}
	}
	
	/// Checks if this packet is contiguous.
	#[inline(always)]
	fn debug_assert_is_contiguous(self)
	{
		debug_assert!(packet.is_contiguous(), "Inbound packets should be contiguous; scatter receive (`offloads::DEV_RX_OFFLOAD_SCATTER`) is not supported. To make sure not packets are dropped by poll-mode drivers (PMDs) in this mode, the value of `data_room_size` passed to rte_pktmbuf_pool_create() is at least Self::data_room_size_for_packet_buffer_pool()");
	}
	
	/// A contiguous packet has only one segment, ie there is not a chain of segments.
	///
	/// Opposite of `is_segmented()`.
	#[inline(always)]
	fn is_contiguous(self) -> bool
	{
		debug_assert_ne!(self.reference().nb_segs, 0, "No segments!");
		
		self.reference().nb_segs == 1
	}
	
	/// A segmented packet has more than one segment, ie there is a chain of segments.
	///
	/// Opposite of `is_contiguous()`.
	#[inline(always)]
	fn is_segmented(self) -> bool
	{
		debug_assert_ne!(self.reference().nb_segs, 0, "No segments!");
		
		self.reference().nb_segs != 1
	}
	
	/// This function moves the data into the first segment if there is enough tail room.
	///
	/// In effect, compaction to try to make a Packet Buffer contiguous.
	///
	/// The subsequent segments are unchained and freed.
	#[inline(always)]
	fn linearize(self)
	{
		unsafe { rust_rte_pktmbuf_linearize(self.as_ptr()) }
	}
	
	/// Current reference count.
	#[inline(always)]
	fn reference_count(self) -> u16
	{
		self.reference().refcnt
	}
	
	/// Adjust reference count by delta for all segments.
	#[inline(always)]
	fn adjust_reference_count_for_all_segments(self, delta: i16)
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
	
	/// Packet (*not TCP*) sequence number.
	///
	/// Used for re-ordering out-of-order packets, typically when packets are being received by multiple threads.
	/// In this case, the sequence number can be a global atomically incremented counter.
	///
	/// See `ReorderBuffer`.
	#[inline(always)]
	fn sequence_number(self) -> u32
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
	fn set_sequence_number(self, sequence_number: u32)
	{
		self.mutable_reference().seqn = sequence_number
	}
	
	/// Clone.
	///
	/// Allocates the clone from `packet_buffer_pool`.
	#[inline(always)]
	fn clone(&self, packet_buffer_pool: PacketBufferPool) -> Result<NonNull<PacketBuffer>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_clone(self.as_ptr(), packet_buffer_pool.as_ptr()) };
		if unlikely(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result) })
		}
	}
	
	/// Reset the fields of a packet to their default values; allows re-use.
	///
	/// The packet must have only one segment.
	#[inline(always)]
	fn reset(self)
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
	fn reset_segment_buffer_reserved_head_room(self) -> u16
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
	fn attach(self, attach_as_indirect: PacketBuffer)
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
	fn detach_indirect_packet(self)
	{
		unsafe { rust_rte_pktmbuf_detach(self.as_ptr()) }
	}
	
	/// Append `length` bytes to the packet and return a pointer to the start address of the added data.
	///
	/// If there is not enough head room in the first segment, the function will return an error and will not have modified the packet.
	#[inline(always)]
	fn prepend(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_prepend(self.as_ptr(), length) };
		if unlikely(result.is_null())
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
	fn append(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_append(self.as_ptr(), length) };
		if unlikely(result.is_null())
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
	fn remove(self, length: u16) -> Result<NonNull<u8>, ()>
	{
		let result = unsafe { rust_rte_pktmbuf_adj(self.as_ptr(), length) };
		if unlikely(result.is_null())
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
	fn trim(self, length: u16) -> Result<(), ()>
	{
		let result = unsafe { rust_rte_pktmbuf_trim(self.as_ptr(), length) };
		if likely(result == 0)
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
	fn validate_transmit_offload(self) -> Result<(), PosixErrorNumber>
	{
		let result = unsafe { rust_rte_validate_tx_offload(self.as_ptr() as *const _) };
		if likely(result == 0)
		{
			Ok(())
		}
		else if likely(result < 0)
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
	fn has_receive_side_scaling_hash(self) -> bool
	{
		self.has_offload_flag(PKT_RX_RSS_HASH)
	}
	
	/// Receive side scaling hash.
	///
	/// Only valid if `self.has_receive_side_scaling_hash()` is true.
	#[inline(always)]
	fn hash_as_receive_side_scaling_hash(self) -> u32
	{
		* self.reference().rss.as_ref()
	}
	
	/// Is the hash a flow director filter identifier?
	#[inline(always)]
	fn has_flow_director_filter_identifier(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR)
	}
	
	/// Flow director filter identifier.
	#[inline(always)]
	fn hash_as_flow_director_filter_identifier(self) -> u32
	{
		let flow_director = self.reference().fdir.as_ref();
		flow_director.hi
	}
	
	/// Is the hash a flow director hash and filter identifier?
	#[inline(always)]
	fn has_flow_director_hash_and_filter_identifier(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR_ID)
	}
	
	/// Flow director hash and filter identifier.
	#[inline(always)]
	fn hash_as_flow_director_hash_and_filter_identifier(self) -> (u16, u16)
	{
		let flow_director = self.reference().fdir.as_ref();
		let hash_and_identifier = flow_director._1._1.as_ref();
		(hash_and_identifier.hash, hash_and_identifier.id)
	}
	
	/// Is the hash a flow director flexible bytes?
	#[inline(always)]
	fn has_flow_director_flexible_bytes_high_and_low(self) -> bool
	{
		self.has_offload_flag(PKT_RX_FDIR_FLX)
	}
	
	/// Flow director flexible bytes.
	#[inline(always)]
	fn hash_as_flow_director_flexible_bytes_high_and_low(self) -> (u32, u32)
	{
		let flow_director = self.reference().fdir.as_ref();
		(flow_director.hi, flow_director._1.lo)
	}
	
	/// Hierarchical Scheduler flexible bytes.
	#[inline(always)]
	fn hash_as_hierarchical_scheduler_bytes_high_and_low(self) -> (u32, u32)
	{
		let scheduler = * self.reference().sched;
		(scheduler.hi, scheduler.lo)
	}
	
	/// User define tags.
	///
	/// See `rte_distributor_process()`.
	#[inline(always)]
	fn hash_as_user_defined_tags(self) -> u32
	{
		* self.reference().usr.as_ref()
	}
	
	/// Finds a segment at the offset given.
	///
	/// Returns the found segment and the reduced offset, ie as if offset from found segment.
	///
	/// Panics if `offset + buffer.len() > self.length()`.
	#[inline(always)]
	fn find_segment_at_offset(self, mut offset: u32) -> (NonNull<rte_mbuf>, u32)
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
	fn write_even_if_non_contiguous(self, offset: u32, copy_from: &[u8])
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
	
	/// Reads data bytes in a packet from `offset` of `buffer.len()` bytes.
	///
	/// If the packet is contiguous, or, the requested `offset` and `buffer.len()` is entirely withing it, returns a slice to the data inside the packet.
	///
	/// If the packet is not contiguous (it contains more than one segment), reads `buffer.len()` bytes and copies it into `buffer`; returns `buffer` as a slice.
	fn read_even_if_non_contiguous<'a>(self, length: u32, offset: u32, buffer: &'a mut [u8]) -> &'a mut [u8];
	
}

impl PrintInformation for NonNull<rte_mbuf>
{
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		const FirstNBytesOfPacketData: usize = 0;
		unsafe { rte_pktmbuf_dump(stderr as *mut FILE, self.as_ptr(), FirstNBytesOfPacketData) }
	}
}

impl PacketBuffer for NonNull<rte_mbuf>
{
}




fn read_even_if_non_contiguous<'a>(self, length: u32, offset: u32, buffer: &'a mut [u8]) -> &'a mut [u8]
{
	let length = buffer.len();
	let buffer_pointer = buffer.as_mut_ptr() as *mut c_void;
	let pointer_to_read_from: *const c_void = unsafe { rte_pktmbuf_read(self.as_ptr() as *const _, offset, length as u32, buffer_pointer) };
	debug_assert(pointer_to_read_from.is_not_null(), "pointer_to_read_from is null");
	
	if pointer_to_read_from == buffer_pointer
	{
		buffer
	}
	else
	{
		unsafe { from_raw_parts_mut(pointer_to_read_from as *mut u8, length) }
	}
	
	XXXXX
	
	/*
	static inline const void *rte_pktmbuf_read(const struct rte_mbuf *m,
uint32_t off, uint32_t len, void *buf)
{
if (likely(off + len <= rte_pktmbuf_data_len(m)))
	return rte_pktmbuf_mtod_offset(m, char *, off);
else
	return __rte_pktmbuf_read(m, off, len, buf);
}
	
	*/
}

// Replicate read_even_if_non_contiguous

// Implement rte_memcpy in Rust.


//pub fn rust_rte_mbuf_data_iova(mb: *const rte_mbuf) -> rte_iova_t;
//pub fn rust_rte_mbuf_data_iova_default(mb: *const rte_mbuf) -> rte_iova_t;
//pub fn rust_rte_mbuf_from_indirect(mi: *mut rte_mbuf) -> *mut rte_mbuf;
//pub fn rust_rte_mbuf_prefetch_part1(m: *mut rte_mbuf);
//pub fn rust_rte_mbuf_prefetch_part2(m: *mut rte_mbuf);
//pub fn rust_rte_mbuf_raw_alloc(mp: *mut rte_mempool) -> *mut rte_mbuf;
//pub fn rust_rte_mbuf_raw_free(m: *mut rte_mbuf);
//pub fn rust_rte_mbuf_refcnt_read(m: *const rte_mbuf) -> u16;
//pub fn rust_rte_mbuf_refcnt_set(m: *mut rte_mbuf, new_value: u16);
//pub fn rust_rte_mbuf_refcnt_update(m: *mut rte_mbuf, value: i16) -> u16;
//pub fn rust_rte_mbuf_to_baddr(md: *mut rte_mbuf) -> *mut c_char;
//
//
///**
// * IPv4 fragmentation.
// *
// * This function implements the fragmentation of IPv4 packets.
// *
// * @param pkt_in
// *   The input packet.
// * @param pkts_out
// *   Array storing the output fragments.
// * @param nb_pkts_out
// *   Number of fragments.
// * @param mtu_size
// *   Size in bytes of the Maximum Transfer Unit (MTU) for the outgoing IPv4
// *   datagrams. This value includes the size of the IPv4 header.
// * @param pool_direct
// *   MBUF pool used for allocating direct buffers for the output fragments.
// * @param pool_indirect
// *   MBUF pool used for allocating indirect buffers for the output fragments.
// * @return
// *   Upon successful completion - number of output fragments placed
// *   in the pkts_out array.
// *   Otherwise - (-1) * errno.
// */
//int32_t rte_ipv4_fragment_packet(struct rte_mbuf *pkt_in,
//struct rte_mbuf **pkts_out,
//uint16_t nb_pkts_out, uint16_t mtu_size,
//struct rte_mempool *pool_direct,
//struct rte_mempool *pool_indirect);
