// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A virtual device as defined by DPDK.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkVirtualDevice(NonNull<rte_vdev_device>);

impl From<NonNull<rte_device>> for DpdkVirtualDevice
{
	#[inline(always)]
	fn from(value: NonNull<rte_device>) -> Self
	{
		DpdkVirtualDevice(unsafe { NonNull::new_unchecked(rust_RTE_DEV_TO_VDEV(value.as_ptr())) })
	}
}

impl DpdkVirtualDevice
{
	/// Device arguments.
	#[inline(always)]
	pub fn device_arguments(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rust_rte_vdev_device_args(self.0.as_ptr() as *const _)) }
	}
	
	/// Device name.
	#[inline(always)]
	pub fn device_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(rust_rte_vdev_device_name(self.0.as_ptr() as *const _)) }
	}
}
