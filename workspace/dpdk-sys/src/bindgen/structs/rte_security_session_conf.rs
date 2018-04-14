// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_security_session_conf
{
	pub action_type: rte_security_session_action_type,
	pub protocol: rte_security_session_protocol,
	pub _1: rte_security_session_conf_1,
	pub crypto_xform: *mut rte_crypto_sym_xform,
	pub userdata: *mut c_void,
}

impl Default for rte_security_session_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_security_session_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_security_session_conf {{ action_type: {:?}, protocol: {:?}, _1: {:?}, crypto_xform: {:?}, userdata: {:?} }}", self.action_type, self.protocol, self._1, self.crypto_xform, self.userdata)
	}
}
