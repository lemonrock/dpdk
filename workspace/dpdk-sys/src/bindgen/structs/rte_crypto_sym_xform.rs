// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_crypto_sym_xform
{
	pub next: *mut rte_crypto_sym_xform,
	pub type_: rte_crypto_sym_xform_type,
	pub _1: rte_crypto_sym_xform_1,
}

impl Default for rte_crypto_sym_xform
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_crypto_sym_xform
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_crypto_sym_xform {{ next: {:?}, type: {:?}, _1: {:?} }}", self.next, self.type_, self._1)
	}
}
