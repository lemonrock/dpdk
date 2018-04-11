// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_intr_handle_type
{
	RTE_INTR_HANDLE_UNKNOWN = 0,
	RTE_INTR_HANDLE_UIO = 1,
	RTE_INTR_HANDLE_UIO_INTX = 2,
	RTE_INTR_HANDLE_VFIO_LEGACY = 3,
	RTE_INTR_HANDLE_VFIO_MSI = 4,
	RTE_INTR_HANDLE_VFIO_MSIX = 5,
	RTE_INTR_HANDLE_ALARM = 6,
	RTE_INTR_HANDLE_EXT = 7,
	RTE_INTR_HANDLE_VDEV = 8,
	RTE_INTR_HANDLE_MAX = 9,
}
