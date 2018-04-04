// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Mount
{
	pub nameOfMountedFileSystem: PathBuf, // eg eg /dev/sda1, proc, etc; not really that useful
	pub mountPoint: PathBuf,
	pub fileSystemType: FileSystemType, // eg proc, sysfs, hugetlbs, ext; listed in second column of /proc/filesystems (tab separated)
	pub mountOptions: HashMap<String, Option<String>>, // eg nodev mode=0177
	pub dumpFrequencyInDays: i32,
	pub passNumberOnParallelFsck: i32,
}

impl Mount
{
	pub fn unmount(mountPoint: &Path, unmountFlags: UnmountFlags) -> Result<(), io::Error>
	{
		let target = pathToCString(mountPoint);
		match unsafe { ::libc::umount2(target.as_ptr(), unmountFlags.bits()) }
		{
			0 => Ok(()),
			
			-1 => match errno().0
			{
				E::EAGAIN =>
				{
					if unmountFlags.contains(UnmountFlags::Expire)
					{
						Ok(())
					}
					else
					{
						panic!("umount() set an illegal errno of EAGAIN when unmount flags did not contain MNT_EXPIRE");
					}
				},
				E::EBUSY => Err(io::Error::new(ErrorKind::TimedOut, "Busy")),
				E::EPERM => Err(io::Error::new(ErrorKind::PermissionDenied, "permission denied")),
				
				E::ENOENT => Err(io::Error::new(ErrorKind::NotFound, "Mount path had an empty or non-existent component")),
				E::EINVAL => Err(io::Error::new(ErrorKind::InvalidData, "One of many possible failures (EINVAL)")),
				E::ENOMEM => panic!("Out of memory (ENOMEM)"),
				E::ENAMETOOLONG => panic!("mountPoint path name is too long"),
				E::EFAULT => panic!("Invalid data"),
				
				illegal @ _ => panic!("umount() set an illegal errno '{}'", illegal),
			},
			
			illegal @ _ => panic!("umount() returned an illegal result '{}'", illegal),
		}
	}
	
	pub fn isFileSystemType(&self, fileSystemType: &FileSystemType) -> bool
	{
		&self.fileSystemType == fileSystemType
	}
	
	pub fn from_mntent(raw: *mut mntent) -> Self
	{
		let nameOfMountedFileSystem = (unsafe { cStringPointerToPathBuf((*raw).mnt_fsname) }).expect("mnt_fsname was empty").expect("mnt_fsname was null");
		let mountPoint = (unsafe { cStringPointerToPathBuf((*raw).mnt_dir) }).expect("mnt_dir was empty").expect("mnt_dir was null");
		let fileSystemType =
		{
			let string = unsafe { cStringPointerToStringWithReplacementsIfAny((*raw).mnt_type) };
			
			FileSystemType::from_String(string.expect("mnt_type was null"))
		};
		
		let mountOptionsString = (unsafe { cStringPointerToStringWithReplacementsIfAny((*raw).mnt_opts) }).expect("mnt_opts was null");
		
		let mut mountOptions = HashMap::with_capacity(16);
		for mountOptionString in mountOptionsString.split(',')
		{
			let mut split = mountOptionString.splitn(2, '=');
			let name = split.next().unwrap().to_owned();
			let valueIfAny = split.next().map(|value| value.to_owned());
			assert!(mountOptions.insert(name, valueIfAny).is_none(), "Duplicate key in mount options for mount '{:?}'", mountPoint);
		}
		
		Mount
		{
			nameOfMountedFileSystem: nameOfMountedFileSystem,
			mountPoint: mountPoint,
			fileSystemType: fileSystemType,
			mountOptions: mountOptions,
			dumpFrequencyInDays: unsafe { (*raw).mnt_freq },
			passNumberOnParallelFsck: unsafe { (*raw).mnt_passno },
		}
	}
	
	pub fn toMountOptionsString(mountOptions: &HashMap<String, Option<String>>) -> CString
	{
		let mut mountOptionsString = String::with_capacity(64);
		let mut afterFirst = false;
		for (name, valueIfAny) in mountOptions
		{
			if afterFirst
			{
				mountOptionsString.push(',');
			}
			else
			{
				afterFirst = true;
			}
			mountOptionsString.push_str(name);
			if let Some(ref value) = *valueIfAny
			{
				mountOptionsString.push('=');
				mountOptionsString.push_str(value);
			}
		}
		CString::new(mountOptionsString).expect("mountOptions should not contain interior ASCII NULs")
	}
	
	pub fn mountHugePages(hugePageMountSettings: &HugePageMountSettings, overrideDefaultHugePageSize: Option<HugePageSize>) -> Result<PathBuf, io::Error>
	{
		let mountFlags = hugePageMountSettings.mountFlags();
		match Self::mountFileSystemWhereSourceIsSameAsFileSystemType(&hugePageMountSettings.mountPoint, &FileSystemType::hugetlbfs, &hugePageMountSettings.asMountOptions(overrideDefaultHugePageSize), mountFlags)
		{
			Ok(()) => Ok(hugePageMountSettings.mountPoint.clone()),
			Err(error) => Err(error),
		}
	}
	
	pub fn mountFileSystemWhereSourceIsSameAsFileSystemType(mountPoint: &Path, fileSystemType: &FileSystemType, mountOptions: &HashMap<String, Option<String>>, mountFlags: MountFlags) -> Result<(), io::Error>
	{
		let source = fileSystemType.to_CString();
		Self::mountFileSystem(&source, mountPoint, fileSystemType, mountOptions, mountFlags)
	}
	
	pub fn mount(&self, mountFlags: MountFlags) -> Result<(), io::Error>
	{
		let source = pathToCString(&self.nameOfMountedFileSystem);
		Self::mountFileSystem(&source, &self.mountPoint, &self.fileSystemType, &self.mountOptions, mountFlags)
	}
	
	pub fn mountFileSystem(source: &CStr, mountPoint: &Path, fileSystemType: &FileSystemType, mountOptions: &HashMap<String, Option<String>>, mountFlags: MountFlags) -> Result<(), io::Error>
	{
		let target = pathToCString(mountPoint);
		let fileSystemType = fileSystemType.to_CString();
		let data = Self::toMountOptionsString(mountOptions);
		
		match unsafe { ::libc::mount(source.as_ptr(), target.as_ptr(), fileSystemType.as_ptr(), mountFlags.bits(), data.as_ptr() as *mut c_void) }
		{
			0 => Ok(()),
			
			-1 => match errno().0
			{
				E::EACCES => Err(io::Error::new(ErrorKind::NotFound, "Component of mount path to mount does not exist")),
				E::ENOENT => Err(io::Error::new(ErrorKind::NotFound, "Mount path had an empty or non-existent component")),
				E::ENOTDIR => Err(io::Error::new(ErrorKind::NotFound, "target or source is not a directory")),
				E::ELOOP => Err(io::Error::new(ErrorKind::NotFound, "Loops - target is a descendent of source, or too many links in mount path")),
				E::EPERM => Err(io::Error::new(ErrorKind::PermissionDenied, "permission denied")),
				E::EBUSY => Err(io::Error::new(ErrorKind::TimedOut, "Busy")),
				E::EINVAL => Err(io::Error::new(ErrorKind::InvalidData, "One of many possible failures (EINVAL)")),
			
				E::EMFILE => panic!("Out of memory (EMFILE)"),
				E::ENOMEM => panic!("Out of memory (ENOMEM)"),
				E::ENODEV => panic!("File system type not supported by Linux Kernel (check /proc/filesystem first)"),
				E::ENOTBLK => panic!("Specified block device wasn't"),
				E::ENXIO => panic!("Block device major number is out of range"),
				E::ENAMETOOLONG => panic!("Mount path name is too long"),
				E::EFAULT => panic!("Invalid data"),
				
				illegal @ _ => panic!("mount() set an illegal errno '{}'", illegal),
			},
			
			illegal @ _ => panic!("mount() returned an illegal result '{}'", illegal),
		}
	}
	
	pub fn add(&self, mounts: Mounts) -> Result<(), io::Error>
	{
		if mounts.readOnly()
		{
			return Err(io::Error::new(ErrorKind::PermissionDenied, "mounts has been opened read-only"));
		}
		
		let mnt_fsname = pathToCString(&self.nameOfMountedFileSystem);
		let mnt_dir = pathToCString(&self.mountPoint);
		let mnt_type = self.fileSystemType.to_CString();
		let mnt_opts = Self::toMountOptionsString(&self.mountOptions);
		
		let mut mount = mntent
		{
			mnt_fsname: mnt_fsname.as_ptr() as *mut _,
			mnt_dir: mnt_dir.as_ptr() as *mut _,
			mnt_type: mnt_type.as_ptr() as *mut _,
			mnt_opts: mnt_opts.as_ptr() as *mut _,
			mnt_freq: self.dumpFrequencyInDays,
			mnt_passno: self.passNumberOnParallelFsck,
		};

		match unsafe { addmntent(mounts.0, &mut mount) }
		{
			0 => Ok(()),
			1 => Err(io::Error::new(ErrorKind::Other, "mount could not be added by addmntent() (addmntent does not supply any indication as to why)")),
			
			illegal @ _ => panic!("addmntent() returned an illegal result '{}'", illegal),
		}
	}
}
