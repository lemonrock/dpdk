// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_security_capabilities_get(instance: *mut rte_security_ctx) -> *const rte_security_capability;
	pub fn rte_security_capability_get(instance: *mut rte_security_ctx, idx: *mut rte_security_capability_idx) -> *const rte_security_capability;
	pub fn rte_security_get_userdata(instance: *mut rte_security_ctx, md: u64) -> *mut c_void;
	pub fn rte_security_session_create(instance: *mut rte_security_ctx, conf: *mut rte_security_session_conf, mp: *mut rte_mempool) -> *mut rte_security_session;
	pub fn rte_security_session_destroy(instance: *mut rte_security_ctx, sess: *mut rte_security_session) -> c_int;
	pub fn rte_security_session_get_size(instance: *mut rte_security_ctx) -> c_uint;
	pub fn rte_security_session_stats_get(instance: *mut rte_security_ctx, sess: *mut rte_security_session, stats: *mut rte_security_stats) -> c_int;
	pub fn rte_security_session_update(instance: *mut rte_security_ctx, sess: *mut rte_security_session, conf: *mut rte_security_session_conf) -> c_int;
	pub fn rte_security_set_pkt_metadata(instance: *mut rte_security_ctx, sess: *mut rte_security_session, mb: *mut rte_mbuf, params: *mut c_void) -> c_int;
}
