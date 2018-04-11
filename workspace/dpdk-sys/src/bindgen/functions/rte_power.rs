// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_power_exit(lcore_id: c_uint) -> c_int;
	pub fn rte_power_get_env() -> power_management_env;
	pub fn rte_power_init(lcore_id: c_uint) -> c_int;
	pub fn rte_power_set_env(env: power_management_env) -> c_int;
	pub fn rte_power_unset_env();
}
