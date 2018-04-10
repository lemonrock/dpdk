// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
pub struct DpdkRteInitData<'a>
{
	pub listOfPciDevices: HashSet<DeviceAddress>,
	
	pub afPacketNetVirtualDevices: VirtualDeviceConfigurations<AfPacketNetVirtualDevice, ()>,
	pub nullNetVirtualDevices: VirtualDeviceConfigurations<NullNetVirtualDevice, ()>,
	pub packetCaptureNetVirtualDevices: VirtualDeviceConfigurations<PacketCaptureNetVirtualDevice, ()>,
	pub ringNetVirtualDevices: VirtualDeviceConfigurations<RingNetVirtualDevice, ()>,
	pub virtIoForContainersNetVirtualDevices: VirtualDeviceConfigurations<VirtIoForContainersNetVirtualDevice, ()>,
	pub virtualHostNetVirtualDevices: VirtualDeviceConfigurations<VirtualHostNetVirtualDevice, ()>,
	pub xenNetVirtualDevices: VirtualDeviceConfigurations<XenNetVirtualDevice, ()>,
	
	pub bondingNetVirtualDevices: VirtualDeviceConfigurations<BondingNetVirtualDevice, ()>,

	pub aesNiGcmCryptoVirtualDevices: VirtualDeviceConfigurations<AesNiGcmCryptoVirtualDevice, ()>,
	pub aesNiMbCryptoVirtualDevices: VirtualDeviceConfigurations<AesNiMultiBufferCryptoVirtualDevice, ()>,
	pub kasumiCryptoVirtualDevices: VirtualDeviceConfigurations<KasumiCryptoVirtualDevice, ()>,
	pub nullCryptoVirtualDevices: VirtualDeviceConfigurations<NullCryptoVirtualDevice, ()>,
	pub snow3gCryptoVirtualDevices: VirtualDeviceConfigurations<Snow3gCryptoVirtualDevice, ()>,
	
	pub additionalDynamicLibraryDriversAndPluginsToLoad: HashSet<&'a Path>,
	
	pub overrideNumberOfMemoryChannels: Option<MemoryChannels>,
	pub overrideNumberOfMemoryRanks: Option<MemoryRanks>,
	pub memoryLimits: Option<MemoryLimits>,
	
	pub processType: Option<ProcessType>,
	pub useHighPrecisionTimer: bool,
	pub useSharedConfigurationMemoryMap: bool,
	pub useVmWareTscMapInsteadOfNativeRdtsc: bool,
	
	#[cfg(any(target_os = "android", target_os = "linux"))] pub supportRunningOnXenDomain0WithoutHugetlbfs: bool,
	#[cfg(any(target_os = "android", target_os = "linux"))] pub baseVirtualAddress: Option<usize>,
	#[cfg(any(target_os = "android", target_os = "linux"))] pub vfioInterruptMode: Option<VfioInterruptMode>,
	#[cfg(any(target_os = "android", target_os = "linux"))] pub createUioDeviceOnFileSystemInSlashDev: bool,
}

impl<'a> Default for DpdkRteInitData<'a>
{
	fn default() -> Self
	{
		DpdkRteInitData
		{
			listOfPciDevices: HashSet::new(),
			
			afPacketNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			nullNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			packetCaptureNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			ringNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			virtIoForContainersNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			virtualHostNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			xenNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			
			bondingNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			
			aesNiGcmCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			aesNiMbCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			kasumiCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			nullCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			snow3gCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			
			additionalDynamicLibraryDriversAndPluginsToLoad: HashSet::new(),
			
			memoryLimits: None,
			overrideNumberOfMemoryChannels: None,
			overrideNumberOfMemoryRanks: None,
			
			processType: None,
			useHighPrecisionTimer: true,
			useSharedConfigurationMemoryMap: false,
			useVmWareTscMapInsteadOfNativeRdtsc: false,
		
			supportRunningOnXenDomain0WithoutHugetlbfs: false,
			baseVirtualAddress: None,
			vfioInterruptMode: None,
			createUioDeviceOnFileSystemInSlashDev: true,
		}
	}
	
	#[cfg(not(any(target_os = "android", target_os = "linux")))]
	fn default() -> Self
	{
		Initialisation
		{
			listOfPciDevices: HashSet::new(),
			
			afPacketNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			nullNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			packetCaptureNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			ringNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			virtIoForContainersNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			virtualHostNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			xenNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			
			bondingNetVirtualDevices: VirtualDeviceConfigurations::empty(),
			
			aesNiGcmCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			aesNiMbCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			kasumiCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			nullCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			snow3gCryptoVirtualDevices: VirtualDeviceConfigurations::empty(),
			
			additionalDynamicLibraryDriversAndPluginsToLoad: HashSet::new(),
			
			memoryLimits: None,
			overrideNumberOfMemoryChannels: None,
			overrideNumberOfMemoryRanks: None,
			
			processType: None,
			useHighPrecisionTimer: true,
			useSharedConfigurationMemoryMap: false,
			useVmWareTscMapInsteadOfNativeRdtsc: false,
		}
	}
}

impl<'a> DpdkRteInitData<'a>
{
	pub fn addPciDevice(&mut self, deviceAddress: DeviceAddress)
	{
		assert!(self.listOfPciDevices.insert(deviceAddress), "Non-unique device address");
	}
	
	pub fn addAfPacketNetVirtualDevice(&mut self, afPacketNetVirtualDevice: AfPacketNetVirtualDevice)
	{
		self.afPacketNetVirtualDevices.createConfiguration(afPacketNetVirtualDevice, ());
	}
	
	pub fn addPacketCaptureNetVirtualDevice(&mut self, packetCaptureNetVirtualDevice: PacketCaptureNetVirtualDevice)
	{
		self.packetCaptureNetVirtualDevices.createConfiguration(packetCaptureNetVirtualDevice, ());
	}
	
	pub fn addVirtIoForContainersNetVirtualDevice(&mut self, virtIoForContainersNetVirtualDevice: VirtIoForContainersNetVirtualDevice)
	{
		self.virtIoForContainersNetVirtualDevices.createConfiguration(virtIoForContainersNetVirtualDevice, ());
	}
	
	pub fn addVirtualHostNetVirtualDevice(&mut self, virtualHostNetVirtualDevices: VirtualHostNetVirtualDevice)
	{
		self.virtualHostNetVirtualDevices.createConfiguration(virtualHostNetVirtualDevices, ());
	}
	
	pub fn addXenNetVirtualDevice(&mut self, xenNetVirtualDevices: XenNetVirtualDevice)
	{
		self.xenNetVirtualDevices.createConfiguration(xenNetVirtualDevices, ());
	}
	
	pub fn addBondingNetVirtualDevice(&mut self, bondingNetVirtualDevice: BondingNetVirtualDevice)
	{
		self.bondingNetVirtualDevices.createConfiguration(bondingNetVirtualDevice, ());
	}
	
	pub fn addMemorySettings(&mut self, memoryLimits: Option<MemoryLimits>, numberOfMemoryChannels: Option<MemoryChannels>, numberOfMemoryRanks: Option<MemoryRanks>)
	{
		self.memoryLimits = memoryLimits;
		self.overrideNumberOfMemoryChannels = numberOfMemoryChannels;
		self.overrideNumberOfMemoryRanks = numberOfMemoryRanks;
	}
	
	pub fn initialiseDpdk(&self, numaSockets: &NumaSockets, initialisationHugePageSettings: HugePageFilePathInformation)
	{
		let hugePageDetails = initialisationHugePageSettings.hugePageFileSystemMountPathAndSoOn();
		let useHugePages = hugePageDetails.is_some();
		
		let mut arguments: Vec<*const c_char> = Vec::initialise();
		
		self.initPciDevices(&mut arguments);
		self.initVirtualDevices(&mut arguments);
		self.initDynamicLibraryDriverAndPluginsToLoad(&mut arguments);
		self.initProcessTypeSettings(&mut arguments);
		Self::initLogicalCoreSettings(&mut arguments, numaSockets);
		self.initMemoryLimits(&mut arguments, useHugePages, numaSockets);
		self.initMemoryRankAndChannelSettings(&mut arguments);
		self.initHugePageSettings(&mut arguments, hugePageDetails);
		self.initOptionalSettings(&mut arguments);
		self.initLogSettings(&mut arguments);
		self.initOsSpecificSettings(&mut arguments);
		
		Self::call_rte_eal_init(arguments);
	}
	
	fn initPciDevices(&self, mut arguments: &mut Vec<*const c_char>)
	{
		if self.listOfPciDevices.is_empty()
		{
			return;
		}
		
		let pciDeviceListKey = DeviceListColour::Whitelist.asInitialisationArgument();
		for pciDeviceAddress in &self.listOfPciDevices
		{
			let value = pciDeviceAddress.asCString();
			arguments.keyCStrValue(pciDeviceListKey, &value);
		}
	}
	
	fn initVirtualDevices(&self, mut arguments: &mut Vec<*const c_char>)
	{
		self.afPacketNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.nullNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.packetCaptureNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.ringNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.virtIoForContainersNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.virtualHostNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.xenNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);

		self.bondingNetVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		
		self.aesNiGcmCryptoVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.aesNiMbCryptoVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.kasumiCryptoVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.nullCryptoVirtualDevices.addVirtualDevicesSorted(&mut arguments);
		self.snow3gCryptoVirtualDevices.addVirtualDevicesSorted(&mut arguments);
	}
	
	fn initDynamicLibraryDriverAndPluginsToLoad(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			_d = "-d"; // libX.so, dynamically load additional drivers or plugins
		}
		
		for additionalDynamicLibraryDriverOrPluginToLoad in &self.additionalDynamicLibraryDriversAndPluginsToLoad
		{
			let cString = additionalDynamicLibraryDriverOrPluginToLoad.to_c_string();
			arguments.keyCStrValue(_d, &cString);
		}
	}
	
	fn initProcessTypeSettings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__proc_type = "--proc-type";            // For multi-process set ups
		}
		
		if let Some(processType) = self.processType
		{
			arguments.keyConstantValue(__proc_type, processType.asInitialisationArgument());
		}
	}
	
	fn initLogicalCoreSettings(mut arguments: &mut Vec<*const c_char>, numaSockets: &NumaSockets)
	{
		const_cstr!
		{
			_c = "-c";                              // COREMASK
			__master_lcore = "--master-lcore";      // u32
			// _l = "-l";                           // CORELIST
			// __lcores = "--lcores";               // COREMAP, mapping of logical cores to physical CPUs, (see http://dpdk.org/doc/guides/testpmd_app_ug/run_app.html)
		}
		
		let value = numaSockets.logicalCoresActive.asHexadecimalCoreMaskCString();
		arguments.keyCStrValue(_c, &value);
		
		let value = CString::new(format!("{}", numaSockets.masterLogicalCore.as_u32())).unwrap();
		arguments.keyCStrValue(__master_lcore, &value);
	}
	
	fn initMemoryRankAndChannelSettings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			_n = "-n";                              // 31-bit, != 0, Number of memory channels to use
			_r = "-r";                              // 5-bit, != 0, <= 16, Number of memory ranks to use
		}
		
		if let Some(overrideNumberOfMemoryChannels) = self.overrideNumberOfMemoryChannels
		{
			let value = CString::new(format!("{}", overrideNumberOfMemoryChannels as u32)).unwrap();
			arguments.keyCStrValue(_n, &value);
		}
		
		if let Some(overrideNumberOfMemoryRanks) = self.overrideNumberOfMemoryRanks
		{
			let value = CString::new(format!("{}", overrideNumberOfMemoryRanks as u8)).unwrap();
			arguments.keyCStrValue(_r, &value);
		}
	}
	
	fn initMemoryLimits(&self, mut arguments: &mut Vec<*const c_char>, useHugePages: bool, numaSockets: &NumaSockets)
	{
		fn initTotalMemoryLimits(mut arguments: &mut Vec<*const c_char>, sizeOfTotalMemoryInMegabytes: u31)
		{
			const_cstr!
			{
				_m = "-m";                              // u32 Mb of RAM (as Mb, not bytes); Maximum of 512 Gb; maximum DPDK supports
			}
			
			let value = CString::new(format!("{}", sizeOfTotalMemoryInMegabytes)).unwrap();
			arguments.keyCStrValue(_m, &value);
		}
		
		fn initPerNumaNodeMemoryLimits(mut arguments: &mut Vec<*const c_char>, perNumaNodeString: CString)
		{
			const_cstr!
			{
				__socket_mem = "--socket-mem";          // Conflicts with -m and useHugePages=false
			}
			
			arguments.keyCStrValue(__socket_mem, &perNumaNodeString);
		}
		
		if let Some(memoryLimits) = self.memoryLimits
		{
			if cfg!(not(any(target_os = "android", target_os = "linux")))
			{
				if let Some(totalMemory) = memoryLimits.totalMemoryInMegabytes(numaSockets)
				{
					initTotalMemoryLimits(arguments, totalMemory);
				}
			}
			else
			{
				if !useHugePages
				{
					panic!("Can not have per NUMA socket memory (memoryLimits) and then have useHugePages as false");
				}
				
				let (perNumaNodeOption, totalMemoryOption) = memoryLimits.asInitialisationStringIfIsANumaMachine(useHugePages, numaSockets);
				if let Some(perNumaNode) = perNumaNodeOption
				{
					initPerNumaNodeMemoryLimits(arguments, perNumaNode)
				}
				else
				{
					if let Some(totalMemory) = totalMemoryOption
					{
						initTotalMemoryLimits(arguments, totalMemory);
					}
				}
			}
		}
	}
	
	fn initHugePageSettings(&self, mut arguments: &mut Vec<*const c_char>, hugePageFileSystemMountPath: Option<(&Path, Option<&OsStr>)>)
	{
		const_cstr!
		{
			__huge_dir = "--huge-dir";
			__huge_unlink = "--huge-unlink";
			__no_huge = "--no-huge";
			__file_prefix = "--file-prefix";
		}
		
		if let Some((hugePageFileSystemMountPath, hugePageFileNamePrefix)) = hugePageFileSystemMountPath
		{
			let cString = hugePageFileSystemMountPath.to_c_string();
			arguments.keyCStrValue(__huge_dir, &cString);
			
			arguments.optionalArgument(__no_huge, false);
			
			if self.processType.is_none()
			{
				arguments.optionalArgument(__huge_unlink, true);
			}
			
			if let Some(hugePageFileNamePrefix) = hugePageFileNamePrefix
			{
				let cString = hugePageFileNamePrefix.os_str_to_c_string();
				arguments.keyCStrValue(__file_prefix, &cString);
			}
		}
		else
		{
			arguments.optionalArgument(__no_huge, true);
		}
	}

	fn initOptionalSettings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__no_hpet = "--no-hpet";                // Debug use only
			__no_pci = "--no-pci";                  // Debug use only
			__no_shconf = "--no-shconf";            // Debug use only
			__vmware_tsc_map = "--vmware-tsc-map";  //
		}
		
		arguments.optionalArgument(__no_hpet, !self.useHighPrecisionTimer);
		
		arguments.optionalArgument(__no_pci, self.listOfPciDevices.is_empty());
		
		arguments.optionalArgument(__no_shconf, !self.useSharedConfigurationMemoryMap);
		
		arguments.optionalArgument(__vmware_tsc_map, self.useVmWareTscMapInsteadOfNativeRdtsc);
	}
	
	fn initLogSettings(&self, arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__syslog = "--syslog";                  // A facility. Not configurable, as we really don't know what DPDK will produce, so we send to 'auth'
			__log_level = "--log-level";            // A log level. Not configurable; we choose either a debug one or a production one
		}
		
		arguments.keyConstantValue(__syslog, const_cstr!("auth"));
		
		let logLevel = if cfg!(debug_assertions)
		{
			const_cstr!("8") // RTE_LOG_DEBUG
		}
		else
		{
			const_cstr!("5") // RTE_LOG_WARNING
		};
		arguments.keyConstantValue(__log_level, logLevel);
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	fn initOsSpecificSettings(&self, mut arguments: &mut Vec<*const c_char>)
	{
		const_cstr!
		{
			__xen_dom0 = "--xen-dom0";
			__base_virtaddr = "--base-virtaddr";
			__vfio_intr = "--vfio-intr";
			__create_uio_dev = "--create-uio-dev";
		}
	
		arguments.optionalArgument(__xen_dom0, self.supportRunningOnXenDomain0WithoutHugetlbfs);
	
		arguments.optionalArgument(__create_uio_dev, self.createUioDeviceOnFileSystemInSlashDev);
	
		if let Some(baseVirtualAddress) = self.baseVirtualAddress
		{
			let value = &CString::new(format!("{0:x}", baseVirtualAddress)).unwrap();
			arguments.keyCStrValue(__base_virtaddr, value);
		}
	
		if let Some(vfioInterruptMode) = self.vfioInterruptMode
		{
			arguments.keyConstantValue(__vfio_intr, vfioInterruptMode.asInitialisationArgument());
		}
	}

	#[cfg(not(any(target_os = "android", target_os = "linux")))]
	fn addOsSpecificSettings(&self, mut arguments: &mut Vec<*const c_char>)
	{
	}
	
	fn call_rte_eal_init(mut arguments: Vec<*const c_char>)
	{
		let count = arguments.len();
		arguments.push(null_mut());
		
		let argc = count as c_int;
		let argv = arguments.as_mut_ptr() as *mut *mut c_char;
		
		match unsafe { ::dpdk_sys::rte_eal_init(argc, argv) }
		{
			numberOfParsedArguments if numberOfParsedArguments >= 0 =>
				{
					if numberOfParsedArguments != count as c_int
					{
						panic!("Parsed only numberOfParsedArguments '{}' but provided count '{}' arguments", numberOfParsedArguments, count);
					}
				},
			
			error @ _ => panic!("Could not initialise DPDK Envrironment Abstraction Layer, received error '{}'", error),
		}
	}
}
