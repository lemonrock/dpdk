// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A packet buffer pool reference makes it possible to reference to memory pools when deserializing with Serde.
///
/// They act as a sort-of reference counted ('Rc') smart pointer.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct PacketBufferPoolReference(u16);

impl PacketBufferPoolReference
{
	/// Finds a memory pool.
	#[inline(always)]
	pub fn find(&self) -> Option<NonNull<rte_mempool>>
	{
		let name = self.name();
		
		NonNull::new(unsafe { rte_mempool_lookup(name.as_ptr()) })
	}
	
	/// Name.
	#[inline(always)]
	pub fn name(&self) -> CString
	{
		let name = format!("PacketBufferPool{}", self.0);
		CString::new(name.as_str()).unwrap()
	}
}
