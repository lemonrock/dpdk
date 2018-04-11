// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_eth_bond_8023ad_agg_selection_get(port_id: u16) -> c_int;
	pub fn rte_eth_bond_8023ad_agg_selection_set(port_id: u16, agg_selection: rte_bond_8023ad_agg_selection) -> c_int;
	pub fn rte_eth_bond_8023ad_conf_get(port_id: u16, conf: *mut rte_eth_bond_8023ad_conf) -> c_int;
	pub fn rte_eth_bond_8023ad_dedicated_queues_disable(port_id: u16) -> c_int;
	pub fn rte_eth_bond_8023ad_dedicated_queues_enable(port_id: u16) -> c_int;
	pub fn rte_eth_bond_8023ad_ext_collect(port_id: u16, slave_id: u16, enabled: c_int) -> c_int;
	pub fn rte_eth_bond_8023ad_ext_collect_get(port_id: u16, slave_id: u16) -> c_int;
	pub fn rte_eth_bond_8023ad_ext_distrib(port_id: u16, slave_id: u16, enabled: c_int) -> c_int;
	pub fn rte_eth_bond_8023ad_ext_distrib_get(port_id: u16, slave_id: u16) -> c_int;
	pub fn rte_eth_bond_8023ad_ext_slowtx(port_id: u16, slave_id: u16, lacp_pkt: *mut rte_mbuf) -> c_int;
	pub fn rte_eth_bond_8023ad_setup(port_id: u16, conf: *mut rte_eth_bond_8023ad_conf) -> c_int;
	pub fn rte_eth_bond_8023ad_slave_info(port_id: u16, slave_id: u16, conf: *mut rte_eth_bond_8023ad_slave_info) -> c_int;
	pub fn rte_eth_bond_active_slaves_get(bonded_port_id: u16, slaves: *mut u16, len: u16) -> c_int;
	pub fn rte_eth_bond_create(name: *const c_char, mode: u8, socket_id: u8) -> c_int;
	pub fn rte_eth_bond_free(name: *const c_char) -> c_int;
	pub fn rte_eth_bond_link_down_prop_delay_get(bonded_port_id: u16) -> c_int;
	pub fn rte_eth_bond_link_down_prop_delay_set(bonded_port_id: u16, delay_ms: u32) -> c_int;
	pub fn rte_eth_bond_link_monitoring_get(bonded_port_id: u16) -> c_int;
	pub fn rte_eth_bond_link_monitoring_set(bonded_port_id: u16, internal_ms: u32) -> c_int;
	pub fn rte_eth_bond_link_up_prop_delay_get(bonded_port_id: u16) -> c_int;
	pub fn rte_eth_bond_link_up_prop_delay_set(bonded_port_id: u16, delay_ms: u32) -> c_int;
	pub fn rte_eth_bond_mac_address_reset(bonded_port_id: u16) -> c_int;
	pub fn rte_eth_bond_mac_address_set(bonded_port_id: u16, mac_addr: *mut ether_addr) -> c_int;
	pub fn rte_eth_bond_mode_get(bonded_port_id: u16) -> c_int;
	pub fn rte_eth_bond_mode_set(bonded_port_id: u16, mode: u8) -> c_int;
	pub fn rte_eth_bond_primary_get(bonded_port_id: u16) -> c_int;
	pub fn rte_eth_bond_primary_set(bonded_port_id: u16, slave_port_id: u16) -> c_int;
	pub fn rte_eth_bond_slave_add(bonded_port_id: u16, slave_port_id: u16) -> c_int;
	pub fn rte_eth_bond_slave_remove(bonded_port_id: u16, slave_port_id: u16) -> c_int;
	pub fn rte_eth_bond_slaves_get(bonded_port_id: u16, slaves: *mut u16, len: u16) -> c_int;
	pub fn rte_eth_bond_xmit_policy_get(bonded_port_id: u16) -> c_int;
	pub fn rte_eth_bond_xmit_policy_set(bonded_port_id: u16, policy: u8) -> c_int;
}
