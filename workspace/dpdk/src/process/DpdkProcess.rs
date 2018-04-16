// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Used to manage the lifetime of a DPDK process.
///
/// Once dropped, resources will be cleaned up such that restart is possible.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkProcess;

impl Drop for DpdkProcess
{
	#[inline(always)]
	fn drop(&mut self)
	{
		match unsafe { rte_eal_cleanup() }
		{
			0 => (),
			
			NegativeE::EFAULT => panic!("EFAULT from rte_eal_cleanup()"),
			
			unexpected if unexpected < 0 => panic!("unexpected error code '{}' from rte_eal_cleanup()", unexpected),
			
			illegal @ _ => panic!("illegal result code '{}' from rte_eal_cleanup()", illegal),
		}
	}
}

impl DpdkProcess
{
	/// Is the primary process alive?
	#[inline(always)]
	pub fn is_primary_dpdk_process_alive() -> bool
	{
		if let Some(primary_process_configuration_file_path) = primary_process_configuration_file_path
		{
			let c_string = primary_process_configuration_file_path.to_c_string();
			
			isTrue(unsafe { rte_eal_primary_proc_alive(c_string.as_ptr()) })
		}
		else
		{
			isTrue(unsafe { rte_eal_primary_proc_alive(null()) })
		}
	}
	
	/// Request `iopl` privilege for all 'RPL'.
	///
	/// This function should be called by poll-mode drivers (PMDs) which need access to PCI io ports.
	#[inline(always)]
	pub fn initialize_io_port_list() -> Result<(), ()>
	{
		match unsafe { rte_eal_iopl_init() }
		{
			0 => Ok(()),
			-1 => Err(()),
			illegal @ _ => panic!("Illegal value '{}' from rte_eal_iopl_init()", illegal),
		}
	}
	
	/// Was configured to create UIO device?
	#[inline(always)]
	pub fn configured_to_create_uio_device_on_file_system_in_slash_dev() -> bool
	{
		(unsafe { rte_eal_create_uio_dev() }) != 0
	}
	
	/// Process type of current process.
	#[inline(always)]
	pub fn process_type() -> Result<ProcessType, ()>
	{
		use self::rte_proc_type_t::*;
		use self::ProcessType::*;
		
		match unsafe { rte_eal_process_type() }
		{
			RTE_PROC_AUTO => Ok(Auto),
			RTE_PROC_PRIMARY => Ok(Primary),
			RTE_PROC_SECONDARY => Ok(Secondary),
			RTE_PROC_INVALID => Err(()),
		}
	}
	
	/// Configured virtual function io ('vfio') interrupt mode.
	#[inline(always)]
	pub fn configured_virtual_function_io_interrupt_mode() -> Option<VirtualFunctionIoInterruptMode>
	{
		VirtualFunctionIoInterruptMode::from_dpdk_value(unsafe { rte_eal_vfio_intr_mode() })
	}
}
