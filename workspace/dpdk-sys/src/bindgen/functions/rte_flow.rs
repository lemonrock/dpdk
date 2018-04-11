// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_flow_classifier_create(params: *mut rte_flow_classifier_params) -> *mut rte_flow_classifier;
	pub fn rte_flow_classifier_free(cls: *mut rte_flow_classifier) -> c_int;
	pub fn rte_flow_classifier_query(cls: *mut rte_flow_classifier, pkts: *mut *mut rte_mbuf, nb_pkts: u16, rule: *mut rte_flow_classify_rule, stats: *mut rte_flow_classify_stats) -> c_int;
	pub fn rte_flow_classify_table_create(cls: *mut rte_flow_classifier, params: *mut rte_flow_classify_table_params) -> c_int;
	pub fn rte_flow_classify_table_entry_add(cls: *mut rte_flow_classifier, attr: *const rte_flow_attr, pattern: *const rte_flow_item, actions: *const rte_flow_action, key_found: *mut c_int, error: *mut rte_flow_error) -> *mut rte_flow_classify_rule;
	pub fn rte_flow_classify_table_entry_delete(cls: *mut rte_flow_classifier, rule: *mut rte_flow_classify_rule) -> c_int;
	pub fn rte_flow_classify_validate(cls: *mut rte_flow_classifier, attr: *const rte_flow_attr, pattern: *const rte_flow_item, actions: *const rte_flow_action, error: *mut rte_flow_error) -> c_int;
	pub fn rte_flow_copy(fd: *mut rte_flow_desc, len: usize, attr: *const rte_flow_attr, items: *const rte_flow_item, actions: *const rte_flow_action) -> usize;
	pub fn rte_flow_create(port_id: u16, attr: *const rte_flow_attr, pattern: *const rte_flow_item, actions: *const rte_flow_action, error: *mut rte_flow_error) -> *mut rte_flow;
	pub fn rte_flow_destroy(port_id: u16, flow: *mut rte_flow, error: *mut rte_flow_error) -> c_int;
	pub fn rte_flow_error_set(error: *mut rte_flow_error, code: c_int, type_: rte_flow_error_type, cause: *const c_void, message: *const c_char) -> c_int;
	pub fn rte_flow_flush(port_id: u16, error: *mut rte_flow_error) -> c_int;
	pub fn rte_flow_isolate(port_id: u16, set: c_int, error: *mut rte_flow_error) -> c_int;
	pub fn rte_flow_ops_get(port_id: u16, error: *mut rte_flow_error) -> *const rte_flow_ops;
	pub fn rte_flow_query(port_id: u16, flow: *mut rte_flow, action: rte_flow_action_type, data: *mut c_void, error: *mut rte_flow_error) -> c_int;
	pub fn rte_flow_validate(port_id: u16, attr: *const rte_flow_attr, pattern: *const rte_flow_item, actions: *const rte_flow_action, error: *mut rte_flow_error) -> c_int;
}
