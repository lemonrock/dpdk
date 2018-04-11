// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const RTE_INTR_EVENT_ADD: u32 = 1;
pub const RTE_INTR_EVENT_DEL: u32 = 2;
pub const RTE_INTR_MODE_LEGACY_NAME: &'static [u8; 7usize] = b"legacy\0";
pub const RTE_INTR_MODE_MSIX_NAME: &'static [u8; 5usize] = b"msix\0";
pub const RTE_INTR_MODE_MSI_NAME: &'static [u8; 4usize] = b"msi\0";
pub const RTE_INTR_MODE_NONE_NAME: &'static [u8; 5usize] = b"none\0";
pub const RTE_INTR_VEC_RXTX_OFFSET: u32 = 1;
pub const RTE_INTR_VEC_ZERO_OFFSET: u32 = 0;
