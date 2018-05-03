// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_cpu_check_supported();
	pub fn rte_cpu_get_flag_enabled(feature: rte_cpu_flag_t) -> c_int;
	pub fn rte_cpu_get_flag_name(feature: rte_cpu_flag_t) -> *const c_char;
	pub fn rte_cpu_getauxval(type_: c_ulong) -> c_ulong;
	pub fn rte_cpu_is_supported() -> c_int;
	pub fn rte_cpu_strcmp_auxval(type_: c_ulong, str: *const c_char) -> c_int;
}
