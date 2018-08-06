// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Packet buffer receive offload feature flags.
	///
	/// The most significant 3-bits are reserved for generic `mbuf` flags.
	pub struct PacketReceiveOffloadFeaturesFlags: u64
	{
		/// The RX packet is a 802.1Q VLAN packet.
		///
		/// If the flag `VLAN_STRIPPED` is also present, the VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.vlan_tci`, else it is still present.
		const VLAN = 1 << 0;

		/// A VLAN has been stripped by the hardware and saved in `rte_mbuf.vlan_tci`.
		///
		/// This can only happen if VLAN stripping is enabled in the RX configuration of the poll-mode driver (PMD).
		///
		/// When `VLAN_STRIPPED` is set, `VLAN` is also be set.
		const VLAN_STRIPPED = 1 << 6;
		
		/// The packet is a QinQ VLAN.
		///
		/// If the flag `QINQ_STRIPPED` is also present:-
		///
		/// * the outer VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.outer_vlan_tci`;
		/// * the inner VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.vlan_tci`.
		///
		/// If that flag is not present, then the inner and outer VLAN headers are still present.
		///
		/// ?The flag `VLAN` should also be set?
		const QINQ = 1 << 20;
		
		///	Indicated hardware stripping of QinQ VLAN data has occurred.
		///
		/// This can only happen if VLAN stripping is enabled in the RX configuration of the poll-mode driver (PMD).
		///
		/// The:-
		///
		/// * the outer VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.outer_vlan_tci`;
		/// * the inner VLAN header (Traffic Control Information, TCI) has been stripped by the hardware and has been saved in `rte_mbuf.vlan_tci`.
		///
		/// When `QINQ_STRIPPED` is set, the following flags are also set:-
		///
		/// * `QINQ`
		/// * `VLAN`
		/// * `VLAN_STRIPPED`
		const QINQ_STRIPPED = 1 << 15;
		
		/// RX packet with Receive Side Scaling ('RSS') hash result.
		const RSS_HASH = 1 << 1;
		
		/// RX packet with Flow Director ('FDIR') matched hash result.
		///
		/// If flags `FDIR_ID` or `FDIR_FLX` are not set, then just the flow director identifier is set.
		const FDIR = 1 << 2;
		
		/// Flow Director hash and filter identifier has been reported if the Flow Director ('FDIR') matched hash result.
		///
		/// Flag `FDIR` is also set.
		const FDIR_ID = 1 << 13;
		
		/// Flow Director flexible bytes reported if the the Flow Director ('FDIR') matched hash result.
		///
		/// Flag `FDIR` is also set.
		const FDIR_FLX = 1 << 14;
		
		/// This flag was set when the layer 4 checksum (TCP, UDP, possibly SCTP) of a packet was detected as wrong by the network card hardware.
		///
		/// Checking this flag alone is deprecated: instead, check the 2 bits of `L4_CKSUM_MASK`.
		#[deprecated]
		const L4_CKSUM_BAD = 1 << 3;

		/// This flag was set when the internet protocol (IP) checksum of a packet was detected as wrong by the network card hardware.
		///
		/// Checking this flag alone is deprecated: instead, check the 2 bits of `IP_CKSUM_MASK`.
		#[deprecated]
		const IP_CKSUM_BAD = 1 << 4;

		/// External internet protocol (IP) header checksum error.
		const EIP_CKSUM_BAD = 1 << 5;
		
		/// IEEE1588 (802.1AS) Precision time protocol (PTP) Ethernet Layer 2 'PT' packet.
		const IEEE1588_PTP = 1 << 9;
		
		/// IEEE1588 (802.1AS) Precision time protocol (PTP) Ethernet Layer 2 / Layer 4 timestamped packet.
		///
		/// If the flag `TIMESTAMP` is also present, the timestamp has been stripped by the hardware and saved in `rte_mbuf.timestamp`.
		const IEEE1588_TMST = 1 << 10;

		/// A IEEE1588 (802.1AS) Precision time protocol (PTP) timestamp has been stripped by the hardware and saved in `rte_mbuf.timestamp`.
		///
		/// When `TIMESTAMP` is set, `IEEE1588_TMST` should also be set.
		const TIMESTAMP = 1 << 17;
		
		/// Large receive offload.
		///
		/// When packets are coalesced by a hardware or virtual driver, this flag can be set in the RX mbuf, meaning that the `rte_mbuf.tso_segsz` field is valid and is set to the segment size of the original packets.
		const LRO = 1 << 16;
		
		/// Indicates that security offload processing (eg IPsec) was applied on the packet.
		///
		/// The flag `SEC_OFFLOAD` will also be set if security offload processing failed.
		const SEC_OFFLOAD = 1 << 18;
		
		/// Indicates that security offload processing (eg IPsec) was applied on the packet and failed.
		///
		/// The flag `SEC_OFFLOAD` should be set.
		const SEC_OFFLOAD_FAILED = 1 << 19;
	}
}

impl PacketReceiveOffloadFeaturesFlags
{
	/// Security offloading, timestamps and flow director features.
	#[inline(always)]
	pub fn hardware_offloading_categorisation_indicates_an_unwanted_packet(self) -> bool
	{
		const Unwanted: Self = Self::SEC_OFFLOAD | Self::SEC_OFFLOAD_FAILED | Self::TIMESTAMP | Self::IEEE1588_TMST | Self::IEEE1588_PTP | Self::FDIR_FLX | Self::FDIR_ID | Self::FDIR;
		
		self.intersects(Unwanted)
	}
	
	/// Was IEEE 802.1Q Virtual LAN Tag Control Information (TCI) stripped?
	#[inline(always)]
	pub fn was_vlan_tag_control_information_stripped(self) -> bool
	{
		self.contains(PacketReceiveOffloadFeaturesFlags::VLAN_STRIPPED)
	}
	
	/// Was IEEE 802.1Q Virtual LAN Tag Control Information (TCI) stripped?
	#[inline(always)]
	pub fn was_vlan_qinq_tag_control_information_stripped(self) -> bool
	{
		self.contains(PacketReceiveOffloadFeaturesFlags::QINQ_STRIPPED)
	}
	
	//noinspection SpellCheckingInspection
	/// Determines the internet protocol (IP) version 4 check sum status.
	#[inline(always)]
	pub fn internet_protocol_version_4_check_sum_status(self) -> HardwareOffloadCheckSumStatus
	{
		use self::HardwareOffloadCheckSumStatus::*;
		
		const IP_CKSUM_UNKNOWN: u64 = 0;
		const L4_CKSUM_BAD: u64 = 1 << 4;
		const L4_CKSUM_GOOD: u64 = 1 << 7;
		const IP_CKSUM_NONE: u64 = L4_CKSUM_BAD | L4_CKSUM_GOOD;
		
		const IP_CKSUM_MASK: u64 = IP_CKSUM_NONE;
		
		match self.bits & IP_CKSUM_MASK
		{
			IP_CKSUM_UNKNOWN => NoInformationKnown,
			L4_CKSUM_BAD => Bad,
			L4_CKSUM_GOOD => Good,
			IP_CKSUM_NONE => IncorrectButInternetProtocolHeaderIntegrityVerified,
			
			invalid @ _ => panic!("Invalid checksum flags '{}'", invalid),
		}
	}
	
	//noinspection SpellCheckingInspection
	/// Determines the layer 4 (TCP, UDP, SCTP) checksum check sum status.
	#[inline(always)]
	pub fn layer_4_check_sum_status(self) -> HardwareOffloadCheckSumStatus
	{
		use self::HardwareOffloadCheckSumStatus::*;
		
		const L4_CKSUM_UNKNOWN: u64 = 0;
		const L4_CKSUM_BAD: u64 = 1 << 3;
		const L4_CKSUM_GOOD: u64 = 1 << 8;
		const L4_CKSUM_NONE: u64 = L4_CKSUM_BAD | L4_CKSUM_GOOD;
		
		const L4_CKSUM_MASK: u64 = L4_CKSUM_NONE;
		
		match self.bits & L4_CKSUM_MASK
		{
			L4_CKSUM_UNKNOWN => NoInformationKnown,
			L4_CKSUM_BAD => Bad,
			L4_CKSUM_GOOD => Good,
			L4_CKSUM_NONE => IncorrectButLayer4DataIntegrityVerified,
			
			invalid @ _ => panic!("Invalid checksum flags '{}'", invalid),
		}
	}
}
