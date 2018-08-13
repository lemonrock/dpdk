// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Transmit hardware offloading flags.
	pub struct TransmitHardwareOffloadingFlags: u64
	{
		/// Inserts an IEEE 802.1Q Virtual LAN tag from `rte_mbuf`'s `vlan_tci` field if the relevant offload flags are set in  `rte_mbuf`'s `ol_flags` field.
		const InsertVirtualLan_802_1_Q_Tag = DEV_TX_OFFLOAD_VLAN_INSERT as u64;
		
		/// Inserts an IEEE 802.1ad QinQ Virtual LAN tag from `rte_mbuf`'s `outer_vlan_tci` field if the relevant offload flags are set in  `rte_mbuf`'s `ol_flags` field and an IEEE 802.1Q Virtual LAN tag from `rte_mbuf`'s `vlan_tci` field if the relevant offload flags are set in `rte_mbuf`'s `ol_flags` field.
		///
		/// Should always be used in combination with `InsertVirtualLan_802_1_Q_Tag`.
		///
		/// Not widely supported.
		const InsertVirtualLan_802_1_ad_Tag = DEV_TX_OFFLOAD_QINQ_INSERT as u64;
		
		/// Inserts MACsec.
		///
		/// Not widely supported.
		const InsertMacSec = DEV_TX_OFFLOAD_MACSEC_INSERT as u64;
		
		/// Various IPsec acceleration features.
		///
		/// Not widely supported.
		const IPsecAcceleration = DEV_TX_OFFLOAD_SECURITY as u64;
		
		/// Calculate Internet Protocol (IP) version 4 checksum in hardware.
		///
		/// This is widely supported, including by tun/tap.
		const CalculateInternetProtocolVersion4CheckSum = DEV_TX_OFFLOAD_IPV4_CKSUM as u64;
		
		/// Calculate outer tunnel's Internet Protocol (IP) version 4 checksum in hardware.
		const CalculateOuterInternetProtocolVersion4CheckSum = DEV_TX_OFFLOAD_OUTER_IPV4_CKSUM as u64;
		
		/// Calculate User Datagram Protocol (UDP) checksum in hardware.
		///
		/// This is widely supported, including by tun/tap.
		const CalculateUserDatagramProtocolCheckSum = DEV_TX_OFFLOAD_UDP_CKSUM as u64;
		
		/// Calculate Transmission Control Protocol (TCP) checksum in hardware.
		///
		/// This is widely supported, including by tun/tap.
		const CalculateTransmissionControlProtocolCheckSum = DEV_TX_OFFLOAD_TCP_CKSUM as u64;
		
		/// Calculate Stream Control Transmission Protocol (SCTP) checksum in hardware.
		///
		/// Not widely supported.
		const CalculateStreamControlTransmissionProtocolProtocolCheckSum = DEV_TX_OFFLOAD_SCTP_CKSUM as u64;
		
		/// Transmission Control Protocol (TCP) Transmission Segmentation Offload (TSO).
		const TransmissionControlProtocolTransmissionSegmentationOffload = DEV_TX_OFFLOAD_TCP_TSO as u64;
		
		/// User Datagram Protocol (UDP) Transmission Segmentation Offload (TSO).
		const UserDatagramProtocolTransmissionSegmentationOffload = DEV_TX_OFFLOAD_UDP_TSO as u64;
		
		/// Virtual Extensible Local Area Network (VXLAN) tunnel Transmission Segmentation Offload (TSO).
		const VirtualExtensibleLocalAreaNetworkTunnelTransmissionSegmentationOffload = DEV_TX_OFFLOAD_VXLAN_TNL_TSO as u64;
		
		/// Generic Routing Encapsulation (GRE) tunnel Transmission Segmentation Offload (TSO).
		const GenericRoutingEncapsulationTunnelTransmissionSegmentationOffload = DEV_TX_OFFLOAD_GRE_TNL_TSO as u64;
		
		/// Internet Protocol (IP) in Internet Protocol (IP) tunnel Transmission Segmentation Offload (TSO).
		const InternetProtocolInInternetProtocolTunnelTransmissionSegmentationOffload = DEV_TX_OFFLOAD_IPIP_TNL_TSO as u64;
		
		/// Generic Network Virtualization Encapsulation (GENEVE) tunnel Transmission Segmentation Offload (TSO).
		const GenericNetworkVirtualizationEncapsulationTunnelTransmissionSegmentationOffload = DEV_TX_OFFLOAD_GENEVE_TNL_TSO as u64;
		
		/// Any (generic) Internet Protocol (IP) tunnel Transmission Segmentation Offload (TSO).
		///
		/// The `PKT_TX_TUNNEL_IP` flag must be set in `rte_mbuf`'s `packet_type` field (`rte_mbuf._3.packet_type`).
		const AnyInternetProtocolTunnelTransmissionSegmentationOffload = DEV_TX_OFFLOAD_IP_TNL_TSO as u64;
		
		/// Any (generic) User Datagram Protocol (UDP) tunnel Transmission Segmentation Offload (TSO).
		///
		/// The `PKT_TX_TUNNEL_UDP` flag must be set in `rte_mbuf`'s `packet_type` field (`rte_mbuf._3.packet_type`).
		const AnyUserDatagramProtocolTunnelTransmissionSegmentationOffload = DEV_TX_OFFLOAD_UDP_TNL_TSO as u64;
		
		/// Multiple threads can invoke transmit burst (`rte_eth_tx_burst()` and underlying functionality) concurrently without obtaining a lock to serialize access.
		///
		/// Only supported currently by Cavium Octeon drivers.
		const MultipleThreadsCanInvokeTransmitBurstConcurrentlyWithoutALock = DEV_TX_OFFLOAD_MT_LOCKFREE as u64;
		
		/// A driver can send a chain (singly linked list) of packet buffers (segments).
		///
		/// This is widely supported, including by tun/tap.
		const PacketBufferChainsSupported = DEV_TX_OFFLOAD_MULTI_SEGS as u64;
		
		/// A driver can use the `rte_mbuf` fast free methods.
		///
		/// An application must ensure that all packets come from the same mempool and the reference count is 1.
		const PacketBufferFastFree = DEV_TX_OFFLOAD_MBUF_FAST_FREE as u64;
	}
}

impl TransmitHardwareOffloadingFlags
{
	/// Common flags.
	#[inline(always)]
	pub fn common_flags() -> Self
	{
		Self::InsertVirtualLan_802_1_Q_Tag | Self::InsertVirtualLan_802_1_ad_Tag | Self::CalculateInternetProtocolVersion4CheckSum | Self::CalculateUserDatagramProtocolCheckSum | Self::CalculateTransmissionControlProtocolCheckSum | Self::TransmissionControlProtocolTransmissionSegmentationOffload | Self::UserDatagramProtocolTransmissionSegmentationOffload
	}
}
