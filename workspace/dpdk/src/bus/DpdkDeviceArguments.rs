// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a generic DPDK device's arguments.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkDeviceArguments<'a>(NonNull<rte_devargs>, PhantomData<&'a rte_devargs>);

impl<'a> DpdkDeviceArguments<'a>
{
	/// Next.
	#[inline(always)]
	pub fn next(&self) -> Option<Self>
	{
		let next = self.deref().tqe_next;
		if next.is_null()
		{
			None
		}
		else
		{
			Some(DpdkDeviceArguments(unsafe { NonNull::new_unchecked(next) }, PhantomData))
		}
	}
	
	/// Either a PCI blacklisted device, a PCI whitelisted device or a virtual device (eg an ethernet bonding device).
	#[inline(always)]
	pub fn device_type(&self) -> rte_devtype
	{
		self.deref().type_
	}
	
	/// Either whitelisted or blacklisted.
	#[inline(always)]
	pub fn policy(&self) -> rte_dev_policy
	{
		self.deref().policy
	}
	
	/// Name (does not exceed 64 bytes).
	#[inline(always)]
	pub fn name(&self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(self.deref().name) }
	}
	
	/// Arguments given by user at configuration time; can be empty.
	#[inline(always)]
	pub fn arguments_given_by_user_at_configuration_time(&self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(self.deref().args) }
	}
	
	/// Bus, eg PCI bus, that the device resides on.
	///
	/// Can be `None` if a virtual device.
	#[inline(always)]
	pub fn bus(&self) -> Option<DpdkBus>
	{
		let bus = self.deref().bus;
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
	fn deref(&self) -> &'a rte_devargs
	{
		unsafe { & * self.0.as_ptr() }
	}
}
