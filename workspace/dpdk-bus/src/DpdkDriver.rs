// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a generic DPDK driver; a sort of super-class.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkDriver<'a>(NonNull<rte_driver>, PhantomData<&'a rte_driver>);

impl<'a> DpdkDriver<'a>
{
	/// Next.
	#[inline(always)]
	pub fn next(&self) -> Option<Self>
	{
		let next = self.reference().next.tqe_next;
		if next.is_null()
		{
			None
		}
		else
		{
			Some(DpdkDriver(unsafe { NonNull::new_unchecked(next) }, PhantomData))
		}
	}
	
	/// Name.
	#[inline(always)]
	pub fn name(&self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(self.reference().name) }
	}
	
	/// Alias.
	#[inline(always)]
	pub fn alias(&self) -> Option<&'a CStr>
	{
		let alias = self.reference().alias;
		if alias.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(alias) })
		}
	}
	
	#[inline(always)]
	fn reference(&self) -> &rte_driver
	{
		unsafe { & * self.0.as_ptr() }
	}
}
