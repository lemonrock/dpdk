// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A list of Linux kernel modules loaded on the system.
///
/// Is *not* updated if a module is loaded or unloaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxKernelModulesList(HashSet<Box<[u8]>>);

impl LinuxKernelModulesList
{
	/// Unloads a Linux kernel module.
	///
	/// Does not use `modprobe`.
	///
	/// true if unloaded.
	/// false if does not exist.
	pub fn unload_linux_kernel_module(linux_kernel_module_name: &[u8]) -> Result<bool, io::Error>
	{
		use self::ErrorKind::*;
		
		let name = CString::new(linux_kernel_module_name).unwrap();
		let flags = ::libc::O_NONBLOCK;
		
		match unsafe { syscall(SYS_delete_module as i64, name.as_ptr(), flags) }
		{
			0 => Ok(true),
			-1 => match errno().0
			{
				E::EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),
				E::EBUSY => Err(io::Error::new(PermissionDenied, "busy")),
				E::ENOENT => Ok(false),
				E::EWOULDBLOCK => Err(io::Error::new(PermissionDenied, "In use")),
				
				E::EFAULT => panic!("EFAULT should not occur"),
				
				unknown @ _ => panic!("syscall delete_module failed with illegal error code '{}'", unknown),
			},
			illegal @ _ => panic!("syscall(SYS_finit_module) returned illegal value '{}'", illegal),
		}
	}
	
	/// Loads a Linux Kernel Module.
	///
	/// Does not use `modprobe`.
	///
	/// Returns true if loaded.
	/// Returns false if permissions error occurred (eg was not root).
	///
	/// `linux_kernel_module_path` normally ends in a '.ko' file extension, but this is not enforced.
	pub fn load_linux_kernel_module_from_ko_file(linux_kernel_module_path: &Path) -> Result<bool, io::Error>
	{
		use self::ErrorKind::*;
		
		let file = OpenOptions::new().read(true).open(linux_kernel_module_path)?;
		let file_descriptor = file.as_raw_fd();
		
		let options = CString::new("").unwrap();
		let flags = 0;
		
		match unsafe { syscall(SYS_finit_module as i64, file_descriptor, options.as_ptr(), flags) }
		{
			0 => Ok(true),
			-1 => match errno().0
			{
				E::EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),
				unknown @ _ => Err(io::Error::new(Other, format!("Error Code was '{}'", unknown))),
			},
			illegal @ _ => panic!("syscall(SYS_finit_module) returned illegal value '{}'", illegal),
		}
	}
	
	
	/// Loads a Linux Kernel Module.
	///
	/// `module_file_base_name` excludes the `.ko` file extension.
	///
	/// Does not use `modprobe`.
	///
	/// Returns true if loaded.
	/// Returns false if already loaded.
	///
	/// Updates the list of loaded modules.
	pub fn load_linux_kernel_module_if_absent_from_ko_file(&mut self, linux_kernel_module_name: &[u8], module_file_base_name: &str, linux_kernel_modules_path: &Path) -> Result<bool, io::Error>
	{
		if self.is_linux_kernel_module_is_loaded(linux_kernel_module_name)
		{
			Ok(false)
		}
		else
		{
			let mut linux_kernel_module_path = PathBuf::from(linux_kernel_modules_path);
			linux_kernel_module_path.push(format!("{}.ko", module_file_base_name));
			let loaded = Self::load_linux_kernel_module_from_ko_file(&linux_kernel_module_path)?;
			self.0.insert(linux_kernel_module_name.to_vec().into_boxed_slice());
			Ok(loaded)
		}
	}
	
	/// Loads a module if absent from the Kernel.
	///
	/// Uses `modprobe`.
	///
	/// Updates the list of loaded modules.
	pub fn load_linux_kernel_module_if_absent_using_modprobe(&mut self, linux_kernel_module_name: &[u8], module_file_base_name: &[u8]) -> Result<bool, ModProbeError>
	{
		if self.is_linux_kernel_module_is_loaded(linux_kernel_module_name)
		{
			Ok(false)
		}
		else
		{
			modprobe(module_file_base_name)?;
			self.0.insert(linux_kernel_module_name.to_vec().into_boxed_slice());
			Ok(true)
		}
	}
	
	/// Is the `linux_kernel_module_name` loaded?
	pub fn is_linux_kernel_module_is_loaded(&self, linux_kernel_module_name: &[u8]) -> bool
	{
		self.0.contains(linux_kernel_module_name)
	}
	
	/// Parses the list of loaded Linux Kernel modules.
	pub(crate) fn parse(file_path: &Path) -> Result<Self, LinuxKernelModulesListParseError>
	{
		let reader = BufReader::with_capacity(4096, File::open(file_path)?);
		
		let mut modules_list = HashSet::new();
		let mut zero_based_line_number = 0;
		for line in reader.split(b'\n')
		{
			{
				let mut line = line?;
				let mut split = splitn(&line, 2, b' ');

				let linux_kernel_module_name = split.next().unwrap();
				
				if linux_kernel_module_name.is_empty()
				{
					return Err(LinuxKernelModulesListParseError::CouldNotParseEmptyModuleName { zero_based_line_number })
				}
				
				let is_original = modules_list.insert(linux_kernel_module_name.to_vec().into_boxed_slice());
				if !is_original
				{
					 return Err(LinuxKernelModulesListParseError::DuplicateModuleName { zero_based_line_number, linux_kernel_module_name: linux_kernel_module_name.to_vec().into_boxed_slice() });
				}
			}

			zero_based_line_number += 1;
		}
		
		Ok(LinuxKernelModulesList(modules_list))
	}
}
