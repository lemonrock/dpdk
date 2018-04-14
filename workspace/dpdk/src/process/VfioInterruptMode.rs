// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// VFIO interrupt mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VfioInterruptMode
{
	#[allow(missing_docs)]
	Legacy,
	
	#[allow(missing_docs)]
	Msi,
	
	#[allow(missing_docs)]
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
	
	/// As DPDK value.
	#[inline(always)]
	pub fn to_rte_intr_mode(self) -> rte_intr_mode
	{
		use self::VfioInterruptMode::*;
		use self::rte_intr_mode::*;
		
		match self
		{
			Legacy => RTE_INTR_MODE_LEGACY,
			Msi => RTE_INTR_MODE_MSI,
			MsiX => RTE_INTR_MODE_MSIX,
		}
	}
	
	#[inline(always)]
	pub(crate) fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::VfioInterruptMode::*;
		
		match self
		{
			Legacy => Self::legacy,
			Msi => Self::msi,
			MsiX => Self::msix,
		}
	}
}
