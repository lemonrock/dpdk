// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Virtual Function IO ('vfio') interrupt mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualFunctionIoInterruptMode
{
	#[allow(missing_docs)]
	Legacy,
	
	/// Message Signalled Interrupts.
	Msi,
	
	/// Message Signalled Interrupts, Extended.
	MsiX,
}

impl VirtualFunctionIoInterruptMode
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
		use self::VirtualFunctionIoInterruptMode::*;
		use self::rte_intr_mode::*;
		
		match self
		{
			Legacy => RTE_INTR_MODE_LEGACY,
			Msi => RTE_INTR_MODE_MSI,
			MsiX => RTE_INTR_MODE_MSIX,
		}
	}
	
	/// From DPDK value.
	#[inline(always)]
	pub fn from_rte_intr_mode(dpdk_value: rte_intr_mode) -> Option<Self>
	{
		use self::VirtualFunctionIoInterruptMode::*;
		use self::rte_intr_mode::*;
		
		match dpdk_value
		{
			RTE_INTR_MODE_NONE => None,
			RTE_INTR_MODE_LEGACY => Some(Legacy),
			RTE_INTR_MODE_MSI => Some(Msi),
			RTE_INTR_MODE_MSIX => Some(MsiX),
		}
	}
	
	#[inline(always)]
	pub(crate) fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::VirtualFunctionIoInterruptMode::*;
		
		match self
		{
			Legacy => Self::legacy,
			Msi => Self::msi,
			MsiX => Self::msix,
		}
	}
}
