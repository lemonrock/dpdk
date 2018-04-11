// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_kernel_driver
{
	RTE_KDRV_UNKNOWN = 0,
	RTE_KDRV_IGB_UIO = 1,
	RTE_KDRV_VFIO = 2,
	RTE_KDRV_UIO_GENERIC = 3,
	RTE_KDRV_NIC_UIO = 4,
	RTE_KDRV_NONE = 5,
}
