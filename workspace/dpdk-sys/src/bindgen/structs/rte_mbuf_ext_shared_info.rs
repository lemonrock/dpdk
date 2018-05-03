// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_mbuf_ext_shared_info
{
	pub free_cb: rte_mbuf_extbuf_free_callback_t,
	pub fcb_opaque: *mut c_void,
	pub refcnt_atomic: rte_atomic16_t,
}

impl Default for rte_mbuf_ext_shared_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mbuf_ext_shared_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mbuf_ext_shared_info {{ fcb_opaque: {:?}, refcnt_atomic: {:?} }}", self.fcb_opaque, self.refcnt_atomic)
	}
}
