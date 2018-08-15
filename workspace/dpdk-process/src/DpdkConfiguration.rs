// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// DPDK RTE init data.
#[derive(Debug)]
#[derive(Deserialize)]
#[serde(default)]
pub struct DpdkConfiguration
{
	/// Linux `AFPACKET` virtual network devices by index.
	pub af_packet_net_virtual_devices: BTreeMap<VirtualDeviceIndex, AfPacketNetVirtualDevice>,
	
	/// Bonded virtual network devices by index.
	pub bonding_net_virtual_devices: BTreeMap<VirtualDeviceIndex, BondingNetVirtualDevice>,
	
	/// Linux Kernel Native Interface (KNI) virtual network devices by index.
	///
	/// Internally a `ctrl` thread may be created for these (see `rte_ctrl_thread_create`).
	pub kernel_native_interface_net_virtual_devices: BTreeMap<VirtualDeviceIndex, KernelNativeInterfaceNetVirtualDevice>,
	
	/// Packet capture (`pcap`) virtual network devices by index.
	pub packet_capture_net_virtual_devices: BTreeMap<VirtualDeviceIndex, PacketCaptureNetVirtualDevice>,
	
	/// `VirtIO` virtual network devices by index.
	pub virt_io_net_virtual_devices: BTreeMap<VirtualDeviceIndex, VirtIoNetVirtualDevice>,
	
	/// `vhost` host virtual network devices by index.
	///
	/// Internally several `ctrl` threads may be created for these (see `rte_ctrl_thread_create`).
	pub virtual_host_net_virtual_devices: BTreeMap<VirtualDeviceIndex, VirtualHostNetVirtualDevice>,
	
	/// Can be changed from default (`None`).
	pub memory_channels: Option<MemoryChannels>,
	
	/// Can be changed from default (`None`).
	pub memory_ranks: Option<MemoryRanks>,
	
	/// Where and how to mount huge pages.
	pub huge_page_mount_settings: HugePageMountSettings,
	
	/// How many huge pages to allocate?
	///
	/// If `None`, then memory limits and huge page memory reservation are not done.
	pub huge_page_allocation_strategy: Option<HugePageAllocationStrategy>,
	
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
	
	#[cfg(target_os = "linux")]
	/// Can be changed from default (`None`).
	pub base_virtual_address: Option<usize>,
	
	#[cfg(target_os = "linux")]
	/// Can be changed from default (`None`).
	pub virtual_function_io_interrupt_mode: Option<VirtualFunctionIoInterruptMode>,
	
	#[cfg(target_os = "linux")]
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
			kernel_native_interface_net_virtual_devices: Default::default(),
			packet_capture_net_virtual_devices: Default::default(),
			virt_io_net_virtual_devices: Default::default(),
			virtual_host_net_virtual_devices: Default::default(),

			memory_channels: None,
			memory_ranks: None,
			huge_page_mount_settings: HugePageMountSettings::default(),
			huge_page_allocation_strategy: Some(HugePageAllocationStrategy::default()),
			huge_page_file_name_prefix: get_program_name(),

			process_type: None,
			use_high_precision_event_timer: true,
			use_shared_configuration_memory_map: false,
			use_vmware_tsc_map_instead_of_native_rdtsc: false,
			
			#[cfg(target_os = "linux")] base_virtual_address: None,
			#[cfg(target_os = "linux")] virtual_function_io_interrupt_mode: None,
			#[cfg(target_os = "linux")] create_uio_device_on_file_system_in_slash_dev: true,
		}
	}
}

impl DpdkConfiguration
{
	/// Run this after successful initialization before program termination.
	#[inline(always)]
	pub fn dpdk_clean_up()
	{
		unsafe { rte_eal_cleanup() };
	}
	
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
	pub fn initialize_dpdk<V>(&self, logging_configuration: &LoggingConfiguration, pci_devices: &HashMap<PciDevice, V>, hugetlbfs_mount_path: &Path, memory_limits: Option<MachineOrNumaNodes<MegaBytes>>, master_logical_core: HyperThread, slave_logical_cores: &BTreeSet<HyperThread>, service_logical_cores: &BTreeSet<HyperThread>)
	{
		let mut arguments = Arguments::new();

		Self::initialize_dpdk_pci_device_settings(&mut arguments, pci_devices);
		self.initialize_dpdk_virtual_device_settings(&mut arguments);
		self.initialize_dpdk_process_type_settings(&mut arguments);
		Self::initialize_dpdk_logical_core_settings(&mut arguments, master_logical_core, slave_logical_cores, service_logical_cores);
		self.initialize_dpdk_memory_limits_settings(&mut arguments, memory_limits);
		self.initialize_dpdk_huge_page_settings(&mut arguments, hugetlbfs_mount_path);
		self.initialize_dpdk_memory_rank_and_memory_channel_settings(&mut arguments);
		self.initialize_dpdk_optional_settings(&mut arguments, pci_devices);
		self.initialize_dpdk_log_settings(&mut arguments, logging_configuration);
		self.initialize_dpdk_os_specific_settings(&mut arguments);

		Self::call_rte_eal_init(arguments)
	}
	
	#[inline(always)]
	pub(crate) fn initialize_dpdk_pci_device_settings<V>(arguments: &mut Arguments, pci_devices: &HashMap<PciDevice, V>)
	{
		for pci_device in pci_devices.keys()
		{
			arguments.variable_argument(Arguments::__pci_whitelist, &pci_device.to_address_string());
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_virtual_device_settings(&self, arguments: &mut Arguments)
	{
		#[inline(always)]
		fn add_virtual_devices<V: VirtualDevice>(arguments: &mut Arguments, map: &BTreeMap<u8, V>)
		{
			for (index, virtual_device) in map.iter()
			{
				arguments.variable_argument(Arguments::__vdev, &virtual_device.as_initialization_argument(*index));
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
			arguments.constant_argument(Arguments::__proc_type, process_type.as_initialization_argument());
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_logical_core_settings(arguments: &mut Arguments, master_hyper_thread: HyperThread, slave_logical_cores: &BTreeSet<HyperThread>, service_logical_cores: &BTreeSet<HyperThread>)
	{
		let logical_cores =
		{
			let mut logical_cores = BTreeSet::new();
			logical_cores.insert(master_hyper_thread);
			for slave_logical_core in slave_logical_cores.iter()
			{
				logical_cores.insert(*slave_logical_core);
			}
			for service_logical_core in service_logical_cores.iter()
			{
				logical_cores.insert(*service_logical_core);
			}
			logical_cores
		};
		
		#[inline(always)]
		fn to_logical_core_list(logical_cores: &BTreeSet<HyperThread>) -> String
		{
			let mut logical_core_list = String::with_capacity(logical_cores.len() * 4);
			for hyper_thread in logical_cores.iter()
			{
				if !logical_core_list.is_empty()
				{
					logical_core_list.push(',');
				}
				let into: u16 = (*hyper_thread).into();
				logical_core_list.push_str(&format!("{}", into))
			}
			logical_core_list
		}
		
		arguments.variable_argument(Arguments::_l, &to_logical_core_list(&logical_cores));
		arguments.variable_argument(Arguments::_S, &to_logical_core_list(service_logical_cores));
		let into: u16 = master_hyper_thread.into();
		arguments.variable_argument(Arguments::__master_lcore, &format!("{}", into));
	}
	
	#[inline(always)]
	fn initialize_dpdk_memory_limits_settings(&self, arguments: &mut Arguments, memory_limits: Option<MachineOrNumaNodes<MegaBytes>>)
	{
		if let Some(memory_limits) = memory_limits
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
			arguments.constant_argument(Arguments::_n, override_number_of_memory_channels.as_initialization_argument());
		}
		
		if let Some(override_number_of_memory_ranks) = self.memory_ranks
		{
			arguments.constant_argument(Arguments::_r, override_number_of_memory_ranks.as_initialization_argument());
		}
	}
	
	#[inline(always)]
	fn initialize_dpdk_optional_settings<V>(&self, arguments: &mut Arguments, pci_devices: &HashMap<PciDevice, V>)
	{
		arguments.option_argument(Arguments::__no_hpet, !self.use_high_precision_event_timer);

		arguments.option_argument(Arguments::__no_pci, pci_devices.is_empty());

		arguments.option_argument(Arguments::__no_shconf, !self.use_shared_configuration_memory_map);

		arguments.option_argument(Arguments::__vmware_tsc_map, self.use_vmware_tsc_map_instead_of_native_rdtsc);
	}
	
	#[inline(always)]
	fn initialize_dpdk_log_settings(&self, arguments: &mut Arguments, logging_configuration: &LoggingConfiguration)
	{
		arguments.constant_argument(Arguments::__syslog, logging_configuration.syslog_facility.as_initialization_argument());
		arguments.constant_argument(Arguments::__log_level, logging_configuration.syslog_priority.as_initialization_argument());
	}

	#[inline(always)]
	fn initialize_dpdk_os_specific_settings(&self, arguments: &mut Arguments)
	{
		#[cfg(target_os = "linux")]
		{
			arguments.option_argument(Arguments::__create_uio_dev, self.create_uio_device_on_file_system_in_slash_dev);
	
			if let Some(base_virtual_address) = self.base_virtual_address
			{
				arguments.variable_argument(Arguments::__base_virtaddr, &format!("{0:x}", base_virtual_address));
			}
	
			if let Some(virtual_function_io_interrupt_mode) = self.virtual_function_io_interrupt_mode
			{
				arguments.constant_argument(Arguments::__vfio_intr, virtual_function_io_interrupt_mode.as_initialization_argument());
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
				
				-1 => match LogicalCore::current_logical_core_error_number()
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
