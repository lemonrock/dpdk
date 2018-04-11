// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_metrics_get_names(names: *mut rte_metric_name, capacity: u16) -> c_int;
	pub fn rte_metrics_get_values(port_id: c_int, values: *mut rte_metric_value, capacity: u16) -> c_int;
	pub fn rte_metrics_init(socket_id: c_int);
	pub fn rte_metrics_reg_name(name: *const c_char) -> c_int;
	pub fn rte_metrics_reg_names(names: *const *const c_char, cnt_names: u16) -> c_int;
	pub fn rte_metrics_update_value(port_id: c_int, key: u16, value: u64) -> c_int;
	pub fn rte_metrics_update_values(port_id: c_int, key: u16, values: *const u64, count: u32) -> c_int;
}
