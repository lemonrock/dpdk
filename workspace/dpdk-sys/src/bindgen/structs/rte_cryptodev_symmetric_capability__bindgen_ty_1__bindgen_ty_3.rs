// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_cryptodev_symmetric_capability__bindgen_ty_1__bindgen_ty_3
{
	pub algo: rte_crypto_aead_algorithm,
	pub block_size: u16,
	pub key_size: rte_crypto_param_range,
	pub digest_size: rte_crypto_param_range,
	pub aad_size: rte_crypto_param_range,
	pub iv_size: rte_crypto_param_range,
}

impl Default for rte_cryptodev_symmetric_capability__bindgen_ty_1__bindgen_ty_3
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_cryptodev_symmetric_capability__bindgen_ty_1__bindgen_ty_3
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_cryptodev_symmetric_capability__bindgen_ty_1__bindgen_ty_3 {{ algo: {:?}, key_size: {:?}, digest_size: {:?}, aad_size: {:?}, iv_size: {:?} }}", self.algo, self.key_size, self.digest_size, self.aad_size, self.iv_size)
	}
}
