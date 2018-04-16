// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Configuration
{
	procPath: PathBuf,
	sys_path: PathBuf,
	devPath: PathBuf,
	loadModulesFromPath: PathBuf,
	hugePagesConfiguration: HugePagesConfiguration,
	numa_sockets: NumaSockets,
	memoryConfiguration: MemoryConfiguration,
	networkInterfacesConfiguration: NetworkInterfacesConfiguration,

	#[cfg(any(target_os = "android", target_os = "linux"))] resource_limits: ResourceLimitsSet,
}

impl Default for Configuration
{
	fn default() -> Self
	{
		let procPath = PathBuf::from("/proc");
		let sys_path = PathBuf::from("/sys");

		let numaNodeData = NumaSocketId::numaNodesData(&sys_path).expect("Could not read NUMA nodes data");
		let numa_sockets = NumaSockets::detectNumaSockets(&sys_path, numaNodeData).expect("Could not detect CPUs or NUMA sockets");

		fn parentPath() -> PathBuf
		{
			if let Ok(path) = current_exe()
			{
				if let Ok(path) = path.canonicalize()
				{
					if let Some(parent) = path.parent()
					{
						return parent.to_path_buf()
					}
				}
			}
			PathBuf::from("/")
		}

		// remove bin and replace with lib, or push lib (eg if not in /bin, /usr/bin, /usr/local/bin or /opt/<package>/bin
		let mut loadModulesFromPath = parentPath();
		if loadModulesFromPath.to_str().map(|path| path.ends_with("/bin") || path.ends_with("/sbin")).unwrap_or(false)
		{
			loadModulesFromPath.set_file_name("lib");
		}
		else
		{
			loadModulesFromPath.push("lib");
		}
		loadModulesFromPath.push("linux_kernel_modules/dpdk");

		let resource_limits = ResourceLimitsSet::defaultish(ResourceLimit::maximum_number_of_open_file_descriptors(&procPath).expect("Could not read maximum number of file descriptors"));

		Configuration
		{
			procPath,
			sys_path,
			devPath: PathBuf::from("/dev"),
			loadModulesFromPath,
			hugePagesConfiguration: HugePagesConfiguration::default(),
			numa_sockets,
			memoryConfiguration: MemoryConfiguration::default(),
			networkInterfacesConfiguration: NetworkInterfacesConfiguration::default(),

			resource_limits,
		}
	}
}

impl Configuration
{
	#[inline(always)]
	pub fn procPath(&self) -> &Path
	{
		&self.procPath
	}

	#[inline(always)]
	pub fn sys_path(&self) -> &Path
	{
		&self.sys_path
	}

	#[inline(always)]
	pub fn devPath(&self) -> &Path
	{
		&self.devPath
	}

	#[inline(always)]
	pub fn borrowNumaSockets(&self) -> &NumaSockets
	{
		&self.numa_sockets
	}

	pub fn destroyAsNumaSockets(self) -> NumaSockets
	{
		self.numa_sockets
	}

	fn use_pci_kernel_driver(&self, pci_kernel_driver: PciKernelDriver) -> bool
	{
		self.networkInterfacesConfiguration.use_pci_kernel_driver(pci_kernel_driver)
	}

	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn linuxModules(&self) -> (&Path, Vec<LinuxKernelModule>, bool)
	{
		let usesVfioPciKernelModule = self.usesVfioPciKernelModule();
		(self.loadModulesFromPath(), self.dpdkModulesToEnsureLoaded(usesVfioPciKernelModule), usesVfioPciKernelModule)
	}

	fn usesVfioPciKernelModule(&self) -> bool
	{
		self.use_pci_kernel_driver(PciKernelDriver::VfioPci)
	}

	fn loadModulesFromPath(&self) -> &Path
	{
		&self.loadModulesFromPath
	}

	#[cfg(any(target_os = "android", target_os = "linux"))]
	fn dpdkModulesToEnsureLoaded(&self, usesVfioPciKernelModule: bool) -> Vec<LinuxKernelModule>
	{
		let mut modules = Vec::with_capacity(6);

		let mut dependsOnUio = false;
		if self.use_pci_kernel_driver(PciKernelDriver::IgbUio)
		{
			dependsOnUio = true;
			modules.push(LinuxKernelModule::IgbUio);
		}

		if self.use_pci_kernel_driver(PciKernelDriver::UioPciGeneric)
		{
			dependsOnUio = true;
			modules.push(LinuxKernelModule::UioPciGeneric);
		}

		if dependsOnUio
		{
			modules.insert(0, LinuxKernelModule::Uio);
		}

		if self.networkInterfacesConfiguration.hasKernelNativeInterfaceDevices()
		{
			modules.push(LinuxKernelModule::RteKni);
		}

		if usesVfioPciKernelModule
		{
			modules.push(LinuxKernelModule::VfioPci);
		}

		if self.networkInterfacesConfiguration.hasXenVirtualDevices()
		{
			modules.push(LinuxKernelModule::XenDom0Mm);
		}

		modules
	}

	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn setUpHugePagesAndNumaMemory(&self, finishers: &mut Finishers) -> HugePageFilePathInformation
	{
		let (hugePageMountPathOption, hugePageFinisher) = self.hugePagesConfiguration.setUpHugePagesAndNumaMemory(self.procPath(), self.sys_path(), self.borrowNumaSockets());

		finishers.push(Box::new(hugePageFinisher));
		HugePageFilePathInformation::new(hugePageMountPathOption)
	}
	
	#[cfg(target_os = "freebsd")]
	pub fn setUpHugePagesAndNumaMemory(&self, finishers: &mut Finishers) -> (HugePageFilePathInformation, HugePageFinisher)
	{
		finishers.push(Box::new(HugePageFinisher::FreeBsd));
		HugePageFilePathInformation::new()
	}

	pub fn dpdkRteInitData(&self, finishers: &mut Finishers) -> (DpdkRteInitData, EthernetPortConfigurations)
	{
		let mut dpdkRteInitData = DpdkRteInitData::default();

		self.memoryConfiguration.addTo(&mut dpdkRteInitData);
		let (unbinds, ethernetPortConfigurations) = self.networkInterfacesConfiguration.addTo(&mut dpdkRteInitData, &self.sys_path);

		let pci_devices_finisher = PciDevicesFinisher
		{
			unbinds,
		};
		finishers.push(Box::new(pci_devices_finisher));

		(dpdkRteInitData, ethernetPortConfigurations)
	}

	pub fn changeResourceLimits(&self)
	{
		self.resource_limits.change();
	}

	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn loadAndConfigureLinuxKernelModules(&self, finishers: &mut Finishers)
	{
		loadAndConfigureLinuxKernelModules(self, finishers);
	}

	#[cfg(not(any(target_os = "android", target_os = "linux")))]
	pub fn loadAndConfigureLinuxKernelModules(&self, finishers: &mut Finishers)
	{
	}
}
