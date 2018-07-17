// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Power Management.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PowerManagement
{
	/// Pysical.
	Physical,
	
	/// Kernel Virtual Machine (KVM).
	KernelVirtualMachine,
}

impl PowerManagement
{
	/// Current.
	#[inline(always)]
	pub fn current() -> Option<Self>
	{
		use self::power_management_env::*;
		use self::PowerManagement::*;
		
		match unsafe { rte_power_get_env() }
		{
			PM_ENV_NOT_SET => None,
			PM_ENV_ACPI_CPUFREQ => Some(Physical),
			PM_ENV_KVM_VM => Some(KernelVirtualMachine),
		}
	}

	/// Not thread safe
	/// Not really needed; auto-detection occurs whenever a LogicalCore has power management started
	#[inline(always)]
	pub fn start(&self) -> Result<(), i32>
	{
		use self::PowerManagement::*;
		use self::power_management_env::*;
		
		let environment = match *self
		{
			Physical => PM_ENV_ACPI_CPUFREQ,
			KernelVirtualMachine => PM_ENV_KVM_VM,
		};

		match unsafe { rte_power_set_env(environment) }
		{
			0 => Ok(()),

			x if x < 0 => Err(x),

			illegal @ _ => panic!("rte_power_set_env() returned an invalid positive return code of '{}'", illegal),
		}
	}

	/// Should only be called when all threads have finished!
	#[inline(always)]
	pub fn stop()
	{
		unsafe { rte_power_unset_env() }
	}
}
