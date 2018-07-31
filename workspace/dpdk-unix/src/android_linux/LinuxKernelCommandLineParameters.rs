// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Linux kernel command line parameters.
///
/// Some of these are interpreted as environment variables set just as `init` is executed (typically in upper case, eg `TERM=vt100`). Others are not claimed by the kernel, but by `init`, eg `single`.
///
/// A very small number can occur more than once, eg `console`.
///
/// See <https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html> and <https://www.kernel.org/doc/Documentation/x86/x86_64/boot-options.txt>.
#[derive(Default, Debug, Clone)]
pub struct LinuxKernelCommandLineParameters(HashMap<String, Vec<Option<String>>>);

impl LinuxKernelCommandLineParameters
{
	/// `numa_balancing`.
	#[inline(always)]
	pub fn numa_balancing(&self) -> Option<bool>
	{
		self.get_value("numa_balancing").map(|value|
		{
			match value
			{
				"enable" => true,
				"disable" => false,
				_ => panic!("numa_balancing '{}' is unrecognised", value)
			}
		})
	}
	
	/// `noexec`.
	#[inline(always)]
	pub fn noexec(&self) -> Option<bool>
	{
		self.get_value("noexec").map(|value|
		{
			match value
			{
				"on" => true,
				"off" => false,
				_ => panic!("noexec '{}' is unrecognised", value)
			}
		})
	}
	
	/// `noexec32`.
	#[inline(always)]
	pub fn noexec32(&self) -> Option<bool>
	{
		self.get_value("noexec32").map(|value|
		{
			match value
			{
				"on" => true,
				"off" => false,
				_ => panic!("noexec32 '{}' is unrecognised", value)
			}
		})
	}
	
	/// `skew_tick`.
	#[inline(always)]
	pub fn skew_tick(&self) -> Option<bool>
	{
		self.get_value("skew_tick").map(|value|
		{
			match value
			{
				"0" => false,
				"1" => true,
				_ => panic!("skew_tick '{}' is unrecognised", value)
			}
		})
	}
	
	/// `hpet_mmap`.
	#[inline(always)]
	pub fn hpet_mmap(&self) -> Option<bool>
	{
		self.get_value("hpet_mmap").map(|value|
		{
			match value
			{
				"0" => false,
				"1" => true,
				_ => panic!("hpet_mmap '{}' is unrecognised", value)
			}
		})
	}
	
	/// `vdso`.
	#[inline(always)]
	pub fn vdso(&self) -> Option<bool>
	{
		self.get_value("vdso").map(|value|
		{
			match value
			{
				"0" => false,
				"1" => true,
				_ => panic!("vdso '{}' is unrecognised", value)
			}
		})
	}
	
	/// `vdso32`.
	#[inline(always)]
	pub fn vdso32(&self) -> Option<bool>
	{
		self.get_value("vdso32").map(|value|
		{
			match value
			{
				"0" => false,
				"1" => true,
				"2" => false,
				_ => panic!("vdso32 '{}' is unrecognised", value)
			}
		})
	}
	
	/// CPUs isolated from the Linux scheduler.
	///
	/// eg "0-9,10-20:2/5" and "nohz,domain,0-9,10-20:2/5". Note in the latter example there isn't a separate delimiter separating the flags from the list, so one either has to be know all possible flags (unlikely and subject to change) or have some trully revolting parsing code, which is what we do below (`Self::split_flags_and_cpu_list`). For extra brownie points the Linux kernel treats the values as ASCII not UTF-8.
	#[inline(always)]
	pub fn isolcpus(&self) -> Option<(HashSet<&str>, BTreeSet<u16>)>
	{
		self.get_value("isolcpus").map(|value|
		{
			let (flags_to_split, cpu_list) = Self::split_flags_and_cpu_list(value);
			
			let mut flags = HashSet::with_capacity(2);
			match flags_to_split
			{
				None =>
				{
					flags.insert("domain");
				}
				
				Some(flags_to_split) =>
				{
					for flag in flags_to_split.split(',')
					{
						flags.insert(flag);
					}
				}
			}
			
			(flags, Self::parse_cpu_list(cpu_list))
		})
	}
	
	#[inline(always)]
	fn split_flags_and_cpu_list(value: &str) -> (Option<&str>, &str)
	{
		#[inline(always)]
		fn index_of_split_between_flags_and_cpu_list(value: &str) -> Option<usize>
		{
			#[inline(always)]
			fn is_ascii_alpha(character: char) -> bool
			{
				character.is_ascii_uppercase() || character.is_ascii_lowercase()
			}
			
			let mut index = 0;
			let mut previous_previous_character = '\0';
			let mut previous_character = '\0';
			for character in value.chars()
			{
				if character.is_ascii_digit() && previous_character == ',' && is_ascii_alpha(previous_previous_character)
				{
					return Some(index)
				}
				previous_previous_character = previous_character;
				previous_character = character;
				index += 1;
			}
			None
		}
		
		match index_of_split_between_flags_and_cpu_list(value)
		{
			None =>
			{
				let flags = None;
				let list = value;
				
				(flags, list)
			}
			
			Some(index_after_comma) =>
			{
				let split = value.split_at(index_after_comma);
				let flags = Some(&split.0[ .. (split.0.len() - 1)]);
				let list = split.1;
				
				(flags, list)
			}
		}
	}
	
	#[inline(always)]
	fn parse_cpu_list(list: &str) -> BTreeSet<u16>
	{
		ListParseError::parse_linux_list_string(list, |value| value).unwrap()
	}
	
	/// CPUs isolated from the Linux scheduler.
	///
	/// Ordinarily should match `isolcpus`.
	#[inline(always)]
	pub fn rcu_nocbs(&self) -> Option<BTreeSet<u16>>
	{
		self.get_value("rcu_nocbs").map(Self::parse_cpu_list)
	}
	
	/// CPUs isolated from the Linux scheduler.
	///
	/// Ordinarily should match `rcu_nocbs`.
	#[inline(always)]
	pub fn nohz_full(&self) -> Option<BTreeSet<u16>>
	{
		self.get_value("nohz_full").map(Self::parse_cpu_list)
	}
	
	/// CPUs in the default IRQ affinity mask.
	///
	/// This should probably not be set.
	#[inline(always)]
	pub fn irqaffinity(&self) -> Option<BTreeSet<u16>>
	{
		self.get_value("irqaffinity").map(Self::parse_cpu_list)
	}
	
	/// `nosmp`.
	///
	/// Disables SMP support.
	///
	/// Rarely used in practice.
	///
	/// S/390 equivalent is `nosmt`.
	#[inline(always)]
	pub fn nosmp(&self) -> bool
	{
		self.is_present_with_no_value("nosmp")
	}
	
	/// `maxcpus`.
	///
	/// Limits the Linux kernel to making only this number of CPUs online at boot-time; CPUs can be brought online later.
	///
	/// May also be `0` (zero) in which case it behaves the same as `nosmp`.
	///
	/// Rarely used in practice.
	///
	/// S/390 equivalent is `smt` (altohugh the zero value is not supported in this case).
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn maxcpus(&self) -> Option<u16>
	{
		self.get_value("maxcpus").map(|value| value.parse().unwrap())
	}
	
	/// `nr_cpus`.
	///
	/// Limits the Linux kernel to a maximum of this number of CPUs (expressed as `HyperThread::kernel_maximum_cpu() + 1`).
	///
	/// Rarely used in practice, except to increase a compile time limit.
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn nr_cpus(&self) -> Option<u16>
	{
		self.get_value("nr_cpus").map(|value| value.parse().unwrap())
	}
	
	/// `possible_cpus`.
	///
	/// Limits the Linux kernel to a maximum of this number of CPUs (see `HyperThread::possible()`).
	///
	/// Rarely used in practice.
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn possible_cpus(&self) -> Option<u16>
	{
		self.get_value("possible_cpus").map(|value| value.parse().unwrap())
	}
	
	/// NUMA `hashdist`.
	#[inline(always)]
	pub fn hashdist(&self) -> Option<bool>
	{
		self.get_value("hashdist").map(|value|
		{
			match value
			{
				"0" => false,
				"1" => true,
				_ => panic!("Unknown hashdist value '{}'", value),
			}
		})
	}
	
	/// `pci`.
	#[inline(always)]
	pub fn pci(&self) -> Option<HashSet<&str>>
	{
		self.get_value("pci").map(|value|
		{
			value.split(',').collect()
		})
	}
	
	/// `acpi`.
	///
	/// Do not confuse this with `noapic`, which is something different entirely.
	#[inline(always)]
	pub fn acpi(&self) -> Option<&str>
	{
		self.get_value("acpi")
	}
	
	/// `iommu`.
	#[inline(always)]
	pub fn iommu(&self) -> Option<&str>
	{
		self.get_value("iommu")
	}
	
	/// `intel_iommu`.
	#[inline(always)]
	pub fn intel_iommu(&self) -> Option<&str>
	{
		self.get_value("intel_iommu")
	}
	
	/// `numa_zonelist_order`.
	///
	/// Deprecated according to Linux source code.
	#[inline(always)]
	pub fn numa_zonelist_order(&self) -> Option<&str>
	{
		self.get_value("numa_zonelist_order")
	}
	
	/// `numa`.
	///
	/// Returns setting and optional fake size or number.
	///
	/// * `numa=off`
	/// * `numa=noacpi`
	/// * `numa=fake=SIZE` where size if suffixed with `M` or `G`
	/// * `numa=fake=N` where `N` is a number of fake NUMA nodes to create.
	///
	/// See <https://www.kernel.org/doc/Documentation/x86/x86_64/boot-options.txt>.
	#[inline(always)]
	pub fn numa(&self) -> Option<(&str, Option<&str>)>
	{
		self.get_value("numa").map(|value|
		{
			let mut split = value.splitn(2, '=');
			(split.next().unwrap(), split.last())
		})
	}
	
	/// `idle`.
	///
	/// See <https://www.kernel.org/doc/Documentation/x86/x86_64/boot-options.txt>.
	#[inline(always)]
	pub fn idle(&self) -> Option<&str>
	{
		self.get_value("idle")
	}
	
	/// `nopcid`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nopcid(&self) -> bool
	{
		self.is_present_with_no_value("nopcid")
	}
	
	/// `noinvpcid`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn noinvpcid(&self) -> bool
	{
		self.is_present_with_no_value("noinvpcid")
	}
	
	/// `norandmaps`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn norandmaps(&self) -> bool
	{
		self.is_present_with_no_value("norandmaps")
	}
	
	/// `noapic`.
	///
	/// Note the confusion with `acpi`, which is something else entirely.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn noapic(&self) -> bool
	{
		self.is_present_with_no_value("noapic")
	}
	
	/// `disableapic`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn disableapic(&self) -> bool
	{
		self.is_present_with_no_value("disableapic")
	}
	
	/// `nolapic`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nolapic(&self) -> bool
	{
		self.is_present_with_no_value("nolapic")
	}
	
	/// `noapictimer`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn noapictimer(&self) -> bool
	{
		self.is_present_with_no_value("noapictimer")
	}
	
	/// `nox2apic`.
	///
	/// Note the confusion with `acpi`, which is something else entirely.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nox2apic(&self) -> bool
	{
		self.is_present_with_no_value("nox2apic")
	}
	
	/// `nopat`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nopat(&self) -> bool
	{
		self.is_present_with_no_value("nopat")
	}
	
	/// `noxsaveopt`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn noxsaveopt(&self) -> bool
	{
		self.is_present_with_no_value("noxsaveopt")
	}
	
	/// NUMA `noaliencache`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn noaliencache(&self) -> bool
	{
		self.is_present_with_no_value("noaliencache")
	}
	
	/// NUMA `movable_node`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn movable_node(&self) -> bool
	{
		self.is_present_with_no_value("movable_node")
	}
	
	/// `nokaslr`.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nokaslr(&self) -> bool
	{
		self.is_present_with_no_value("nokaslr")
	}
	
	/// `nospectre_v2`.
	///
	/// Stupid command line option so that benchmarkers can game results and idiots 'apparently' increase performance.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nospectre_v2(&self) -> bool
	{
		self.is_present_with_no_value("nospectre_v2")
	}
	
	/// `spectre_v2`.
	#[inline(always)]
	pub fn spectre_v2(&self) -> Option<&str>
	{
		self.get_value("spectre_v2")
	}
	
	/// `nopti`.
	///
	/// Stupid command line option so that benchmarkers can game results and idiots 'apparently' increase performance.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nopti(&self) -> bool
	{
		self.is_present_with_no_value("nopti")
	}
	
	/// `pti`.
	#[inline(always)]
	pub fn pti(&self) -> Option<&str>
	{
		self.get_value("pti")
	}
	
	/// `vsyscall`.
	#[inline(always)]
	pub fn vsyscall(&self) -> Option<&str>
	{
		self.get_value("vsyscall")
	}
	
	/// `nohugeiomap`.
	///
	/// Disables Huge Pages for IO support.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nohugeiomap(&self) -> bool
	{
		self.is_present_with_no_value("nohugeiomap")
	}
	
	/// `notsc`.
	///
	/// Disables Time Stamp Counter (TSC) support.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn notsc(&self) -> bool
	{
		self.is_present_with_no_value("notsc")
	}
	
	/// `notsc`.
	///
	/// Disables High Precision Event Timer (HPET) support.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn nohpet(&self) -> bool
	{
		self.is_present_with_no_value("nohpet")
	}
	
	/// `panic` in seconds.
	///
	/// Rarely used in practice.
	#[inline(always)]
	pub fn panic(&self) -> Option<i64>
	{
		self.get_value("panic").map(|value| value.parse().unwrap())
	}
	
	/// Default `locale`.
	///
	/// Typically passed to `init`.
	#[inline(always)]
	pub fn locale(&self) -> Option<&str>
	{
		self.get_value("locale")
	}
	
	/// Parses `root`:-
	///
	/// * If it is of the form `root=/dev/sda`, returns `(None, "/dev/sda")`.
	/// * If it is of the form `root=UUID=59ca0b21-9bf6-4ccc-a06b-fdecc91bf2aa`, returns `(Some("UUID"), "59ca0b21-9bf6-4ccc-a06b-fdecc91bf2aa")`.
	#[inline(always)]
	pub fn root(&self) -> Option<(Option<&str>, &str)>
	{
		self.get_value("root").map(|root|
		{
			let mut key_value = root.splitn(2, '=');
			let key_or_value = key_value.next().unwrap();
			match key_value.next()
			{
				None => (None, key_or_value),
				Some(value) => (Some(key_or_value), value)
			}
		})
	}
	
	/// `default_hugepagesz`.
	#[inline(always)]
	pub fn default_hugepagesz(&self) -> Option<&str>
	{
		self.get_value("default_hugepagesz")
	}
	
	/// `hugepagesz`.
	#[inline(always)]
	pub fn hugepagesz(&self) -> Option<Vec<&str>>
	{
		self.get_values("hugepagesz")
	}
	
	/// `hugepages`.
	#[inline(always)]
	pub fn hugepages(&self) -> Option<Vec<&str>>
	{
		self.get_values("hugepages")
	}
	
	/// Detects if SELinux is enabled or disabled.
	#[inline(always)]
	pub fn selinux(&self) -> Option<bool>
	{
		self.get_value("selinux").map(|selinux|
		{
			match selinux
			{
				"0" => false,
				"1" => true,
				_ => panic!("Invalid value of selinux '{}'", selinux),
			}
		})
	}
	
	/// Parses `modules`.
	///
	/// Returns a list of modules.
	#[inline(always)]
	pub fn modules(&self) -> Option<Vec<&str>>
	{
		self.get_value("modules").map(|modules|
		{
			let mut set = Vec::new();
			for module in modules.split(',')
			{
				set.push(module)
			}
			set
		})
	}
	
	/// `nomodeset`.
	#[inline(always)]
	pub fn nomodeset(&self) -> bool
	{
		self.is_present_with_no_value("nomodeset")
	}
	
	/// `quiet`.
	#[inline(always)]
	pub fn quiet(&self) -> bool
	{
		self.is_present_with_no_value("quiet")
	}
	
	/// Single-user mode.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub fn S(&self) -> bool
	{
		self.is_present_with_no_value("S")
	}
	
	/// Single-user mode.
	///
	/// Not a kernel parameter, but passed through to init.
	#[inline(always)]
	pub fn single(&self) -> bool
	{
		self.is_present_with_no_value("single")
	}
	
	/// Kernel debugging is enabled.
	#[inline(always)]
	pub fn debug(&self) -> bool
	{
		self.is_present_with_no_value("debug")
	}
	
	/// Mount root file system read only, `ro`.
	#[inline(always)]
	pub fn ro(&self) -> bool
	{
		self.is_present_with_no_value("ro")
	}
	
	/// Mount root file system read write, `rw`.
	#[inline(always)]
	pub fn rw(&self) -> bool
	{
		self.is_present_with_no_value("rw")
	}
	
	/// `nogbpages`.
	///
	/// See <https://www.kernel.org/doc/Documentation/x86/x86_64/boot-options.txt>.
	#[inline(always)]
	pub fn nogbpages(&self) -> bool
	{
		self.is_present_with_no_value("nogbpages")
	}
	
	/// `gbpages`.
	//	///
	//	/// See <https://www.kernel.org/doc/Documentation/x86/x86_64/boot-options.txt>.
	#[inline(always)]
	pub fn gbpages(&self) -> bool
	{
		self.is_present_with_no_value("gbpages")
	}
	
	/// `initrd`.
	///
	/// Returns eg `initramfs-hardened`.
	#[inline(always)]
	pub fn initrd(&self) -> Option<&str>
	{
		self.get_value("initrd")
	}
	
	/// `init`.
	///
	/// Returns eg `/sbin/init`.
	#[inline(always)]
	pub fn init(&self) -> Option<PathBuf>
	{
		self.get_value("init").map(|value| PathBuf::from(value))
	}
	
	/// `rootfstype`.
	///
	/// Returns eg `ext4`.
	#[inline(always)]
	pub fn rootfstype(&self) -> Option<FileSystemType>
	{
		self.get_value("rootfstype").map(|value| FileSystemType::from_str(value))
	}
	
	/// `console`.
	#[inline(always)]
	pub fn console(&self) -> Option<Vec<&str>>
	{
		self.get_values("console")
	}
	
	/// Is this 'boolean' parameter present?
	///
	/// Panics if present with a value.
	#[inline(always)]
	pub fn is_present_with_no_value<'a>(&self, parameter_name: &'a str) -> bool
	{
		match self.get(parameter_name)
		{
			None => false,
			Some(ref list) =>
			{
				debug_assert_ne!(list.len(), 0, "list has no elements");
				
				assert_eq!(list.len(), 1, "more than one value for parameter");
				assert_eq!(list[0], None, "present with a value");
				true
			}
		}
	}
	
	/// Gets the value of this parameter.
	///
	/// Panics if present without a value or if multiple values are present.
	#[inline(always)]
	pub fn get_value<'a>(&self, parameter_name: &'a str) -> Option<&str>
	{
		match self.get(parameter_name)
		{
			None => None,
			
			// &Vec<Option<String>>
			Some(list) =>
			{
				debug_assert_ne!(list.len(), 0, "list has no elements");
				assert_eq!(list.len(), 1, "more than one value for parameter");
				
				(unsafe { list.get_unchecked(0) }).as_ref().map(|value| value.as_str())
			}
		}
	}
	
	/// Gets the values of this parameter.
	#[inline(always)]
	pub fn get_values<'a>(&self, parameter_name: &'a str) -> Option<Vec<&str>>
	{
		match self.get(parameter_name)
		{
			None => None,
			Some(list) =>
			{
				debug_assert_ne!(list.len(), 0, "list has no elements");
				
				let mut strings = Vec::with_capacity(list.len());
				
				for index in 0 .. list.len()
				{
					strings.push((unsafe { list.get_unchecked(index) }).as_ref().map(|value| value.as_str()).unwrap())
				}
				
				Some(strings)
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse(file_path: &Path) -> Result<Self, io::Error>
	{
		let line_of_parameters = file_path.read_string_without_line_feed()?;
		
		let mut map = HashMap::with_capacity(32);
		
		for parameter in line_of_parameters.split(' ')
		{
			let mut key_value = parameter.splitn(2, '=');
			let key = key_value.next().expect("There is no key");
			if key.is_empty()
			{
				continue
			}
			let key = key.replace('-', "_");
			
			let entry = map.entry(key).or_insert_with(|| Vec::with_capacity(1));
			
			let raw_value = key_value.next();
			if raw_value.is_none()
			{
				entry.push(None);
				continue
			}
			
			let potentially_quoted = raw_value.unwrap();
			
			let value = if potentially_quoted.len() >= 2 && potentially_quoted.starts_with('"') && potentially_quoted.ends_with('"')
			{
				&potentially_quoted[1 .. (potentially_quoted.len() - 1)]
			}
			else
			{
				potentially_quoted
			};
			
			entry.push(Some(value.to_owned()));
		}
		
		// strictly speaking, should be a multi-map because of `console=tty1 console=hvc0`
		
		map.shrink_to_fit();
		
		Ok(LinuxKernelCommandLineParameters(map))
	}
	
	#[inline(always)]
	fn get<'a>(&self, parameter_name: &'a str) -> Option<&Vec<Option<String>>>
	{
		self.0.get(parameter_name)
	}
}
