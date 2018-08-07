// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a generic DPDK device's arguments.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkDeviceArguments<'a>(NonNull<rte_devargs>, PhantomData<&'a rte_devargs>);

impl<'a> DpdkDeviceArguments<'a>
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
			Some(DpdkDeviceArguments(unsafe { NonNull::new_unchecked(next) }, PhantomData))
		}
	}
	
	/// Either whitelisted or blacklisted.
	#[inline(always)]
	pub fn policy(&self) -> rte_dev_policy
	{
		self.reference().policy
	}
	
	/// Bus, eg PCI bus, that the device resides on.
	///
	/// Can be `None` if a virtual device.
	#[inline(always)]
	pub fn bus(&self) -> Option<DpdkBus>
	{
		let bus = self.reference().bus;
		if bus.is_null()
		{
			None
		}
		else
		{
			Some(DpdkBus(unsafe { NonNull::new_unchecked(bus) }))
		}
	}
	
	#[inline(always)]
	fn reference(&self) -> &'a rte_devargs
	{
		unsafe { & * self.0.as_ptr() }
	}
}
