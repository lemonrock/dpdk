// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_mp_action_register(name: *const c_char, action: rte_mp_t) -> c_int;
	pub fn rte_mp_action_unregister(name: *const c_char);
	pub fn rte_mp_reply(msg: *mut rte_mp_msg, peer: *const c_char) -> c_int;
	pub fn rte_mp_request_async(req: *mut rte_mp_msg, ts: *const timespec, clb: rte_mp_async_reply_t) -> c_int;
	pub fn rte_mp_request_sync(req: *mut rte_mp_msg, reply: *mut rte_mp_reply, ts: *const timespec) -> c_int;
	pub fn rte_mp_sendmsg(msg: *mut rte_mp_msg) -> c_int;
}
