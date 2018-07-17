// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Reperesents an essential linux kernel module.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct EssentialKernelModule
{
	module_name: &'static str,
	file_base_name: &'static str,
	is_provided_by_dpdk: bool,
	depends_on_uio: bool,
}

impl EssentialKernelModule
{
	/// UIO kernel module.
	#[cfg(target_os = "linux")]
	pub const Uio: Self = Self
	{
		module_name: "uio",
		file_base_name: "uio",
		is_provided_by_dpdk: false,
		depends_on_uio: false,
	};
	
	/// IGB-UIO kernel module.
	///
	/// Supplied out-of-tree with DPDK.
	#[cfg(target_os = "linux")]
	pub const IgbUio: Self = Self
	{
		module_name: "igb_uio",
		file_base_name: "igb_uio",
		is_provided_by_dpdk: true,
		depends_on_uio: true,
	};
	
	/// UIO-PCI-Generi kernel modulec.
	#[cfg(target_os = "linux")]
	pub const UioPciGeneric: Self = Self
	{
		module_name: "uio_pci_generic",
		file_base_name: "uio_pci_generic",
		is_provided_by_dpdk: false,
		depends_on_uio: true,
	};
	
	/// RTE-KNI kernel module.
	///
	/// Supplied out-of-tree with DPDK.
	#[cfg(target_os = "linux")]
	pub const RteKni: Self = Self
	{
		module_name: "rte_kni",
		file_base_name: "rte_kni",
		is_provided_by_dpdk: true,
		depends_on_uio: false,
	};
	
	/// VFIO-PCI kernel module.
	///
	/// Supplied out-of-tree with DPDK.
	#[cfg(target_os = "linux")]
	pub const VfioPci: Self = Self
	{
		module_name: "vfio-pci",
		file_base_name: "vfio_pci", // sic: There is an underscore here.
		is_provided_by_dpdk: false,
		depends_on_uio: false,
	};
	
	/// Module name.
	#[inline(always)]
	pub fn module_name(&self) -> &'static str
	{
		self.module_name
	}
	
	/// Load this kernel module if necessary.
	#[cfg(target_os = "linux")]
	pub fn load_if_necesary(&self, modules_loaded: &mut LinuxKernelModulesList, dpdk_provided_kernel_modules_path: &Path, essential_kernel_modules_to_unload: &mut EssentialKernelModulesToUnload, dev_path: &Path)
	{
		if self.depends_on_uio
		{
			Self::Uio.load_if_necesary(modules_loaded, dpdk_provided_kernel_modules_path, essential_kernel_modules_to_unload, dev_path);
		}
		
		let was_loaded = if self.is_provided_by_dpdk
		{
			match modules_loaded.load_linux_kernel_module_if_absent_from_ko_file(self.module_name, self.file_base_name, dpdk_provided_kernel_modules_path)
			{
				Ok(was_loaded) => was_loaded,
				
				Err(error) => panic!("Could not load absent '{}' kernel module (file name is `{}.ko`) provided by DPDK from path {:?} because '{}'; check your module versions and kernel version match", self.module_name.to_owned(), self.file_base_name.to_owned(), dpdk_provided_kernel_modules_path, error),
			}
		}
		else
		{
			match modules_loaded.load_linux_kernel_module_if_absent_using_modprobe(self.module_name, self.file_base_name)
			{
				Ok(was_loaded) => was_loaded,
				
				Err(error) => panic!("Could not load absent '{}' kernel module (file name is probably `{}.ko`) using modprobe because '{}'; check your module versions and kernel version match (use `uname -r`), because this module is quite common", self.module_name.to_owned(), self.file_base_name.to_owned(), error)
			}
		};
		
		if was_loaded
		{
			essential_kernel_modules_to_unload.add_to_list_of_those_to_unload(self)
		}
		
		if self == &Self::VfioPci
		{
			Self::guard_vfio_pci_memlock_resource_limit_is_correct();
		}
	}
	
	#[cfg(target_os = "linux")]
	#[inline(always)]
	fn guard_vfio_pci_memlock_resource_limit_is_correct()
	{
		const _64MegaBytesInKiloBytes: u64 = 65_536;
		
		let limits = ResourceName::MaximumNumberOfBytesThatProcessCanMemLock.get();
		if limits.hard_limit().value() < _64MegaBytesInKiloBytes
		{
			panic!("MemLock is limited to less than 64Mb; VFIO will not be able to initialize (check `ulimit -l`)");
		}
	}
}
