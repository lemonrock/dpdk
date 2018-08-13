// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Receive hardware offloading flags.
	pub struct ReceiveHardwareOffloadingFlags: u64
	{
		/// Strip IEEE 802.1Q Virtual LAN tag from incoming packet, so packet data appears to start with an Ether type, ie as a normal Ethernet packet.
		///
		/// A stripped Virtual LAN tag will then be placed in `rte_mbuf`'s `vlan_tci` field and the offload flag `PKT_RX_VLAN_STRIPPED` (and `PKT_RX_VLAN`) set in `rte_mbuf's` `ol_flags` field.
		///
		/// It is recommended that the `FilterVirtualLan` and `VirtualLanExtend` flags are also set.
		const StripVirtualLan_802_1_Q = DEV_RX_OFFLOAD_VLAN_STRIP as u64;
		
		/// Strip IEEE 802.1ad QinQ Virtual LAN tag from incoming packet, so packet data appears to start with an Ether type, ie as a normal Ethernet packet.
		///
		/// A stripped Virtual LAN tag will then be placed in `rte_mbuf`'s `outer_vlan_tci` field and the offload flag `PKT_RX_QINQ_STRIPPED` (and `PKT_RX_VLAN_STRIPPED` and  `PKT_RX_VLAN`) set in `rte_mbuf's` `ol_flags` field.
		/// An 'inner' 802.1Q Virtual LAN tag will also have been stripped and placed in  `rte_mbuf`'s `vlan_tci` field.
		///
		/// Specifying this flag requires the `StripVirtualLan` flag to also be specified even though it is implicit.
		const StripVirtualLan_802_1_ad = DEV_RX_OFFLOAD_QINQ_STRIP as u64;
		
		/// Filtering.
		const VirtualLanFilter = DEV_RX_OFFLOAD_VLAN_FILTER as u64;
		
		/// Extend.
		const VirtualLanExtend = DEV_RX_OFFLOAD_VLAN_EXTEND as u64;
		
		/// Strip Ethernet trailing Frame Check Sequence (FCS).
		///
		/// Most DPDK drivers do this without this flag being specified.
		const StripTrailingFrameCheckSequence = DEV_RX_OFFLOAD_CRC_STRIP as u64;
		
		/// Validate Internet Protocol (IP) version 4 check sum in hardware.
		const ValidateInternetProtocolVersion4CheckSum = DEV_RX_OFFLOAD_IPV4_CKSUM as u64;
		
		/// Validate Internet Protocol (IP) version 4 outer check sum in hardware if this is a tunneled packet.
		const ValidateOuterInternetProtocolVersion4CheckSum = DEV_RX_OFFLOAD_OUTER_IPV4_CKSUM as u64;
		
		/// Validate User Datagram Protocol (UDP) check sum in hardware.
		const ValidateUserDatagrramProtocolCheckSum = DEV_RX_OFFLOAD_UDP_CKSUM as u64;
		
		/// Validate Transmission Control Protocol (TCP) check sum in hardware.
		const ValidateTransmissionControlProtocolCheckSum = DEV_RX_OFFLOAD_TCP_CKSUM as u64;
		
		/// Transmission Control Protocol (TCP) Large Receive Offload (LRO).
		///
		/// Hardware coalseces TCP data segments into one large segment to make processing by software more efficient.
		const TransmissionControlProtocolLargeReceiveOffload = DEV_RX_OFFLOAD_TCP_LRO as u64;
		
		/// Support Jumbo frames.
		///
		/// Most DPDK drivers do this without this flag being specified.
		const SupportJumboFrames = DEV_RX_OFFLOAD_JUMBO_FRAME as u64;
		
		/// Strip MACsec if present.
		///
		/// Not widely supported.
		const MacSecStrip = DEV_RX_OFFLOAD_MACSEC_STRIP as u64;
		
		/// Various IPsec acceleration features.
		///
		/// Not widely supported.
		const IPsecAcceleration = DEV_RX_OFFLOAD_SECURITY as u64;
		
		/// Populate a timestamp field in `rte_mbuf`s `timestamp` field.
		///
		/// Not widely supported.
		///
		/// Some drivers interprete this as relating to IEEE 1488 features, others to just the current network card clock.
		const Timestamp = DEV_RX_OFFLOAD_TIMESTAMP as u64;
		
		/// Header split; meaning uncertain.
		///
		/// Not widely supported.
		///
		/// Some drivers interprete this as relating to IEEE 1488 features, others to just the current network card clock.
		const HeaderSplit = DEV_RX_OFFLOAD_HEADER_SPLIT as u64;
		
		/// A packet is received as a chain (singly linked list) of packets
		///
		/// Allows for more efficient processing by the network card at the expense of making it extremely awkward for software to process a received packet, as various data structures might cross packet boundaries.
		const ReceivePacketAsAChainOfPackets = DEV_RX_OFFLOAD_SCATTER as u64;
	}
}

impl ReceiveHardwareOffloadingFlags
{
	/// Common flags.
	pub const CommonFlags: Self = Self::StripTrailingFrameCheckSequence | Self::StripVirtualLan_802_1_Q | Self::StripVirtualLan_802_1_ad | Self::VirtualLanFilter | Self::VirtualLanExtend | Self::ValidateInternetProtocolVersion4CheckSum | Self::ValidateUserDatagrramProtocolCheckSum | Self::ValidateTransmissionControlProtocolCheckSum | Self::TransmissionControlProtocolLargeReceiveOffload;
	
	/// Common flags with jumbo frames support.
	pub const CommonFlagsWithJumboFramesSupport: Self = Self::CommonFlags | Self::SupportJumboFrames;
}
