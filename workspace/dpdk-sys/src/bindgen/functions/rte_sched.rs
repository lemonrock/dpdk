// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_sched_pipe_config(port: *mut rte_sched_port, subport_id: u32, pipe_id: u32, pipe_profile: i32) -> c_int;
	pub fn rte_sched_port_config(params: *mut rte_sched_port_params) -> *mut rte_sched_port;
	pub fn rte_sched_port_dequeue(port: *mut rte_sched_port, pkts: *mut *mut rte_mbuf, n_pkts: u32) -> c_int;
	pub fn rte_sched_port_enqueue(port: *mut rte_sched_port, pkts: *mut *mut rte_mbuf, n_pkts: u32) -> c_int;
	pub fn rte_sched_port_free(port: *mut rte_sched_port);
	pub fn rte_sched_port_get_memory_footprint(params: *mut rte_sched_port_params) -> u32;
	pub fn rte_sched_port_pipe_profile_add(port: *mut rte_sched_port, params: *mut rte_sched_pipe_params, pipe_profile_id: *mut u32) -> c_int;
	pub fn rte_sched_port_pkt_read_color(pkt: *const rte_mbuf) -> rte_meter_color;
	pub fn rte_sched_port_pkt_read_tree_path(pkt: *const rte_mbuf, subport: *mut u32, pipe: *mut u32, traffic_class: *mut u32, queue: *mut u32);
	pub fn rte_sched_port_pkt_write(pkt: *mut rte_mbuf, subport: u32, pipe: u32, traffic_class: u32, queue: u32, color: rte_meter_color);
	pub fn rte_sched_queue_read_stats(port: *mut rte_sched_port, queue_id: u32, stats: *mut rte_sched_queue_stats, qlen: *mut u16) -> c_int;
	pub fn rte_sched_subport_config(port: *mut rte_sched_port, subport_id: u32, params: *mut rte_sched_subport_params) -> c_int;
	pub fn rte_sched_subport_read_stats(port: *mut rte_sched_port, subport_id: u32, stats: *mut rte_sched_subport_stats, tc_ov: *mut u32) -> c_int;
}
