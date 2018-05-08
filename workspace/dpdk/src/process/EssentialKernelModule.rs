// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Reperesents an essential linux kernel module.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub(crate) struct EssentialKernelModule
{
	pub(crate) module_name: &'static str,
	file_base_name: &'static str,
	is_provided_by_dpdk: bool,
	depends_on_uio: bool,
}

impl EssentialKernelModule
{
	#[cfg(any(target_os = "android", target_os = "linux"))]
	const Uio: Self = Self
	{
		module_name: "uio",
		file_base_name: "uio",
		is_provided_by_dpdk: false,
		depends_on_uio: false,
	};
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub(crate) const IgbUio: Self = Self
	{
		module_name: "igb_uio",
		file_base_name: "igb_uio",
		is_provided_by_dpdk: true,
		depends_on_uio: true,
	};
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub(crate) const UioPciGeneric: Self = Self
	{
		module_name: "uio_pci_generic",
		file_base_name: "uio_pci_generic",
		is_provided_by_dpdk: false,
		depends_on_uio: true,
	};
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub(crate) const RteKni: Self = Self
	{
		module_name: "rte_kni",
		file_base_name: "rte_kni",
		is_provided_by_dpdk: true,
		depends_on_uio: false,
	};
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub(crate) const VfioPci: Self = Self
	{
		module_name: "vfio-pci",
		file_base_name: "vfio_pci", // sic: There is an underscore here.
		is_provided_by_dpdk: false,
		depends_on_uio: false,
	};
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub(crate) fn load_if_necesary(&self, modules_loaded: &mut LinuxKernelModulesList, dpdk_provided_kernel_modules_path: &Path, essential_kernel_modules_to_unload: &mut EssentialKernelModulesToUnload, dev_path: &Path)
	{
		if self.depends_on_uio
		{
			Self::Uio.load_linux_kernel_module_if_absent(modules_loaded, dpdk_provided_kernel_modules_path, essential_kernel_modules_to_unload);
		}
		
		if self.is_provided_by_dpdk
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
		}
		
		if was_loaded
		{
			essential_kernel_modules_to_unload.add_to_list_of_those_to_unload(self)
		}
		
		if self == &Self::VfioPci
		{
			Self::guard_vfio_pci_settings_are_correct(dev_path);
		}
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn guard_vfio_pci_settings_are_correct(dev_path: &Path)
	{
		Self::guard_vfio_pci_memlock_resource_limit_is_correct();
		
		Self::guard_vfio_pci_device_is_accessible(dev_path);
		
		// Additionally, the BIOS/EFI and kernel must have IO virtualization enabled, eg Intel VT-d.
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn guard_vfio_pci_memlock_resource_limit_is_correct()
	{
		const _64MegaBytesInKiloBytes: u64 = 65_536;
		
		let limits = ResourceName::MaximumNumberOfBytesThatProcessCanMemLock.get();
		if limits.hard_limit().value() < _64MegaBytesInKiloBytes
		{
			warn!("MemLock is limited to less than 64Mb; VFIO may not be able to initialize (check `ulimit -l`)");
		}
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn guard_vfio_pci_device_is_accessible(dev_path: &Path)
	{
		let mut dev_vfio_path = PathBuf::from(dev_path);
		dev_vfio_path.push("vfio");
		
		match dev_vfio_path.make_folder_searchable_to_all()
		{
			Err(error) => warn!("Could not change permissions on '{:?}' because '{}'", dev_vfio_path, error),
			Ok(_) => (),
		};
		
		let mut dev_vfio_vfio_path = PathBuf::from(dev_vfio_path);
		dev_vfio_vfio_path.push("vfio");
		
		if !dev_vfio_vfio_path.exists()
		{
			panic!("Path '{:?}' does not exist", dev_vfio_vfio_path);
		}
		
		match read_dir(&dev_vfio_vfio_path)
		{
			Err(_) => panic!("Could not read directory entries for {:?}!", dev_vfio_vfio_path),
			
			Ok(read_directory) =>
			{
				for entry in read_directory
				{
					match entry
					{
						Err(error) => panic!("Could not access directory entry for '{:?}' because '{}'", &dev_vfio_vfio_path, error),
						
						Ok(entry) =>
						{
							let path = &entry.path();
							match path.make_file_read_write_all()
							{
								Err(error) => panic!("Could not change permissions on '{:?}' because '{}'", path, error),
								
								Ok(_) => (),
							}
						},
					};
				}
			}
		}
	}
}
