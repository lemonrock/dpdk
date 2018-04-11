// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_crypto_auth_xform
{
	pub op: rte_crypto_auth_operation,
	pub algo: rte_crypto_auth_algorithm,
	pub key: rte_crypto_auth_xform__bindgen_ty_1,
	pub iv: rte_crypto_auth_xform__bindgen_ty_2,
	pub digest_length: u16,
}

impl Default for rte_crypto_auth_xform
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_crypto_auth_xform
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_crypto_auth_xform {{ op: {:?}, algo: {:?}, key: {:?}, iv: {:?} }}", self.op, self.algo, self.key, self.iv)
	}
}
