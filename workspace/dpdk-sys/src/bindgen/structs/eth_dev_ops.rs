// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct eth_dev_ops
{
	pub dev_configure: eth_dev_configure_t,
	pub dev_start: eth_dev_start_t,
	pub dev_stop: eth_dev_stop_t,
	pub dev_set_link_up: eth_dev_set_link_up_t,
	pub dev_set_link_down: eth_dev_set_link_down_t,
	pub dev_close: eth_dev_close_t,
	pub dev_reset: eth_dev_reset_t,
	pub link_update: eth_link_update_t,
	pub is_removed: eth_is_removed_t,
	pub promiscuous_enable: eth_promiscuous_enable_t,
	pub promiscuous_disable: eth_promiscuous_disable_t,
	pub allmulticast_enable: eth_allmulticast_enable_t,
	pub allmulticast_disable: eth_allmulticast_disable_t,
	pub mac_addr_remove: eth_mac_addr_remove_t,
	pub mac_addr_add: eth_mac_addr_add_t,
	pub mac_addr_set: eth_mac_addr_set_t,
	pub set_mc_addr_list: eth_set_mc_addr_list_t,
	pub mtu_set: mtu_set_t,
	pub stats_get: eth_stats_get_t,
	pub stats_reset: eth_stats_reset_t,
	pub xstats_get: eth_xstats_get_t,
	pub xstats_reset: eth_xstats_reset_t,
	pub xstats_get_names: eth_xstats_get_names_t,
	pub queue_stats_mapping_set: eth_queue_stats_mapping_set_t,
	pub dev_infos_get: eth_dev_infos_get_t,
	pub rxq_info_get: eth_rxq_info_get_t,
	pub txq_info_get: eth_txq_info_get_t,
	pub fw_version_get: eth_fw_version_get_t,
	pub dev_supported_ptypes_get: eth_dev_supported_ptypes_get_t,
	pub vlan_filter_set: vlan_filter_set_t,
	pub vlan_tpid_set: vlan_tpid_set_t,
	pub vlan_strip_queue_set: vlan_strip_queue_set_t,
	pub vlan_offload_set: vlan_offload_set_t,
	pub vlan_pvid_set: vlan_pvid_set_t,
	pub rx_queue_start: eth_queue_start_t,
	pub rx_queue_stop: eth_queue_stop_t,
	pub tx_queue_start: eth_queue_start_t,
	pub tx_queue_stop: eth_queue_stop_t,
	pub rx_queue_setup: eth_rx_queue_setup_t,
	pub rx_queue_release: eth_queue_release_t,
	pub rx_queue_count: eth_rx_queue_count_t,
	pub rx_descriptor_done: eth_rx_descriptor_done_t,
	pub rx_descriptor_status: eth_rx_descriptor_status_t,
	pub tx_descriptor_status: eth_tx_descriptor_status_t,
	pub rx_queue_intr_enable: eth_rx_enable_intr_t,
	pub rx_queue_intr_disable: eth_rx_disable_intr_t,
	pub tx_queue_setup: eth_tx_queue_setup_t,
	pub tx_queue_release: eth_queue_release_t,
	pub tx_done_cleanup: eth_tx_done_cleanup_t,
	pub dev_led_on: eth_dev_led_on_t,
	pub dev_led_off: eth_dev_led_off_t,
	pub flow_ctrl_get: flow_ctrl_get_t,
	pub flow_ctrl_set: flow_ctrl_set_t,
	pub priority_flow_ctrl_set: priority_flow_ctrl_set_t,
	pub uc_hash_table_set: eth_uc_hash_table_set_t,
	pub uc_all_hash_table_set: eth_uc_all_hash_table_set_t,
	pub mirror_rule_set: eth_mirror_rule_set_t,
	pub mirror_rule_reset: eth_mirror_rule_reset_t,
	pub udp_tunnel_port_add: eth_udp_tunnel_port_add_t,
	pub udp_tunnel_port_del: eth_udp_tunnel_port_del_t,
	pub l2_tunnel_eth_type_conf: eth_l2_tunnel_eth_type_conf_t,
	pub l2_tunnel_offload_set: eth_l2_tunnel_offload_set_t,
	pub set_queue_rate_limit: eth_set_queue_rate_limit_t,
	pub rss_hash_update: rss_hash_update_t,
	pub rss_hash_conf_get: rss_hash_conf_get_t,
	pub reta_update: reta_update_t,
	pub reta_query: reta_query_t,
	pub get_reg: eth_get_reg_t,
	pub get_eeprom_length: eth_get_eeprom_length_t,
	pub get_eeprom: eth_get_eeprom_t,
	pub set_eeprom: eth_set_eeprom_t,
	pub get_module_info: eth_get_module_info_t,
	pub get_module_eeprom: eth_get_module_eeprom_t,
	pub filter_ctrl: eth_filter_ctrl_t,
	pub get_dcb_info: eth_get_dcb_info,
	pub timesync_enable: eth_timesync_enable_t,
	pub timesync_disable: eth_timesync_disable_t,
	pub timesync_read_rx_timestamp: eth_timesync_read_rx_timestamp_t,
	pub timesync_read_tx_timestamp: eth_timesync_read_tx_timestamp_t,
	pub timesync_adjust_time: eth_timesync_adjust_time,
	pub timesync_read_time: eth_timesync_read_time,
	pub timesync_write_time: eth_timesync_write_time,
	pub xstats_get_by_id: eth_xstats_get_by_id_t,
	pub xstats_get_names_by_id: eth_xstats_get_names_by_id_t,
	pub tm_ops_get: eth_tm_ops_get_t,
	pub mtr_ops_get: eth_mtr_ops_get_t,
	pub pool_ops_supported: eth_pool_ops_supported_t,
}

impl Default for eth_dev_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for eth_dev_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "eth_dev_ops {{  }}")
	}
}
