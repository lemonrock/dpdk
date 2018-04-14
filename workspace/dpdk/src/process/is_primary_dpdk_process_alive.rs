// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Is the primary DPDK process alive?
#[inline(always)]
pub fn is_primary_dpdk_process_alive(primary_process_configuration_file_path: Option<&Path>) -> bool
{
	if let Some(primary_process_configuration_file_path) = primary_process_configuration_file_path
	{
		let c_string = primary_process_configuration_file_path.to_c_string();

		isTrue(unsafe { rte_eal_primary_proc_alive(c_string.as_ptr()) })
	}
	else
	{
		isTrue(unsafe { rte_eal_primary_proc_alive(null()) })
	}
}
