// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Dpdk information available after initialization.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkProcess;

impl DpdkProcess
{
	/// Global configuration.
	///
	/// Only valid after `rte_eal_init()` called.
	#[inline(always)]
	pub(crate) fn global_configuration() -> &'static mut rte_config
	{
		unsafe { &mut * rte_eal_get_configuration() }
	}
	
	/// Was configured to create UIO device?
	///
	/// Only valid after `DpdkConfiguration.initialize_dpdk()` called.
	#[inline(always)]
	pub fn configured_to_create_uio_device_on_file_system_in_slash_dev() -> bool
	{
		(unsafe { rte_eal_create_uio_dev() }) != 0
	}
	
	/// Configured virtual function io ('vfio') interrupt mode.
	///
	/// Only valid after `DpdkConfiguration.initialize_dpdk()` called.
	#[inline(always)]
	pub fn configured_virtual_function_io_interrupt_mode() -> Option<VirtualFunctionIoInterruptMode>
	{
		VirtualFunctionIoInterruptMode::from_rte_intr_mode(unsafe { rte_eal_vfio_intr_mode() })
	}
}
