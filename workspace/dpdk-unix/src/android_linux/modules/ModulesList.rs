// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulesList(HashSet<String>);


impl ModulesList
{
	/// true if unloaded, false if does not exist
	pub fn unload(moduleName: &str) -> Result<bool, io::Error>
	{
		let name = CString::new(moduleName).unwrap();
		let flags = ::libc::O_NONBLOCK;
		
		match unsafe { syscall(SYS_delete_module as i64, name.as_ptr(), flags) }
		{
			0 => Ok(true),
			-1 => match errno().0
			{
				E::EPERM => Err(io::Error::new(ErrorKind::PermissionDenied, "permission denied")),
				E::EBUSY => Err(io::Error::new(ErrorKind::PermissionDenied, "busy")),
				E::ENOENT => Ok(false),
				E::EWOULDBLOCK => Err(io::Error::new(ErrorKind::PermissionDenied, "In use")),
				
				E::EFAULT => panic!("EFAULT should not occur"),
				
				unknown @ _ => panic!("syscall delete_module failed with illegal error code '{}'", unknown),
			},
			illegal @ _ => panic!("syscall(SYS_finit_module) returned illegal value '{}'", illegal),
		}
	}
	
	// Ought to end '.ko'
	pub fn loadModuleFromKoFile(modulePath: &Path) -> Result<bool, io::Error>
	{
		let file = try!(OpenOptions::new().read(true).open(modulePath));
		let fileDescriptor = file.as_raw_fd();
		
		let options = CString::new("").unwrap();
		let flags = 0;
		
		match unsafe { syscall(SYS_finit_module as i64, fileDescriptor, options.as_ptr(), flags) }
		{
			0 => Ok(true),
			-1 => match errno().0
			{
				E::EPERM => Err(io::Error::new(ErrorKind::PermissionDenied, "permission denied")),
				unknown @ _ => Err(io::Error::new(ErrorKind::Other, format!("Error Code was '{}'", unknown))),
			},
			illegal @ _ => panic!("syscall(SYS_finit_module) returned illegal value '{}'", illegal),
		}
	}
	
	/// true if loaded
	pub fn loadModuleIfAbsentFromKoFile(&self, moduleName: &str, moduleFileBaseName: &str, modulesPath: &Path) -> Result<bool, io::Error>
	{
		if self.hasModule(moduleName)
		{
			Ok(false)
		}
		else
		{
			let mut modulePath = PathBuf::from(modulesPath);
			modulePath.push(format!("{}.ko", moduleFileBaseName));
			Self::loadModuleFromKoFile(&modulePath)
		}
	}
	
	pub fn loadModuleIfAbsent(&self, moduleName: &str, moduleFileBaseName: &str) -> Result<(), ModProbeError>
	{
		if self.hasModule(moduleName)
		{
			Ok(())
		}
		else
		{
			modprobe(moduleFileBaseName)
		}
	}
	
	pub fn hasAfPacket(&self) -> bool
	{
		self.hasModule("af_packet")
	}
	
	pub fn hasVirtioNet(&self) -> bool
	{
		self.hasModule("virtio_net")
	}
	
	// insmod /path/to/rte_kni.ko - not a regular linux module
	pub fn hasKni(&self) -> bool
	{
		self.hasModule("rte_kni")
	}
	
	// /lib/modules/4.4.23-0-grsec/kernel/drivers/vfio/pci/vfio-pci.ko
	// modprobe -- vfio-pci
	// chmod a+x /dev/vfio  (all users can read folder contents)
	// exists /dev/vfio/vfio
	pub fn hasVfioPci(&self) -> bool
	{
		self.hasModule("vfio_pci")
	}
	
	pub fn hasModule(&self, moduleName: &str) -> bool
	{
		self.0.contains(moduleName)
	}
	
	pub fn parse(procPath: &Path) -> Result<Self, ModulesListParseError>
	{
		let mut modulesFilePath = PathBuf::from(procPath);
		modulesFilePath.push("modules");
		
		let openFile = try!(File::open(modulesFilePath));
		let mut reader = BufReader::with_capacity(4096, openFile);
		
		let mut modulesList = HashSet::new();
		let mut lineCount = 0;
		let mut line = String::with_capacity(512);
		while try!(reader.read_line(&mut line)) > 0
		{
			{
				let mut split = line.splitn(2, ' ');

				let moduleName = split.next().unwrap();
				
				if moduleName.is_empty()
				{
					return Err(ModulesListParseError::CouldNotParseEmptyModuleName(lineCount))
				}
				
				let isOriginal = modulesList.insert(moduleName.to_owned());
				if !isOriginal
				{
					 return Err(ModulesListParseError::DuplicateModuleName(lineCount, moduleName.to_owned()));
				}
			}
			
			line.clear();
			lineCount += 1;
		}
		
		Ok(ModulesList(modulesList))
	}
}
