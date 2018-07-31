// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl PacketBufferReceiveOffloadFeaturesFlags
{
	//noinspection SpellCheckingInspection
	/// Determines the internet protocol (IP) checksum status.
	#[inline(always)]
	pub fn internet_protocol_checksum_status(self) -> HardwareOffloadCheckSumStatus
	{
		use self::HardwareOffloadCheckSumStatus::*;
		
		const PKT_RX_IP_CKSUM_UNKNOWN: u64 = 0;
		const PKT_RX_L4_CKSUM_BAD: u64 = 1 << 4;
		const PKT_RX_L4_CKSUM_GOOD: u64 = 1 << 7;
		const PKT_RX_IP_CKSUM_NONE: u64 = PKT_RX_L4_CKSUM_BAD | PKT_RX_L4_CKSUM_GOOD;
		
		const PKT_RX_IP_CKSUM_MASK: u64 = PKT_RX_IP_CKSUM_NONE;
		
		match self.bits & PKT_RX_IP_CKSUM_MASK
		{
			PKT_RX_IP_CKSUM_UNKNOWN => NoInformationKnown,
			PKT_RX_L4_CKSUM_BAD => Bad,
			PKT_RX_L4_CKSUM_GOOD => Good,
			PKT_RX_IP_CKSUM_NONE => IncorrectButInternetProtocolHeaderIntegrityVerified,
			
			invalid @ _ => panic!("Invalid checksum flags '{}'", invalid),
		}
	}
	
	//noinspection SpellCheckingInspection
	/// Determines the layer 4 (TCP, UDP, SCTP) checksum checksum status.
	#[inline(always)]
	pub fn layer_4_checksum_status(self) -> HardwareOffloadCheckSumStatus
	{
		use self::HardwareOffloadCheckSumStatus::*;
		
		const PKT_RX_L4_CKSUM_UNKNOWN: u64 = 0;
		const PKT_RX_L4_CKSUM_BAD: u64 = 1 << 3;
		const PKT_RX_L4_CKSUM_GOOD: u64 = 1 << 8;
		const PKT_RX_L4_CKSUM_NONE: u64 = PKT_RX_L4_CKSUM_BAD | PKT_RX_L4_CKSUM_GOOD;
		
		const PKT_RX_L4_CKSUM_MASK: u64 = PKT_RX_L4_CKSUM_NONE;
		
		match self.bits & PKT_RX_L4_CKSUM_MASK
		{
			PKT_RX_L4_CKSUM_UNKNOWN => NoInformationKnown,
			PKT_RX_L4_CKSUM_BAD => Bad,
			PKT_RX_L4_CKSUM_GOOD => Good,
			PKT_RX_L4_CKSUM_NONE => IncorrectButLayer4DataIntegrityVerified,
			
			invalid @ _ => panic!("Invalid checksum flags '{}'", invalid),
		}
	}
}

bitflags!
{
	/// Packet buffer receive offload feature flags.
	///
	/// The most significant 3-bits are reserved for generic `mbuf` flags.
	pub struct PacketBufferReceiveOffloadFeaturesFlags: u64
	{
		/// The RX packet is a 802.1Q VLAN packet.
		///
		/// If the flag `PKT_RX_VLAN_STRIPPED` is also present, the VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.vlan_tci`, else it is still present.
		const PKT_RX_VLAN = 1 << 0;

		/// A VLAN has been stripped by the hardware and saved in `rte_mbuf.vlan_tci`.
		///
		/// This can only happen if VLAN stripping is enabled in the RX configuration of the poll-mode driver (PMD).
		///
		/// When `PKT_RX_VLAN_STRIPPED` is set, `PKT_RX_VLAN` is also be set.
		const PKT_RX_VLAN_STRIPPED = 1 << 6;
		
		/// The packet is a QinQ VLAN.
		///
		/// If the flag `PKT_RX_QINQ_STRIPPED` is also present:-
		///
		/// * the outer VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.outer_vlan_tci`;
		/// * the inner VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.vlan_tci`.
		///
		/// If that flag is not present, then the inner and outer VLAN headers are still present.
		///
		/// ?The flag `PKT_RX_VLAN` should also be set?
		const PKT_RX_QINQ = 1 << 20;
		
		///	Indicated hardware stripping of QinQ VLAN data has occurred.
		///
		/// This can only happen if VLAN stripping is enabled in the RX configuration of the poll-mode driver (PMD).
		///
		/// The:-
		///
		/// * the outer VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.outer_vlan_tci`;
		/// * the inner VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.vlan_tci`.
		///
		/// When `PKT_RX_QINQ_STRIPPED` is set, the following flags are also set:-
		///
		/// * `PKT_RX_QINQ`
		/// * `PKT_RX_VLAN`
		/// * `PKT_RX_VLAN_STRIPPED`
		const PKT_RX_QINQ_STRIPPED = 1 << 15;
		
		/// RX packet with Receive Side Scaling ('RSS') hash result.
		const PKT_RX_RSS_HASH = 1 << 1;
		
		/// RX packet with Flow Director ('FDIR') matched hash result.
		///
		/// If flags `PKT_RX_FDIR_ID` or `PKT_RX_FDIR_FLX` are not set, then just the flow director identifier is set.
		const PKT_RX_FDIR = 1 << 2;
		
		/// Flow Director hash and filter identifier has been reported if the Flow Director ('FDIR') matched hash result.
		///
		/// Flag `PKT_RX_FDIR` is also set.
		const PKT_RX_FDIR_ID = 1 << 13;
		
		/// Flow Director flexible bytes reported if the the Flow Director ('FDIR') matched hash result.
		///
		/// Flag `PKT_RX_FDIR` is also set.
		const PKT_RX_FDIR_FLX = 1 << 14;
		
		/// This flag was set when the layer 4 checksum (TCP, UDP, possibly SCTP) of a packet was detected as wrong by the network card hardware.
		///
		/// Checking this flag alone is deprecated: instead, check the 2 bits of `PKT_RX_L4_CKSUM_MASK`.
		#[deprecated]
		const PKT_RX_L4_CKSUM_BAD = 1 << 3;

		/// This flag was set when the internet protocol (IP) checksum of a packet was detected as wrong by the network card hardware.
		///
		/// Checking this flag alone is deprecated: instead, check the 2 bits of `PKT_RX_IP_CKSUM_MASK`.
		#[deprecated]
		const PKT_RX_IP_CKSUM_BAD = 1 << 4;

		/// External internet protocol (IP) header checksum error.
		const PKT_RX_EIP_CKSUM_BAD = 1 << 5;
		
		/// IEEE1588 (802.1AS) Precision time protocol (PTP) Ethernet Layer 2 'PT' packet.
		const PKT_RX_IEEE1588_PTP = 1 << 9;
		
		/// IEEE1588 (802.1AS) Precision time protocol (PTP) Ethernet Layer 2 / Layer 4 timestamped packet.
		///
		/// If the flag `PKT_RX_TIMESTAMP` is also present, the timestamp has been stripped by the hardware and saved in `rte_mbuf.timestamp`.
		const PKT_RX_IEEE1588_TMST = 1 << 10;

		/// A IEEE1588 (802.1AS) Precision time protocol (PTP) timestamp has been stripped by the hardware and saved in `rte_mbuf.timestamp`.
		///
		/// When `PKT_RX_TIMESTAMP` is set, `PKT_RX_IEEE1588_TMST` should also be set.
		const PKT_RX_TIMESTAMP = 1 << 17;
		
		/// Large receive offload.
		///
		/// When packets are coalesced by a hardware or virtual driver, this flag can be set in the RX mbuf, meaning that the `rte_mbuf.tso_segsz` field is valid and is set to the segment size of the original packets.
		const PKT_RX_LRO = 1 << 16;
		
		/// Indicates that security offload processing (eg IPsec) was applied on the packet.
		///
		/// The flag `PKT_RX_SEC_OFFLOAD` will also be set if security offload processing failed.
		const PKT_RX_SEC_OFFLOAD = 1 << 18;
		
		/// Indicates that security offload processing (eg IPsec) was applied on the packet and failed.
		///
		/// The flag `PKT_RX_SEC_OFFLOAD` should be set.
		const PKT_RX_SEC_OFFLOAD_FAILED = 1 << 19;
	}
}
