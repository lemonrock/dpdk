// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Hardware offload packet type.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct HardwareOffloadPacketType(u32);

impl From<u32> for HardwareOffloadPacketType
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		HardwareOffloadPacketType(value)
	}
}
impl Into<u32> for HardwareOffloadPacketType
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}

impl HardwareOffloadPacketType
{
	/// Layer 2 packet type.
	#[inline(always)]
	pub fn layer_2_packet_type(self) -> HardwareOffloadLayer2PacketType
	{
		use self::HardwareOffloadLayer2PacketType::*;
		use self::HardwareOffloadCategorisedLayer2PacketType::*;
		
		// Only bits 3:0 (0x0F) are significant.
		match self.0 & RTE_PTYPE_L2_MASK
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
			
			_ => HardwareOffloadLayer2PacketType::Other
		}
	}
	
	/// Layer 3 packet type.
	#[inline(always)]
	pub fn layer_3_packet_type(self) -> HardwareOffloadLayer3PacketType
	{
		use self::HardwareOffloadLayer3PacketType::*;
		use self::HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType::*;
		
		// Only bits 7:4 (0xF0) are significant.
		match self.0 & RTE_PTYPE_L3_MASK
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
	
	/// Layer 4 packet type.
	#[inline(always)]
	pub fn layer_4_packet_type(self) -> HardwareOffloadLayer4PacketType
	{
		use self::HardwareOffloadLayer4PacketType::*;
		
		// Only bits 11:8 (0x0F00) are significant.
		match self.0 & RTE_PTYPE_L4_MASK
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
	
	/// Tunnel packet type.
	#[inline(always)]
	pub fn tunnel_packet_type(self) -> HardwareOffloadTunnelPacketType
	{
		use self::HardwareOffloadTunnelPacketType::*;
		
		// Only bits 15:12 (0xF000) are significant.
		match self.0 & RTE_PTYPE_TUNNEL_MASK
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
	
	/// Tunnel layer 2 packet type.
	#[inline(always)]
	pub fn tunnel_inner_layer_2_packet_type(self) -> HardwareOffloadLayer2PacketType
	{
		use self::HardwareOffloadLayer2PacketType::*;
		use self::HardwareOffloadCategorisedLayer2PacketType::*;
		
		// Only bits 19:16 are significant.
		match self.0 & RTE_PTYPE_INNER_L2_MASK
		{
			0 => Unknown,
			
			RTE_PTYPE_INNER_L2_ETHER => Ethernet(None),
			
			RTE_PTYPE_INNER_L2_ETHER_VLAN => Ethernet(Some(VirtualLan)),
			
			RTE_PTYPE_INNER_L2_ETHER_QINQ => Ethernet(Some(QinQVirtualLan)),
			
			_ => Ethernet(Some(HardwareOffloadCategorisedLayer2PacketType::Other))
		}
	}
	
	/// Tunnel layer 3 packet type.
	#[inline(always)]
	pub fn tunnel_inner_layer_3_packet_type(self) -> HardwareOffloadLayer3PacketType
	{
		use self::HardwareOffloadLayer3PacketType::*;
		use self::HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType::*;
		
		// Only bits 23:20 are significant.
		match self.0 & RTE_PTYPE_INNER_L3_MASK
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
	
	/// Tunnel layer 4 packet type.
	#[inline(always)]
	pub fn tunnel_inner_layer_4_packet_type(self) -> HardwareOffloadLayer4PacketType
	{
		use self::HardwareOffloadLayer4PacketType::*;
		
		// Only bits 27:24 are significant.
		match self.0 & RTE_PTYPE_INNER_L4_MASK
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
		unsafe { CStr::from_ptr(rte_get_ptype_l2_name(self.0)) }
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
	pub fn hardware_offload_layer_3_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l3_name(self.0)) }
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
	pub fn hardware_offload_layer_4_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.0)) }
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
	pub fn hardware_offload_tunnel_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_tunnel_name(self.0)) }
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
	pub fn hardware_offload_tunnel_inner_layer_2_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_inner_l2_name(self.0)) }
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
	pub fn hardware_offload_tunnel_inner_layer_3_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_inner_l3_name(self.0)) }
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
	pub fn hardware_offload_tunnel_inner_layer_4_packet_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rte_get_ptype_l4_name(self.0)) }
	}
}
