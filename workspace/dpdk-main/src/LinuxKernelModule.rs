// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct LinuxKernelModule
{
	moduleName: &'static str,
	fileBaseName: &'static str,
	isProvidedByDpdk: bool,
}

impl LinuxKernelModule
{
	pub const Uio: LinuxKernelModule = LinuxKernelModule
	{
		moduleName: "uio",
		fileBaseName: "uio",
		isProvidedByDpdk: false,
	};
	
	pub const IgbUio: LinuxKernelModule = LinuxKernelModule
	{
		moduleName: "igb_uio",
		fileBaseName: "igb_uio",
		isProvidedByDpdk: true,
	};
	
	pub const UioPciGeneric: LinuxKernelModule = LinuxKernelModule
	{
		moduleName: "uio_pci_generic",
		fileBaseName: "uio_pci_generic",
		isProvidedByDpdk: false,
	};
	
	pub const RteKni: LinuxKernelModule = LinuxKernelModule
	{
		moduleName: "rte_kni",
		fileBaseName: "rte_kni",
		isProvidedByDpdk: true,
	};
	
	pub const VfioPci: LinuxKernelModule = LinuxKernelModule
	{
		moduleName: "vfio-pci",
		fileBaseName: "vfio_pci", // sic: There is an underscore here
		isProvidedByDpdk: false,
	};
	
	pub const XenDom0Mm: LinuxKernelModule = LinuxKernelModule
	{
		moduleName: "rte_dom0_mm",
		fileBaseName: "rte_dom0_mm",
		isProvidedByDpdk: true,
	};
	
	pub fn loadModuleIfAbsent(&self, modulesList: &ModulesList, loadModulesFromPath: &Path, finishers: &mut Finishers)
	{
		if self.isProvidedByDpdk
		{
			match modulesList.loadModuleIfAbsentFromKoFile(self.moduleName, self.fileBaseName, loadModulesFromPath)
			{
				Err(error) => panic!("Could not load absent '{}' kernel module (file name is `{}.ko`) provided by DPDK from path {:?} because '{}'; check your module versions and kernel version match", self.moduleName.to_owned(), self.fileBaseName.to_owned(), loadModulesFromPath, error),
				Ok(wasLoaded) => UnloadModuleFinisher::ifWasLoaded(wasLoaded, finishers, self.moduleName),
			}
		}
		else
		{
			if let Err(error) = modulesList.loadModuleIfAbsent(self.moduleName, self.fileBaseName)
			{
				panic!("Could not load absent '{}' kernel module (file name is probably `{}.ko`) using modprobe because '{}'; check your module versions and kernel version match (use `uname -r`), because this module is quite common", self.moduleName.to_owned(), self.fileBaseName.to_owned(), error);
			}
			UnloadModuleFinisher::ifWasLoaded(true, finishers, self.moduleName);
		}
	}
	
	pub fn loadAbsentModules(procPath: &Path, loadModulesFromPath: &Path, modules: Vec<LinuxKernelModule>, finishers: &mut Finishers)
	{
		let modulesList = ModulesList::parse(procPath).expect("Could not parse modules list");
		for module in modules
		{
			module.loadModuleIfAbsent(&modulesList, loadModulesFromPath, finishers);
		}
	}
}
