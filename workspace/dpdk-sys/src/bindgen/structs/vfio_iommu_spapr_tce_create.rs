// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct vfio_iommu_spapr_tce_create
{
	pub argsz: __u32,
	pub flags: __u32,
	pub page_shift: __u32,
	pub window_size: __u64,
	pub levels: __u32,
	pub start_addr: __u64,
}

impl Default for vfio_iommu_spapr_tce_create
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for vfio_iommu_spapr_tce_create
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "vfio_iommu_spapr_tce_create {{  }}")
	}
}
