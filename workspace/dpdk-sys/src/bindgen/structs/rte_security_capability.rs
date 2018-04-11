// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_security_capability
{
	pub action: rte_security_session_action_type,
	pub protocol: rte_security_session_protocol,
	pub __bindgen_anon_1: rte_security_capability__bindgen_ty_1,
	pub crypto_capabilities: *const rte_cryptodev_capabilities,
	pub ol_flags: u32,
}

impl Default for rte_security_capability
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_security_capability
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_security_capability {{ action: {:?}, protocol: {:?}, __bindgen_anon_1: {:?}, crypto_capabilities: {:?} }}", self.action, self.protocol, self.__bindgen_anon_1, self.crypto_capabilities)
	}
}
