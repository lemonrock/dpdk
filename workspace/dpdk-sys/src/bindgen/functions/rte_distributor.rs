// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_distributor_clear_returns(d: *mut rte_distributor);
	pub fn rte_distributor_create(name: *const c_char, socket_id: c_uint, num_workers: c_uint, alg_type: c_uint) -> *mut rte_distributor;
	pub fn rte_distributor_flush(d: *mut rte_distributor) -> c_int;
	pub fn rte_distributor_get_pkt(d: *mut rte_distributor, worker_id: c_uint, pkts: *mut *mut rte_mbuf, oldpkt: *mut *mut rte_mbuf, retcount: c_uint) -> c_int;
	pub fn rte_distributor_poll_pkt(d: *mut rte_distributor, worker_id: c_uint, mbufs: *mut *mut rte_mbuf) -> c_int;
	pub fn rte_distributor_process(d: *mut rte_distributor, mbufs: *mut *mut rte_mbuf, num_mbufs: c_uint) -> c_int;
	pub fn rte_distributor_request_pkt(d: *mut rte_distributor, worker_id: c_uint, oldpkt: *mut *mut rte_mbuf, count: c_uint);
	pub fn rte_distributor_return_pkt(d: *mut rte_distributor, worker_id: c_uint, oldpkt: *mut *mut rte_mbuf, num: c_int) -> c_int;
	pub fn rte_distributor_returned_pkts(d: *mut rte_distributor, mbufs: *mut *mut rte_mbuf, max_mbufs: c_uint) -> c_int;
}
