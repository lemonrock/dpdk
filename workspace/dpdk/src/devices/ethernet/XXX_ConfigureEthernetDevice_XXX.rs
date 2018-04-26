// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub enum ConfigureEthernetDevice
{
	Broadcom_bnxt
	{
	
	},
	
	Cisco_enic_v3,
	
	Intel_i40e,
	
	Intel_igb,
	
	Intel_ixgbe,
	
	Mellanox_mlx4,
	
	Mellanox_mlx5,
	
	Solarflare_sfc,
	
	Virtual_tap,
	
	Virtual_bonding,
	
	Virtual_ipsec,
	// Bonding
	
	// IPSec
}

impl ConfigureEthernetDevice
{
	#[inline(always)]
	pub fn configure_for_receive(&self)
	{
		use self::ConfigureEthernetDevice::*;
		
		// Supported ptypes: rte_eth_dev_get_supported_ptypes
		
		match *self
		{
			Intel_i40e =>
				{
					// CRC offloading
					offloads:DEV_RX_OFFLOAD_CRC_STRIP
					
					// VLAN offloading
					// Mellanox4/5, ixgbe, bnxt, enic, i40e, igb but not solarflare or tap (?)
					offloads:DEV_RX_OFFLOAD_VLAN_STRIP,DEV_RX_OFFLOAD_VLAN_FILTER,DEV_RX_OFFLOAD_VLAN_EXTEND  aka DEV_RX_OFFLOAD_VLAN
					rte_eth_dev_set_vlan_offload();
					// Sets: mbuf.ol_flags:PKT_RX_VLAN_STRIPPED, mbuf.vlan_tci, ?removes from ethernet header?
					
					// QinQ offload - only i40e
					offloads:DEV_RX_OFFLOAD_QINQ_STRIP
					// Sets: mbuf.ol_flags:PKT_RX_QINQ_STRIPPED et al, mbuf.outer_vlan_tci and mbu.vlan_tci, ?removes from ethernet header?
					
					// L3 checksum offload
					offloads:DEV_RX_OFFLOAD_IPV4_CKSUM
					// Sets: mbuf.ol_flags:PKT_RX_IP_CKSUM_UNKNOWN, mbuf.ol_flags:PKT_RX_IP_CKSUM_GOOD, mbuf.ol_flags:PKT_RX_IP_CKSUM_BAD, mbuf.ol_flags:PKT_RX_IP_CKSUM_NONE
					
					// L4 checksum offload (UDP)
					offloads:DEV_RX_OFFLOAD_UDP_CKSUM
					// Sets: mbuf.ol_flags:PKT_RX_L4_CKSUM_UNKNOWN, mbuf.ol_flags:PKT_RX_L4_CKSUM_GOOD, mbuf.ol_flags:PKT_RX_L4_CKSUM_BAD, mbuf.ol_flags:PKT_RX_L4_CKSUM_NONE
					
					// L4 checksum offload (TCP)
					offloads::DEV_RX_OFFLOAD_TCP_CKSUM
					// Sets: mbuf.ol_flags:PKT_RX_L4_CKSUM_UNKNOWN, mbuf.ol_flags:PKT_RX_L4_CKSUM_GOOD, mbuf.ol_flags:PKT_RX_L4_CKSUM_BAD, mbuf.ol_flags:PKT_RX_L4_CKSUM_NONE (same as for UDP)
					
					
					DEV_RX_OFFLOAD_TCP_LRO
					Only Intel_ixgbe, and then, only ixgbe_mac_82599EB and ixgbe_mac_X540
					Prefer rte_eth_rxmode.enable_lro
					
					DEV_RX_OFFLOAD_JUMBO_FRAME
					Only SFC, Mellanox_mlx5 check for it; neither pay much attention to it.
				
				DEV_RX_OFFLOAD_SCATTER
					Tap, SFC, Mellanox_mlx5, Mellanox_mlx4, Cisco_enic_v3 pay attention to it.
				
				rte_eth_dev_set_mtu()
				
				}
		}
	}
}
