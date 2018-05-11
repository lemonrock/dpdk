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
	
	/// Defaults to `false`.
	///
	/// Uses calculated number of huge pages to constrain memory.
	pub limit_memory: bool,
	
	/// Where and how to mount huge pages.
	pub huge_page_mount_settings: HugePageMountSettings,
	
	/// How many huge pages to allocate?
	pub huge_page_allocation_strategy: HugePageAllocationStrategy,
	
	/// What prefix to use for the huge pages.
	/// Defaults to program name.
	///
	/// Must not be empty.
	pub huge_page_file_name_prefix: String,
	
	/// Can be changed from default (`None`).
	pub process_type: Option<ProcessType>,
	
	/// Use High Precision Event Timer (HPET).
	///
	/// Can be changed from default (`true`).
	pub use_high_precision_event_timer: bool,
	
	/// Can be changed from default (`false`).
	pub use_shared_configuration_memory_map: bool,
	
	/// Can be changed from default (`false`).
	pub use_vmware_tsc_map_instead_of_native_rdtsc: bool,
	
	/// Defaults to `auth`.
	pub syslog_facility: DpdkSyslogFacility,
	
	/// Defaults to `debug` for debug builds and `warning` for production builds.
	///
	/// DPDK also supports specifying either a regex or a pattern; this is not supported by `DpdkConfiguration` at this time.
	pub syslog_priority: DpdkSyslogPriority,
	
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

			memory_channels: None,
			memory_ranks: None,
			limit_memory: false,
			huge_page_mount_settings: HugePageMountSettings::default(),
			huge_page_allocation_strategy: HugePageAllocationStrategy::default(),
			huge_page_file_name_prefix: get_program_name(),

			process_type: None,
			use_high_precision_event_timer: true,
			use_shared_configuration_memory_map: false,
			use_vmware_tsc_map_instead_of_native_rdtsc: false,
			
			syslog_facility: Default::default(),
			syslog_priority: Default::default(),
			
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
		if self.use_high_precision_event_timer
		{
			assert_eq!(unsafe { rte_eal_hpet_init(1) }, 0, "Could not initialize high precision event timer (HPET)");
		}
	}
	
	/// Initialise DPDK.
	///
	/// Panics if this fails.
	#[inline(always)]
	pub fn initialize_dpdk<V>(&self, pci_devices: &HashMap<PciDevice, V>, hugetlbfs_mount_path: &Path, memory_limits: MachineOrNumaNodes<MegaBytes>)
	{
		let arguments = Arguments::new();

		Self::initialize_dpdk_pci_device_settings(&arguments, pci_devices);
		self.initialize_dpdk_virtual_device_settings(&arguments);
		self.initialize_dpdk_process_type_settings(&arguments);
		Self::initialize_dpdk_logical_core_settings(&arguments);
		self.initialize_dpdk_memory_limits_settings(&arguments, memory_limits);
		self.initialize_dpdk_huge_page_settings(&arguments, hugetlbfs_mount_path);
		self.initialize_dpdk_memory_rank_and_memory_channel_settings(&arguments);
		self.initialize_dpdk_optional_settings(&arguments, pci_devices);
		self.initialize_dpdk_log_settings(&arguments);
		self.initialize_dpdk_os_specific_settings(&arguments);

		Self::call_rte_eal_init(arguments)
	}
	
	#[inline(always)]
	pub(crate) fn initialize_dpdk_pci_device_settings<V>(argument: &mut Arguments, pci_devices: &HashMap<PciDevice, V>)
	{
		for pci_device in pci_devices.iter_keys()
		{
			arguments.variable_argument(Arguments::__pci_whitelist, &pci_device.to_address_string());
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_virtual_device_settings(&self, arguments: &mut Arguments)
	{
		#[inline(always)]
		fn add_virtual_devices<V: VirtualDevice>(argument: &mut Arguments, map: &BTreeMap<u8, V>)
		{
			for (index, virtual_device) in map.iter()
			{
				arguments.variable_argument(Arguments::__vdev, &virtual_device.as_initialisation_argument(*index));
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
	fn initialize_dpdk_process_type_settings(&self, arguments: &mut Arguments)
	{
		if let Some(process_type) = self.process_type
		{
			arguments.constant_argument(Arguments::__proc_type, process_type.as_initialisation_argument());
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_logical_core_settings(arguments: &mut Arguments, logical_core_list: MachineOrNumaNodes<X>)
	{
		use self::MachineOrNumaNodes::*;
		
		let (core_list, logical_core_that_overlaps_with_linux) = match logical_core_list
		{
			Machine(xxx) =>
			{
				// Sadly, the logical core mapping (--lcores) screws up the NUMA node information that DPDK uses.
				
				// This stops a scale-down model from working (ie treating cores as pthreads).
				
				// Consider parsing isolcpus, find the cpus that AREN'T isolated, then use one of those for the master logical core (and potentially any service cores).
			}
			
			NumaNodes(ref map) =>
			{
			
			}
		};
		
		arguments.variable_argument(Arguments::_l, &core_list);

		arguments.variable_argument(Arguments::__master_lcore, &format!("{}", logical_core_that_overlaps_with_linux));
	}
	
	#[inline(always)]
	fn initialize_dpdk_memory_limits_settings(&self, arguments: &mut Arguments, memory_limits: MachineOrNumaNodes<MegaBytes>)
	{
		if self.limit_memory
		{
			use self::MachineOrNumaNodes::*;
			
			match memory_limits
			{
				Machine(mega_bytes) => arguments.variable_argument(Arguments::_m, &mega_bytes.to_string_capped_at_dpdk_maximum()),
				
				NumaNodes(ref map) =>
				{
					let mut per_numa_node_string = String::with_capacity(128);
					let expected_next_numa_node = 0;
					let mut after_first = false;
					for (numa_node, mega_bytes) in map.iter()
					{
						let numa_node = *numa_node;
						let mega_bytes = *mega_bytes;
						for _unspecified in expected_next_numa_node .. (numa_node.into() - expected_next_numa_node)
						{
							if after_first
							{
								per_numa_node_string.push(',');
							}
							else
							{
								after_first = true;
							}
							per_numa_node_string.push('0');
						}
						
						if after_first
						{
							per_numa_node_string.push(',');
						}
						else
						{
							after_first = true;
						}
						per_numa_node_string.push_str(&mega_bytes.to_string_capped_at_dpdk_maximum());
					}
					
					arguments.variable_argument(Arguments::__socket_mem, &per_numa_node_string);
				}
			}
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_huge_page_settings(&self, arguments: &mut Arguments, hugetlbfs_mount_path: &Path)
	{
		arguments.variable_argument(Arguments::__huge_dir, hugetlbfs_mount_path.to_str().unwrap());
		
		arguments.option_argument(Arguments::__no_huge, false);
		
		if self.process_type.is_none()
		{
			arguments.option_argument(Arguments::__huge_unlink, true);
		}
		
		assert_ne!(self.huge_page_file_name_prefix.len(), 0, "huge_page_file_name_prefix must not be empty");
		
		arguments.variable_argument(Arguments::__file_prefix, &self.huge_page_file_name_prefix);
	}
	
	#[inline(always)]
	fn initialize_dpdk_memory_rank_and_memory_channel_settings(&self, arguments: &mut Arguments)
	{
		if let Some(override_number_of_memory_channels) = self.memory_channels
		{
			arguments.constant_argument(Arguments::_n, override_number_of_memory_channels.as_initialisation_argument());
		}
		
		if let Some(override_number_of_memory_ranks) = self.memory_ranks
		{
			arguments.constant_argument(Arguments::_r, override_number_of_memory_ranks.as_initialisation_argument());
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_optional_settings(&self, arguments: &mut Arguments, pci_devices: &HashMap<PciDevice, V>)
	{
		arguments.option_argument(Arguments::__no_hpet, !self.use_high_precision_event_timer);

		arguments.option_argument(Arguments::__no_pci, pci_devices.is_empty());

		arguments.option_argument(Arguments::__no_shconf, !self.use_shared_configuration_memory_map);

		arguments.option_argument(Arguments::__vmware_tsc_map, self.use_vmware_tsc_map_instead_of_native_rdtsc);
	}
	
	#[inline(always)]
	fn initialize_dpdk_log_settings(&self, argument: &mut Arguments)
	{
		arguments.constant_argument(Arguments::__syslog, self.syslog_facility.as_initialisation_argument());
		arguments.constant_argument(Arguments::__log_level, self.syslog_priority.as_initialisation_argument());
	}

	#[inline(always)]
	fn initialize_dpdk_os_specific_settings(&self, arguments: &mut Arguments)
	{
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			arguments.option_argument(Arguments::__create_uio_dev, self.create_uio_device_on_file_system_in_slash_dev);
	
			if let Some(base_virtual_address) = self.base_virtual_address
			{
				arguments.variable_argument(Arguments::__base_virtaddr, &format!("{0:x}", base_virtual_address));
			}
	
			if let Some(virtual_function_io_interrupt_mode) = self.virtual_function_io_interrupt_mode
			{
				arguments.constant_argument(Arguments::__vfio_intr, virtual_function_io_interrupt_mode.as_initialisation_argument());
			}
		}
		
		#[cfg(target_os = "freebsd")]
		{
		}
	}
	
	#[inline(always)]
	fn call_rte_eal_init(arguments: Arguments)
	{
		arguments.use_arguments(|argc, argv|
		{
			match unsafe { rte_eal_init(argc, argv) }
			{
				number_of_parsed_arguments if number_of_parsed_arguments >= 0 =>
				{
					assert_eq!(number_of_parsed_arguments, argc, "Did not return correct number of parsed arguments");
				},
				
				-1 => match unsafe { rte_errno() }
				{
					E::EACCES => panic!("Could not initialise DPDK Environment Abstraction Layer: permissions issue"),
					E::EAGAIN => panic!("Could not initialise DPDK Environment Abstraction Layer: either a bus or system resource was not available; try again"),
					E::EALREADY => panic!("Could not initialise DPDK Environment Abstraction Layer: already initialized"),
					E::EFAULT => panic!("Could not initialise DPDK Environment Abstraction Layer: the tailq configuration name was not found in the memory configuration"),
					E::EINVAL => panic!("Could not initialise DPDK Environment Abstraction Layer: invalid parameters in argc or argv"),
					E::ENOMEM => panic!("Could not initialise DPDK Environment Abstraction Layer: failure likely caused by an out-of-memory condition"),
					E::ENODEV => panic!("Could not initialise DPDK Environment Abstraction Layer: memory setup issues"),
					E::ENOTSUP => panic!("Could not initialise DPDK Environment Abstraction Layer: the EAL cannot initialize on this system (not supported)"),
					E::EPROTO => panic!("Could not initialise DPDK Environment Abstraction Layer: the PCI bus is not present or unreadable"),
					E::ENOEXEC => panic!("Could not initialise DPDK Environment Abstraction Layer: service core failed to launch successfully"),
				},
				
				illegal @ _ => panic!("Could not initialise DPDK Environment Abstraction Layer: received illegal result '{}'", illegal),
			}
		})
	}
}
