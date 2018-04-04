// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub flags DriverFlags: u32
	{
		const NeedsBaseAddressRegisterMapping = ::dpdk_sys::RTE_PCI_DRV_NEED_MAPPING,
		const SupportsLinkStatusInterrupt = ::dpdk_sys::RTE_PCI_DRV_INTR_LSC,
	}
}

impl Default for DriverFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
