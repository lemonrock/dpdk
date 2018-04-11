// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_mempool_memhdr
{
	pub next: rte_mempool_memhdr__bindgen_ty_1,
	pub mp: *mut rte_mempool,
	pub addr: *mut c_void,
	pub __bindgen_anon_1: rte_mempool_memhdr__bindgen_ty_2,
	pub len: usize,
	pub free_cb: rte_mempool_memchunk_free_cb_t,
	pub opaque: *mut c_void,
}

impl Default for rte_mempool_memhdr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mempool_memhdr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mempool_memhdr {{ next: {:?}, mp: {:?}, addr: {:?}, __bindgen_anon_1: {:?}, free_cb: {:?}, opaque: {:?} }}", self.next, self.mp, self.addr, self.__bindgen_anon_1, self.free_cb, self.opaque)
	}
}
