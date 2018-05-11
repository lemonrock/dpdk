// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
struct KernelCommandLineValidator(LinuxKernelCommandLineParameters);

impl KernelCommandLineValidator
{
	#[inline(always)]
	pub(crate) fn validate(path_configuration: &PathConfiguration, warnings_to_suppress: &WarningsToSuppress, cpu_features: &CpuFeatures, pci_net_devices_configuration: &PciNetDevicesConfiguration) -> BTreeSet<HyperThread>
	{
		let (uses_igb_uio, uses_vfio_pci) = pci_net_devices_configuration.uses_ugb_uio_or_pci_vfio();
		
		let kernel_command_line_validator = KernelCommandLineValidator::new(&path_configuration.proc_path);
		kernel_command_line_validator.validate_dpdk_pci_drivers(uses_igb_uio, uses_vfio_pci);
		let isolated_hyper_threads = kernel_command_line_validator.validate_cpus();
		kernel_command_line_validator.validate_huge_page_sizes(cpu_features.has_1gb_huge_pages);
		kernel_command_line_validator.panic_on_incompatible_settings(cpu_features.has_1gb_huge_pages);
		kernel_command_line_validator.warnings(warnings_to_suppress);
		
		isolated_hyper_threads
	}
	
	#[inline(always)]
	fn new(proc_path: &ProcPath) -> Self
	{
		KernelCommandLineValidator(proc_path.linux_command_line_parameters().expect("Could not parse linux command line parameters"))
	}
	
	#[inline(always)]
	fn validate_dpdk_pci_drivers(&self, uses_igb_uio: bool, uses_vfio_pci: bool)
	{
		if uses_igb_uio
		{
			if let Some(iommu_setting) = self.0.iommu()
			{
				if iommu_setting != "pt"
				{
					panic!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu` is not `iommu=pt` (pass through)");
				}
			}
			else
			{
				panic!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu=pt` (pass through) was not specified");
			}
			
			if let Some(intel_iommu_setting) = self.0.intel_iommu()
			{
				if intel_iommu_setting != "on"
				{
					panic!("Using igb_uio driver and iommu Linux Kernel command line parameter `intel_iommu` is not `intel_iommu=on`");
				}
			}
			else
			{
				panic!("Using igb_uio driver and iommu Linux Kernel command line parameter `intel_iommu=on` was not specified");
			}
		}
		
		if uses_vfio_pci
		{
			if let Some(iommu_setting) = self.0.iommu()
			{
				if iommu_setting != "pt" || iommu_setting != "on"
				{
					panic!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu` is not `iommu=pt` (pass through) or `iommu=on`");
				}
			}
			else
			{
				panic!("Using igb_uio driver and iommu Linux Kernel command line parameter `iommu=pt` (pass through) or `iommu=on` was not specified");
			}
		}
	}
	
	#[inline(always)]
	fn validate_cpus(&self) -> BTreeSet<HyperThread>
	{
		let (isolated_cpu_flags, isolated_cpus) = self.0.isolcpus().expect("Kernel parameter `isolcpus` should be specified");
		
		assert!(isolated_cpu_flags.contains("domain"), "Kernel parameter `isolcpus` does not contain or imply domain flag");
		
		let rcu_nocbs = self.0.rcu_nocbs().expect("Kernel parameter `rcu_nocbs` should be specified because isolcpus was specified");
		
		let nohz_full = self.0.nohz_full().expect("Kernel parameter `nohz_full` should be specified because isolcpus was specified");
		
		// let irqaffinity = self.0.irqaffinity().expect("Kernel parameter `irqaffinity` should be specified because isolcpus was specified");
		
		if isolated_cpus != rcu_nocbs || rcu_nocbs != nohz_full
		{
			panic!("Kernel parameters `isolcpus`, `rcu_nocbs` and `nohz_full` should all match")
		}
		
		isolated_cpus.iter().map(|value| HyperThread(value)).collect()
	}
	
	#[inline(always)]
	fn validate_huge_page_sizes(&self, cpu_supports_1gb_pages: bool)
	{
		if cpu_supports_1gb_pages
		{
			match self.0.default_hugepagesz()
			{
				Some("1G") => (),
				_ => panic!("Kernel should have `default_hugepagesz=1G` for this CPU"),
			}
			
			let huge_page_sizes = self.0.hugepagesz().expect("Kernel should have `hugepagesz` for this CPU");
			
			let hugepages = self.0.hugepages().expect("Kernel should have `hugepages` for this CPU");
			
			assert_eq!(huge_page_sizes.len(), hugepages.len(), "Kernel should have equal numbers of definitions of `hugepagesz` and `hugepages`");
			
			for huge_page_size in huge_page_sizes.iter()
			{
				match huge_page_size
				{
					"1G" | "2M" => (),
					_ => panic!("Invalid Kernel 'hugepagesz={}'", huge_page_size),
				}
			}
		}
		else
		{
			match self.0.default_hugepagesz()
			{
				None | Some("2M") => (),
				_ => panic!("Kernel should have `default_hugepagesz=2M` for this CPU"),
			}
			
			let huge_page_sizes_option = self.0.hugepagesz();
			let hugepages_option = self.0.hugepages();
			
			assert!(huge_page_sizes_option.is_none() && hugepages_option.is_none() || huge_page_sizes_option.is_some() && hugepages_option.is_some(), "Define either both of `hugepagesz` or `hugepage` or neither");
			
			if huge_page_sizes_option.is_some() && hugepages_option.is_some()
			{
				assert_eq!(huge_page_sizes_option.unwrap().len(), hugepages_option.unwrap().len(), "Kernel should have equal numbers of definitions of `hugepagesz` and `hugepages`");
				
				assert_eq!(huge_page_sizes_option.unwrap().len(), 1, "This CPU only supports one size of huge page");
				assert_eq!(huge_page_sizes_option.iter().next().unwrap(), "2M", "This CPU only supports `hugepagesz=2M`");
			}
		}
	}
	
	#[inline(always)]
	fn panic_on_incompatible_settings(&self, cpu_supports_1gb_pages: bool)
	{
		if self.0.norandmaps()
		{
			panic!("Kernel has `norandmaps` enabled; this isn't secure")
		}
		
		if self.0.nokaslr()
		{
			panic!("Kernel has `nokaslr` enabled; this isn't secure")
		}
		
		if self.0.movable_node()
		{
			panic!("Kernel has `movable_node` enabled; this isn't compatible with this application")
		}
		
		if self.0.nosmp()
		{
			panic!("Kernel has `nosmp` enabled; this disables support for parallel CPUs")
		}
		
		if self.0.maxcpus() == 0
		{
			panic!("Kernel has `maxcpus=0`; this disables support for parallel CPUs")
		}
		
		#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
		{
			match self.0.acpi()
			{
				Some("off") => panic!("Kernel has `acpi=off`"),
				
				_ => (),
			}
		}
		
		#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
		{
			if self.0.noapic()
			{
				panic!("Kernel has `noapic` enabled")
			}
			
			if self.0.disableapic()
			{
				panic!("Kernel has `disableapic` enabled")
			}
			
			if self.0.nolapic()
			{
				panic!("Kernel has `nolapic` enabled")
			}
			
			if self.0.noapictimer()
			{
				panic!("Kernel has `noapictimer` enabled")
			}
			
			if self.0.nospectre_v2()
			{
				panic!("Kernel has `nospectre_v2` enabled; this is wrong and also useless, as DPDK-based applications make very few syscalls")
			}
			
			if let Some(mitigation) = self.0.spectre_v2()
			{
				match mitigation
				{
					"on" | "auto" | "retpoline" | "retpoline,amd" => (),
					
					"retpoline,google" => warn!("Kernel has `spectre_v2=retpoline,google`; this is probably not the best choice"),
					
					"off" => panic!("Kernel spectre_v2 mitigation has been disabled; this is wrong and also useless, as DPDK-based applications make very few syscalls"),
					
					_ => panic!("Kernel spectre_v2 mitigation '{}' not recognised; double-check this is intended", mitigation),
				}
			}
			
			if let Some(pci_parameters) = self.0.pci()
			{
				if pci_parameters.contains("off")
				{
					panic!("Kernel has PCI disabled")
				}
				
				if pci_parameters.contains("noacpi")
				{
					panic!("Kernel has PCI noacpi")
				}
			}
			
			match self.0.numa()
			{
				None => (),
				
				Some("off", None) => panic!("Kernel should not have NUMA disabled; we need it to work correctly"),
				
				Some("noacpi", None) => panic!("Kernel should not have NUMA 'acpi' disabled; we need it to work correctly"),
				
				Some("fake", Some(_)) => panic!("Kernel should not have fake NUMA nodes; they do not have correctly assigned CPUs"),
				
				unrecognised @ _ => panic!("Unrecognised Kernel NUMA options '{:?}", unrecognised),
			}
			
			if self.0.nogbpages()
			{
				panic!("Kernel should not have `nogbpages`; on older systems, simply do not specify this flag")
			}
			
			if self.0.nohugeiomap()
			{
				panic!("Kernel has `nohugeiomap` enabled; this disables huge pages for IO")
			}
			
			if self.0.notsc()
			{
				panic!("Kernel has `notsc` enabled; this disables support for the Time Stamp Counter, TSC")
			}
			
			if self.0.nohpet()
			{
				panic!("Kernel has `nohpet` enabled; this disables support for the High Precision Event Timer, HPET")
			}
			
			if let Some(hpet_mmap_enabled) = self.0.hpet_mmap()
			{
				if !hpet_mmap_enabled
				{
					panic!("Kernel has `hpet_mmap=0`, ie hpet is disabled; this disables support for the High Precision Event Timer, HPET")
				}
			}
			
			if self.0.nopat()
			{
				panic!("Kernel has `nopat` enabled; this isn't useful")
			}
			
			if let Some(noexec_enabled) = self.0.noexec()
			{
				if !noexec_enabled
				{
					panic!("Kernel has `noexec=off`, ie non-executable mappings are disabled")
				}
			}
			
			if let Some(vdso_enabled) = self.0.vdso()
			{
				if !vdso_enabled
				{
					panic!("Kernel has `vdso=0`, ie vdso is disabled; this negatively impacts performance")
				}
			}
			
			if let Some(vdso32_enabled) = self.0.vdso32()
			{
				if !vdso32_enabled
				{
					panic!("Kernel has `vdso32=0`, ie vdso is disabled; this negatively impacts performance")
				}
			}
			
			if self.0.noinvpcid()
			{
				panic!("Kernel has `noinvpcid` enabled; this isn't useful")
			}
		}
		
		#[cfg(target_arch = "x86_64")]
		{
			if self.0.nopti()
			{
				panic!("Kernel has `nopti` enabled; this is wrong and also useless, as DPDK-based applications make very few syscalls")
			}
			
			if let Some(mitigation) = self.0.pti()
			{
				match mitigation
				{
					"on" | "auto" => (),
					
					"off" => panic!("Kernel Control Page Table Isolation (pti) mitigation has been disabled; this is wrong and also useless, as DPDK-based applications make very few syscalls"),
					
					_ => panic!("Kernel Control Page Table Isolation (pti) mitigation '{}' not recognised; double-check this is intended", mitigation),
				}
			}
			
			match self.0.vsyscall()
			{
				None => panic!("Kernel vsyscall mitigation should be disabled with `vsycall=none`"),
				
				Some("none") => (),
				
				Some("emulate") => panic!("Kernel vsyscall should be disabled with `vsycall=none` not `vsyscall=emulate`"),
				
				Some("native") => panic!("vKernel syscall mitigration has been disabled; this is wrong and also useless, as DPDK-based applications do not use vsyscall"),
				
				unknown @ _ => panic!("Kernel vsyscall mitigation '{}' not recognised; double-check this is intended", unknown),
			}
			
			if self.0.nopcid()
			{
				panic!("Kernel has `nopcid` enabled; this isn't useful")
			}
			
			match self.0.numa_balancing()
			{
				None | Some(true) => panic!("Kernel has NUMA balancing enabled"),
				_ => (),
			}
			
			if self.0.nox2apic()
			{
				panic!("Kernel has `nox2apic` enabled")
			}
			
			if let Some(noexec32_enabled) = self.0.noexec32()
			{
				if !noexec32_enabled
				{
					panic!("Kernel has `noexec32=off`, ie non-executable mappings are disabled")
				}
			}
			
			if cpu_supports_1gb_pages
			{
				if !self.0.gbpages()
				{
					panic!("Kernel should have `gbpages`")
				}
				
				if self.0.nogbpages()
				{
					panic!("Kernel should not have `nogbpages`")
				}
			}
		}
	}
	
	#[inline(always)]
	fn warnings(&self, warnings_to_suppress: &WarningsToSuppress)
	{
		match self.0.hashdist()
		{
			None => warnings_to_suppress.kernel_warn_without_check("hashdist", "Kernel should be booted with `hashdist=0` to disable NUMA hash distribution"),
			
			Some(false) => warnings_to_suppress.kernel_warn_without_check("hashdist", "Kernel should be booted with `hashdist=0` to disable NUMA hash distribution; it was booted with `hashdist=1`, which is the default and so pointless"),
			
			_ => (),
		}
		
		warnings_to_suppress.kernel_warn("noaliencache", "Kernel has `noaliencache` enabled; this is likely to hurt performance", || !self.0.noaliencache());
		
		warnings_to_suppress.kernel_warn("numa_zonelist_order", "Kernel has `noaliencache` enabled; this is likely to hurt performance", || self.0.numa_zonelist_order().is_none());
		
		match self.0.skew_tick()
		{
			None | Some(false) => warnings_to_suppress.kernel_warn_without_check("skew_tick", "Kernel should have `skew_tick=1` for maximum performance at the cost of power consumption"),
			Some(true) => (),
		}
		
		#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
		{
			match self.0.idle()
			{
				None => warnings_to_suppress.kernel_warn_without_check("idle_poll", "Kernel should be booted with `idle=poll` for maximum performance at the cost of power consumption"),
				
				Some(value) => match value
				{
					"idle" => (),
					
					"halt" | "nomwait" => warnings_to_suppress.kernel_warn_without_check("idle_poll", &format!("Kernel should be booted with `idle=poll` rather than `idle={}` for maximum performance at the cost of power consumption, although there may be impacts on hyper threading", value)),
					
					_ => (),
				},
			}
			
			warnings_to_suppress.kernel_warn("noxsaveopt", "Kernel has `noxsaveopt` enabled; this is likely to hurt performance", || !self.0.noxsaveopt());
		}
	}
}
