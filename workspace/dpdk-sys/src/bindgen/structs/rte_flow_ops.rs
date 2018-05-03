// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct rte_flow_ops
{
	pub validate: Option<unsafe extern "C" fn(arg1: *mut rte_eth_dev, arg2: *const rte_flow_attr, arg3: *const rte_flow_item, arg4: *const rte_flow_action, arg5: *mut rte_flow_error) -> c_int>,
	pub create: Option<unsafe extern "C" fn(arg1: *mut rte_eth_dev, arg2: *const rte_flow_attr, arg3: *const rte_flow_item, arg4: *const rte_flow_action, arg5: *mut rte_flow_error) -> *mut rte_flow>,
	pub destroy: Option<unsafe extern "C" fn(arg1: *mut rte_eth_dev, arg2: *mut rte_flow, arg3: *mut rte_flow_error) -> c_int>,
	pub flush: Option<unsafe extern "C" fn(arg1: *mut rte_eth_dev, arg2: *mut rte_flow_error) -> c_int>,
	pub query: Option<unsafe extern "C" fn(arg1: *mut rte_eth_dev, arg2: *mut rte_flow, arg3: *const rte_flow_action, arg4: *mut c_void, arg5: *mut rte_flow_error) -> c_int>,
	pub isolate: Option<unsafe extern "C" fn(arg1: *mut rte_eth_dev, arg2: c_int, arg3: *mut rte_flow_error) -> c_int>,
}

impl Default for rte_flow_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
