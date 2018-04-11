// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_kni_alloc(pktmbuf_pool: *mut rte_mempool, conf: *const rte_kni_conf, ops: *mut rte_kni_ops) -> *mut rte_kni;
	pub fn rte_kni_close();
	pub fn rte_kni_get(name: *const c_char) -> *mut rte_kni;
	pub fn rte_kni_get_name(kni: *const rte_kni) -> *const c_char;
	pub fn rte_kni_handle_request(kni: *mut rte_kni) -> c_int;
	pub fn rte_kni_init(max_kni_ifaces: c_uint);
	pub fn rte_kni_register_handlers(kni: *mut rte_kni, ops: *mut rte_kni_ops) -> c_int;
	pub fn rte_kni_release(kni: *mut rte_kni) -> c_int;
	pub fn rte_kni_rx_burst(kni: *mut rte_kni, mbufs: *mut *mut rte_mbuf, num: c_uint) -> c_uint;
	pub fn rte_kni_tx_burst(kni: *mut rte_kni, mbufs: *mut *mut rte_mbuf, num: c_uint) -> c_uint;
	pub fn rte_kni_unregister_handlers(kni: *mut rte_kni) -> c_int;
}
