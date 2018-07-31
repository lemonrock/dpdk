// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Virtual Function IO ('vfio') interrupt mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum VirtualFunctionIoInterruptMode
{
	/// Interrupts are delivered to a physically signalled pin by the Advanced Programmable Interrupt Controller (APIC).
	///
	/// This is the original scheme used to support PCI.
	///
	/// The APIC usually only supports 24 interrupt request lines ('IRQs'), although many of these are hardcoded for specific purposes.
	Legacy,
	
	/// Message Signalled Interrupts.
	///
	/// The PCI bus (as of version 2.2) writes a very small message to a known location.
	///
	/// Up to 64 IRQs are supported.
	Msi,
	
	/// Message Signalled Interrupts, Extended.
	///
	/// Most modern PCIe equipment should be capable of this mode.
	///
	/// Up to 2048 IRQs are supported.
	MsiX,
}

impl VirtualFunctionIoInterruptMode
{
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
	
	/// An an initialisation argument.
	#[inline(always)]
	pub fn as_initialization_argument(self) -> ConstCStr
	{
		use self::VirtualFunctionIoInterruptMode::*;
		
		match self
		{
			Legacy => ConstCStr(b"legacy\0"),
			Msi => ConstCStr(b"msi\0"),
			MsiX => ConstCStr(b"msix\0"),
		}
	}
}
