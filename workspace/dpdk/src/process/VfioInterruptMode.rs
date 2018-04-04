// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VfioInterruptMode
{
	Legacy,
	Msi,
	MsiX,
}

impl VfioInterruptMode
{
	const_cstr!
	{
		legacy = "legacy";
		msi = "msi";
		msix = "msix";
	}
	
	pub fn asFfi(self) -> rte_intr_mode
	{
		match self
		{
			VfioInterruptMode::Legacy => rte_intr_mode::RTE_INTR_MODE_LEGACY,
			VfioInterruptMode::Msi => rte_intr_mode::RTE_INTR_MODE_MSI,
			VfioInterruptMode::MsiX => rte_intr_mode::RTE_INTR_MODE_MSIX,
		}
	}
	
	pub fn asInitialisationArgument(self) -> ConstCStr
	{
		match self
		{
			VfioInterruptMode::Legacy => Self::legacy,
			VfioInterruptMode::Msi => Self::msi,
			VfioInterruptMode::MsiX => Self::msix,
		}
	}
}
