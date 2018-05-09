// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Configuration
{
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
		let numaNodeData = NumaSocketId::numaNodesData(&sys_path).expect("Could not read NUMA nodes data");
		let numa_sockets = NumaSockets::detectNumaSockets(&sys_path, numaNodeData).expect("Could not detect CPUs or NUMA sockets");

		Configuration
		{
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

	pub fn dpdkRteInitData(&self, finishers: &mut Finishers) -> (DpdkConfiguration, EthernetPortConfigurations)
	{
		let mut dpdkRteInitData = DpdkConfiguration::default();

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
}
