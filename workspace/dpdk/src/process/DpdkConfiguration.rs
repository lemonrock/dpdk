// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// DPDK RTE init data.
#[derive(Debug)]
#[derive(Deserialize)]
#[serde(default)]
pub struct DpdkConfiguration
{
	/// Linux `AFPACKET` virtual network devices by index.
	pub af_packet_net_virtual_devices: BTreeMap<u5, AfPacketNetVirtualDevice>,
	
	/// Bonded virtual network devices by index.
	pub bonding_net_virtual_devices: BTreeMap<u5, BondingNetVirtualDevice>,
	
	/// Linux Kernel Native Interface (KNI) virtual network devices by index.
	///
	/// Internally a `ctrl` thread may be created for these (see `rte_ctrl_thread_create`).
	pub kernel_native_interface_net_virtual_devices: BTreeMap<u5, KniNetVirtualDevice>,
	
	/// Packet capture (`pcap`) virtual network devices by index.
	pub packet_capture_net_virtual_devices: BTreeMap<u5, PacketCaptureNetVirtualDevice>,
	
	/// `VirtIO` virtual network devices by index.
	pub virt_io_net_virtual_devices: BTreeMap<u5, VirtIoNetVirtualDevice>,
	
	/// `vhost` host virtual network devices by index.
	///
	/// Internally several `ctrl` threads may be created for these (see `rte_ctrl_thread_create`).
	pub virtual_host_net_virtual_devices: BTreeMap<u5, VirtualHostNetVirtualDevice>,
	
	/// Can be changed from default (`None`).
	pub memory_channels: Option<MemoryChannels>,
	
	/// Can be changed from default (`None`).
	pub memory_ranks: Option<MemoryRanks>,
	
	// TODO: Revise this code as memory handling has changed.
	/// Can be changed from default (`None`).
	memory_limits: Option<MemoryLimits>,
	
	/// Can be changed from default (`None`).
	pub process_type: Option<ProcessType>,
	
	/// Can be changed from default (`true`).
	pub use_hpet_timer: bool,
	
	/// Can be changed from default (`false`).
	pub use_shared_configuration_memory_map: bool,
	
	/// Can be changed from default (`false`).
	pub use_vmware_tsc_map_instead_of_native_rdtsc: bool,

	#[cfg(any(target_os = "android", target_os = "linux"))]
	/// Can be changed from default (`None`).
	pub base_virtual_address: Option<usize>,
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	/// Can be changed from default (`None`).
	pub virtual_function_io_interrupt_mode: Option<VirtualFunctionIoInterruptMode>,
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	/// Can be changed from default (`true`).
	pub create_uio_device_on_file_system_in_slash_dev: bool,
}

impl Default for DpdkConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			af_packet_net_virtual_devices: Default::default(),
			bonding_net_virtual_devices: Default::default(),
			packet_capture_net_virtual_devices: Default::default(),
			virt_io_net_virtual_devices: Default::default(),
			virtual_host_net_virtual_devices: Default::default(),

			memory_limits: None,
			memory_channels: None,
			memory_ranks: None,

			process_type: None,
			use_hpet_timer: true,
			use_shared_configuration_memory_map: false,
			use_vmware_tsc_map_instead_of_native_rdtsc: false,
			
			#[cfg(any(target_os = "android", target_os = "linux"))] base_virtual_address: None,
			#[cfg(any(target_os = "android", target_os = "linux"))] virtual_function_io_interrupt_mode: None,
			#[cfg(any(target_os = "android", target_os = "linux"))] create_uio_device_on_file_system_in_slash_dev: true,
		}
	}
}

impl DpdkConfiguration
{
	/// Are there any Kernel Native Interface (KNI) virtual devices?
	#[inline(always)]
	pub fn has_kernel_native_interface_virtual_devices(&self) -> bool
	{
		self.kernel_native_interface_net_virtual_devices.len() != 0
	}
	
	/// Enable the high precision event timer after initialization of DPDK if configured.
	///
	/// Internally creates a DPDK `ctrl` thread called `hpet-msb-inc` (see `rte_ctrl_thread_create`).
	pub fn enable_high_precision_event_timer_after_dpdk_initialized_if_configured(&self)
	{
		if enable_high_precision_event_timer
		{
			assert_eq!(unsafe { rte_eal_hpet_init(1) }, 0, "Could not initialize high precision event timer (HPET)");
		}
	}
	
	/// Initialise DPDK.
	///
	/// When the returned result is dropped, resources are released.
	#[inline(always)]
	pub fn initialize_dpdk<V>(&self, pci_devices: &HashMap<PciDevice, V>, numa_sockets: &NumaSockets, huge_page_file_path_information: HugePageFilePathInformation) -> Result<(), &'static str>
	{
		let huge_page_details = huge_page_file_path_information.huge_page_file_system_mount_path_and_so_on();
		let use_huge_pages = huge_page_details.is_some();

		let mut arguments: Vec<*const c_char> = Vec::initialise();

		Self::initialize_dpdk_pci_device_settings(&mut arguments, pci_devices);
		self.initialize_dpdk_virtual_device_settings(&mut arguments);
		self.initialize_dpdk_process_type_settings(&mut arguments);
		Self::initialize_dpdk_logical_core_settings(&mut arguments, numa_sockets);
		self.initialize_dpdk_memory_limits_settings(&mut arguments, use_huge_pages, numa_sockets);
		self.initialize_dpdk_memory_rank_and_memory_channel_settings(&mut arguments);
		self.initialize_dpdk_huge_page_settings(&mut arguments, huge_page_details);
		self.initialize_dpdk_optional_settings(&mut arguments);
		self.initialize_dpdk_log_settings(&mut arguments);
		self.initialize_dpdk_os_specific_settings(&mut arguments);

		Self::call_rte_eal_init(arguments)
	}
	
	#[inline(always)]
	pub(crate) fn initialize_dpdk_pci_device_settings<V>(arguments: &mut Vec<*const c_char>, pci_devices: &HashMap<PciDevice, V>)
	{
		for pci_device in pci_devices.iter_keys()
		{
			const_cstr!
			{
				__pci_whitelist = "--pci-whitelist";    // aka -w
			}
			let value = pci_device.to_address_c_string();
			arguments.keyCStrValue(__pci_whitelist.as_ptr(), &value);
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_virtual_device_settings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		#[inline(always)]
		fn add_virtual_devices<V: VirtualDevice>(arguments: &mut Vec<*const c_char>, map: &BTreeMap<u8, V>)
		{
			const_cstr!
			{
				__vdev = "--vdev";
			}
			
			for (index, virtual_device) in map.iter()
			{
				let argument = virtual_device.as_initialisation_argument(*index);
				arguments.keyCStrValue(__vdev, &argument);
			}
		}
		
		add_virtual_devices(arguments, &self.af_packet_net_virtual_devices);
		add_virtual_devices(arguments, &self.bonding_net_virtual_devices);
		add_virtual_devices(arguments, &self.kernel_native_interface_net_virtual_devices);
		add_virtual_devices(arguments, &self.packet_capture_net_virtual_devices);
		add_virtual_devices(arguments, &self.virt_io_net_virtual_devices);
		add_virtual_devices(arguments, &self.virtual_host_net_virtual_devices);
	}
	
	#[inline(always)]
	fn initialize_dpdk_process_type_settings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__proc_type = "--proc-type";            // For multi-process set ups
		}

		if let Some(process_type) = self.process_type
		{
			arguments.keyConstantValue(__proc_type, process_type.as_initialisation_argument());
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_logical_core_settings(mut arguments: &mut Vec<*const c_char>, numa_sockets: &NumaSockets)
	{
		const_cstr!
		{
			_c = "-c";                              // COREMASK
			__master_lcore = "--master-lcore";      // u32
			// _l = "-l";                           // CORELIST
			// __lcores = "--lcores";               // COREMAP, mapping of logical cores to physical CPUs, (see http://dpdk.org/doc/guides/testpmd_app_ug/run_app.html)
		}

		let value = numa_sockets.logical_cores_active.as_hexadecimal_core_mask_c_string();
		arguments.keyCStrValue(_c, &value);

		let value = CString::new(format!("{}", numa_sockets.master_logical_core.as_u32())).unwrap();
		arguments.keyCStrValue(__master_lcore, &value);
	}
	
	#[inline(always)]
	fn initialize_dpdk_memory_rank_and_memory_channel_settings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			_n = "-n";                              // 31-bit, != 0, Number of memory channels to use
			_r = "-r";                              // 5-bit, != 0, <= 16, Number of memory ranks to use
		}

		if let Some(override_number_of_memory_channels) = self.memory_channels
		{
			let value = CString::new(format!("{}", override_number_of_memory_channels as u32)).unwrap();
			arguments.keyCStrValue(_n, &value);
		}

		if let Some(override_number_of_memory_ranks) = self.memory_ranks
		{
			let value = CString::new(format!("{}", override_number_of_memory_ranks as u8)).unwrap();
			arguments.keyCStrValue(_r, &value);
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_memory_limits_settings(&self, mut arguments: &mut Vec<*const c_char>, use_huge_pages: bool, numa_sockets: &NumaSockets)
	{
		// --socket-mem and -m [total, 512Mb] conflict.
		// --socket-mem requires huge pages
		
		// --socket-mem=1024,0,1024
		// Allocate 1Gb socket 0
		// Allocate 0Gb socket 1
		// Allocate 1Gb socket 2
		// No allocation on socket 3
		
		// We need an allocation strategy for numa nodes
		
		/*
			For each Numa node,
				- take the HugePageAllocationStrategy
				- call calculate_nearest_allocation_size()
				- convert to whatever it is that socket-mem takes
		
		
		*/
		
		
		
		#[inline(always)]
		fn initialize_dpdk_total_memory_limits(mut arguments: &mut Vec<*const c_char>, size_of_total_memory_in_megabytes: u31)
		{
			const_cstr!
			{
				_m = "-m";                              // u32 Mb of RAM (as Mb, not bytes); Maximum of 512 Gb; maximum DPDK supports
			}

			let value = CString::new(format!("{}", size_of_total_memory_in_megabytes)).unwrap();
			arguments.keyCStrValue(_m, &value);
		}
		
		#[inline(always)]
		fn initialize_dpdk_per_numa_node_memory_limits(mut arguments: &mut Vec<*const c_char>, per_numa_node_string: CString)
		{
			const_cstr!
			{
				__socket_mem = "--socket-mem";          // Conflicts with -m and use_huge_pages=false
			}

			arguments.keyCStrValue(__socket_mem, &per_numa_node_string);
		}

		if let Some(memory_limits) = self.memory_limits
		{
			if cfg!(target_os = "freebsd")
			{
				if let Some(total_memory) = memory_limits.total_memory_in_megabytes(numa_sockets)
				{
					initialize_dpdk_total_memory_limits(arguments, total_memory);
				}
			}
			else
			{
				if !use_huge_pages
				{
					panic!("Can not have per NUMA socket memory (memory_limits) and then have use_huge_pages as false");
				}

				let (per_numa_node, total_memory_option) = memory_limits.as_initialisation_string_if_is_a_numa_machine(use_huge_pages, numa_sockets);
				if let Some(per_numa_node) = per_numa_node
				{
					initialize_dpdk_per_numa_node_memory_limits(arguments, per_numa_node)
				}
				else
				{
					if let Some(total_memory) = total_memory_option
					{
						initialize_dpdk_total_memory_limits(arguments, total_memory);
					}
				}
			}
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_huge_page_settings(&self, mut arguments: &mut Vec<*const c_char>, huge_page_file_system_mount: Option<(&Path, Option<&OsStr>)>)
	{
		const_cstr!
		{
			__huge_dir = "--huge-dir";
			__huge_unlink = "--huge-unlink";
			__no_huge = "--no-huge";
			__file_prefix = "--file-prefix";
		}

		if let Some((huge_page_file_system_mount_path, huge_page_file_name_prefix)) = huge_page_file_system_mount
		{
			let c_string = huge_page_file_system_mount_path.to_c_string();
			arguments.keyCStrValue(__huge_dir, &c_string);

			arguments.optionalArgument(__no_huge, false);

			if self.process_type.is_none()
			{
				arguments.optionalArgument(__huge_unlink, true);
			}

			if let Some(huge_page_file_name_prefix) = huge_page_file_name_prefix
			{
				let c_string = huge_page_file_name_prefix.os_str_to_c_string();
				arguments.keyCStrValue(__file_prefix, &c_string);
			}
		}
		else
		{
			arguments.optionalArgument(__no_huge, true);
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_optional_settings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__no_hpet = "--no-hpet";                // Debug use only
			__no_pci = "--no-pci";                  // Debug use only
			__no_shconf = "--no-shconf";            // Debug use only
			__vmware_tsc_map = "--vmware-tsc-map";  //
		}

		arguments.optionalArgument(__no_hpet, !self.use_hpet_timer);

		arguments.optionalArgument(__no_pci, self.pci_net_devices.is_empty());

		arguments.optionalArgument(__no_shconf, !self.use_shared_configuration_memory_map);

		arguments.optionalArgument(__vmware_tsc_map, self.use_vmware_tsc_map_instead_of_native_rdtsc);
	}
	
	#[inline(always)]
	fn initialize_dpdk_log_settings(&self, arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__syslog = "--syslog";                  // A facility. Not configurable, as we really don't know what DPDK will produce, so we send to 'auth'
			__log_level = "--log-level";            // A log level. Not configurable; we choose either a debug one or a production one
		}

		arguments.keyConstantValue(__syslog, const_cstr!("auth"));

		let log_level = if cfg!(debug_assertions)
		{
			const_cstr!("8") // RTE_LOG_DEBUG
		}
		else
		{
			const_cstr!("5") // RTE_LOG_WARNING
		};
		arguments.keyConstantValue(__log_level, lovel);
	}

	#[inline(always)]
	fn initialize_dpdk_os_specific_settings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			const_cstr!
			{
				__base_virtaddr = "--base-virtaddr";
				__vfio_intr = "--vfio-intr";
				__create_uio_dev = "--create-uio-dev";
			}
	
			arguments.optionalArgument(__create_uio_dev, self.create_uio_device_on_file_system_in_slash_dev);
	
			if let Some(base_virtual_address) = self.base_virtual_address
			{
				let value = &CString::new(format!("{0:x}", base_virtual_address)).unwrap();
				arguments.keyCStrValue(__base_virtaddr, value);
			}
	
			if let Some(virtual_function_io_interrupt_mode) = self.virtual_function_io_interrupt_mode
			{
				arguments.keyConstantValue(__vfio_intr, virtual_function_io_interrupt_mode.as_initialisation_argument());
			}
		}
		
		#[cfg(target_os = "freebsd")]
		{
		}
	}
	
	#[inline(always)]
	fn call_rte_eal_init(mut arguments: Vec<*const c_char>) -> Result<(), &'static str>
	{
		let count = arguments.len();
		arguments.push(null_mut());

		let argc = count as c_int;
		let argv = arguments.as_mut_ptr() as *mut *mut c_char;

		match unsafe { rte_eal_init(argc, argv) }
		{
			number_of_parsed_arguments if number_of_parsed_arguments >= 0 =>
			{
				if number_of_parsed_arguments != count as c_int
				{
					panic!("Parsed only number_of_parsed_arguments '{}' but provided count '{}' arguments", number_of_parsed_arguments, count);
				}
				
				Ok(())
			}
			
			-1 => match unsafe { rte_errno() }
			{
				E::EACCES => Err("permissions issue"),
				E::EAGAIN => Err("either a bus or system resource was not available; try again"),
				E::EALREADY => Err("already initialized"),
				E::EFAULT => Err("the tailq configuration name was not found in the memory configuration"),
				E::EINVAL => Err("invalid parameters in argc or argv"),
				E::ENOMEM => Err("failure likely caused by an out-of-memory condition"),
				E::ENODEV => Err("memory setup issues"),
				E::ENOTSUP => Err("the EAL cannot initialize on this system (not supported)"),
				E::EPROTO => Err("the PCI bus is not present or unreadable"),
				E::ENOEXEC => Err("service core failed to launch successfully"),
			}
			
			illegal @ _ => panic!("Could not initialise DPDK Environment Abstraction Layer, received illegal result '{}'", illegal),
		}
	}
}
