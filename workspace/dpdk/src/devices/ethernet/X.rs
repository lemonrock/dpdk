// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// DPDK log levels.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum DpdkLogLevel
{
	/// Emergency.
	Emergency = RTE_LOG_EMERG,
	
	/// Alert.
	Alert = RTE_LOG_ALERT,
	
	/// Critical.
	Critical = RTE_LOG_CRIT,
	
	/// Error.
	Error = RTE_LOG_ERR,
	
	/// Warning.
	Warning = RTE_LOG_WARNING,
	
	/// Notice.
	Notice = RTE_LOG_NOTICE,
	
	/// Information.
	Information = RTE_LOG_INFO,
	
	/// Debug.
	Debug = RTE_LOG_DEBUG,
}

impl Default for DpdkLogLevel
{
	#[inline(always)]
	fn default() -> Self
	{
		DpdkLogLevel::Emergency
	}
}

fn check_cpu_supported()
{
	let value = unsafe { rte_cpu_is_supported() };
	if value == 0
	{
		panic!("Unsupported CPU type")
	}
}

fn set_run_once()
{
	static Initialized: AtomicBool = AtomicBool::new(false);
	
	if !Initialized.compare_and_swap(false, true, SeqCst)
	{
		panic!("Already initialized")
	}
}

fn reset_internal_configuration()
{
	// TODO: Access to private static `internal_config`
	// TODO: Access to private `eal_reset_internal_config`
	unsafe { eal_reset_internal_config(&internal_config) };
}

fn set_global_log_level(log_level: DpdkLogLevel, log_level_pattern: Option<&CStr>)
{
	if let Some(pattern) = log_level_pattern
	{
		assert_eq!(unsafe { rte_log_set_level_regexp(pattern.as_ptr(), log_level as u32) }, 0, "rte_log_set_level_regexp failed");
	}
	else
	{
		unsafe { rte_log_set_global_level(log_level as u32) }
	}
}

// If we set up logging and initialize CPUs
fn initialise_logical_core_configuration()
{
	// TODO: Access to private `rte_eal_cpu_init`
	
	let value = unsafe { rte_eal_cpu_init() };
	if value < 0
	{
		panic!("Could not detect logical cores")
	}
}

fn parse_arguments()
{
	fctret = eal_parse_args(argc, argv);
	if (fctret < 0) {
		rte_eal_init_alert("Invalid 'command line' arguments.");
		rte_errno = EINVAL;
		rte_atomic32_clear(&run_once);
		return -1;
	}
}

fn initialize_plugins()
{
	// TODO: Access to private `eal_plugins_init`
	
	let value = unsafe { eal_plugins_init() };
	if value < 0
	{
		panic!("Could not initialize plugins")
	}
}

fn parse_device_options()
{
	// TODO: Access to private `eal_option_device_parse`
	
	assert_eq!(unsafe { eal_option_device_parse() }, 0, "Could not parse device options");
}

fn scan_all_buses_for_devices()
{
	// TODO: Access to private `rte_bus_scan`
	
	assert_eq!(unsafe { rte_bus_scan() }, 0, "Could not scan buses for devices");
}

fn autodetect_io_virtual_address_mapping_mode()
{
	DpdkProcess::global_configuration().iova_mode = DpdkBus::get_common_iommu_class();
}

fn workaround_for_kni_which_requires_physical_addresses_to_work()
{
	// TODO: Access to private `rte_eal_check_module`
	
	use self::rte_iova_mode::*;
	
	const_cstr!
	{
		rte_kni = "rte_kni";
	}
	
	if DpdkProcess::global_configuration().iova_mode == RTE_IOVA_VA && unsafe { rte_eal_check_module(rte_kni.as_ptr()) } == 1
	{
		DpdkProcess::global_configuration().iova_mode = RTE_IOVA_PA
	}
}

fn initialize_huge_pages()
{
	// TODO: Access to private static `internal_config`
	// TODO: Access to private `eal_hugepage_info_init`
	
	if internal_config.no_hugetlbfs == 0 && internal_config.process_type != rte_proc_type_t::RTE_PROC_SECONDARY
	{
		let value = unsafe { eal_hugepage_info_init() };
		if value < 0
		{
			panic!("Could not initialize huge pages");
		}
	}
	
	if internal_config.memory == 0 && internal_config.force_sockets == 0
	{
		if internal_config.no_hugetlbfs != 0
		{
			const MEMSIZE_IF_NO_HUGE_PAGE: u64 = 64 * 1024 * 1024;
			internal_config.memory = MEMSIZE_IF_NO_HUGE_PAGE
		}
	}
}


// ? rte_ctrl_thread_create() ?
// commit 5f19dee604ed5760d819743d1d364493ead2aae6  ?rte_eth_dev_data?
// commit b9bd0f09fa15b9dd6c70c469b45d215f494cab97  - link status
// rte_dev_event_monitor_start - hotplug events
// commit 6b42f75632f04d636d92e8946cdd423f82597206  - non legacy memory mode eal / memory hotplug
// 582bed1e1d1ddfd51bc7d61b95b0b8b55e47c6f9 - support mapping huge pages at runtime    --legacy-mem for contiguous pages
// Dead: rte_eal_dump_physmem_layout()
// commit f0e352ddb016ca346c66b5cd97f15896fdf829ee new pipeline library functionality
// commit 952b20777255d203dafd813b1adfb4b9d6b449d1  API to query valid socket ids
// Logging changes: RTE_LOG_LEVEL has been replaced by RTE_DP_LOG_LEVEL

fn rte_eal_init(log_level: DpdkLogLevel, log_level_pattern: Option<&CStr>)
{

}
