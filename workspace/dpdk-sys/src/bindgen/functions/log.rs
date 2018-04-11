// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_log(level: u32, logtype: u32, format: *const c_char, ...) -> c_int;
	pub fn rte_log_cur_msg_loglevel() -> c_int;
	pub fn rte_log_cur_msg_logtype() -> c_int;
	pub fn rte_log_dump(f: *mut FILE);
	pub fn rte_log_get_global_level() -> u32;
	pub fn rte_log_get_level(logtype: u32) -> c_int;
	pub fn rte_log_register(name: *const c_char) -> c_int;
	pub fn rte_log_set_global_level(level: u32);
	pub fn rte_log_set_level(logtype: u32, level: u32) -> c_int;
	pub fn rte_log_set_level_regexp(pattern: *const c_char, level: u32) -> c_int;
	pub fn rte_openlog_stream(f: *mut FILE) -> c_int;
}
