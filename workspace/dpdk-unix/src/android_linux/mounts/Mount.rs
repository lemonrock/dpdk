// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a mount.
#[derive(Debug)]
pub struct Mount
{
	/// eg eg `/dev/sda1`, `proc`, etc; not really that useful.
	pub source: CString,
	
	/// Mount point.
	pub mount_point: PathBuf,
	
	/// File system type.
	///
	/// eg `proc`, `sysfs`, `hugetlbs`, `ext4`; listed in second column of `/proc/filesystems`/
	pub file_system_type: FileSystemType,
	
	/// Mount options.
	///
	/// eg `nodev mode=0177`
	pub mount_options: HashMap<String, Option<String>>,
	
	/// Typically `0` (zero).
	dump_frequency_in_days: i32,
	
	/// Typically `0` (zero).
	pass_number_on_parallel_filesystem_type: i32,
}

impl Mount
{
	//noinspection SpellCheckingInspection
	/// Unmounts.
	pub fn unmount(mount_point: &Path, unmount_flags: UnmountFlags) -> Result<(), io::Error>
	{
		use self::ErrorKind::*;
		
		let target = mount_point.to_c_string();
		match unsafe { umount2(target.as_ptr(), unmount_flags.bits()) }
		{
			0 => Ok(()),
			
			-1 => match errno().0
			{
				E::EAGAIN =>
				{
					if unmount_flags.contains(UnmountFlags::Expire)
					{
						Ok(())
					}
					else
					{
						panic!("umount() set an illegal errno of EAGAIN when unmount flags did not contain MNT_EXPIRE");
					}
				},
				E::EBUSY => Err(io::Error::new(TimedOut, "Busy")),
				E::EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),
				
				E::ENOENT => Err(io::Error::new(NotFound, "Mount path had an empty or non-existent component")),
				E::EINVAL => Err(io::Error::new(InvalidData, "One of many possible failures (EINVAL)")),
				E::ENOMEM => panic!("Out of memory (ENOMEM)"),
				E::ENAMETOOLONG => panic!("mount_point path name is too long"),
				E::EFAULT => panic!("Invalid data"),
				
				illegal @ _ => panic!("umount() set an illegal errno '{}'", illegal),
			},
			
			illegal @ _ => panic!("umount() returned an illegal result '{}'", illegal),
		}
	}
	
	/// Does this mount have this file system type?
	#[inline(always)]
	pub fn has_file_system_type(&self, file_system_type: &FileSystemType) -> bool
	{
		&self.file_system_type == file_system_type
	}
	
	//noinspection SpellCheckingInspection
	fn from_mntent(raw: *mut mntent) -> Self
	{
		debug_assert!(!unsafe {(*raw).mnt_fsname }.is_null(), "null");
		let source = unsafe { CStr::from_ptr((*raw).mnt_fsname) }.to_owned();
		let mount_point = (unsafe { c_string_pointer_to_path_buf((*raw).mnt_dir) }).expect("mnt_dir was empty").expect("mnt_dir was null");
		let file_system_type =
		{
			let string = unsafe { c_string_pointer_to_string_with_replacements_if_any((*raw).mnt_type) };
			
			FileSystemType::from_string(string.expect("mnt_type was null"))
		};
		
		let mount_options_string = (unsafe { c_string_pointer_to_string_with_replacements_if_any((*raw).mnt_opts) }).expect("mnt_opts was null");
		
		let mut mount_options = HashMap::with_capacity(16);
		for mount_option_string in mount_options_string.split(',')
		{
			let mut split = mount_option_string.splitn(2, '=');
			let name = split.next().unwrap().to_owned();
			let value_if_any = split.next().map(|value| value.to_owned());
			assert!(mount_options.insert(name, value_if_any).is_none(), "Duplicate key in mount options for mount_point '{:?}'", mount_point);
		}
		
		Self
		{
			source,
			mount_point,
			file_system_type,
			mount_options,
			dump_frequency_in_days: unsafe { (*raw).mnt_freq },
			pass_number_on_parallel_filesystem_type: unsafe { (*raw).mnt_passno },
		}
	}
	
	/// Mount a huge page file system.
	pub fn mount_huge_pages(huge_page_mount_settings: &HugePageMountSettings, override_default_huge_page_size: Option<HugePageSize>) -> Result<PathBuf, io::Error>
	{
		let mount_flags = huge_page_mount_settings.mount_flags;
		match Self::new_where_source_is_file_system_type(huge_page_mount_settings.mount_point.clone(), FileSystemType::hugetlbfs, huge_page_mount_settings.as_mount_options(override_default_huge_page_size)).mount(mount_flags)
		{
			Ok(()) => Ok(huge_page_mount_settings.mount_point.clone()),
			Err(error) => Err(error),
		}
	}
	
	/// New instance for file systems which do not have a source (eg `hugetlbs`).
	pub fn new_where_source_is_file_system_type(mount_point: PathBuf, file_system_type: FileSystemType, mount_options: HashMap<String, Option<String>>) -> Self
	{
		Self
		{
			source: file_system_type.to_c_string(),
			mount_point,
			file_system_type,
			mount_options,
			dump_frequency_in_days: 0,
			pass_number_on_parallel_filesystem_type: 0,
		}
	}
	
	//noinspection SpellCheckingInspection
	/// Mount.
	pub fn mount(&self, mount_flags: MountFlags) -> Result<(), io::Error>
	{
		use self::ErrorKind::*;
	
		fn to_mount_options_c_string(mount_options: &HashMap<String, Option<String>>) -> CString
		{
			let mut mount_options_string = String::with_capacity(64);
			let mut after_first = false;
			for (name, value_if_any) in mount_options
			{
				if after_first
				{
					mount_options_string.push(',');
				}
				else
				{
					after_first = true;
				}
				mount_options_string.push_str(name);
				if let Some(ref value) = *value_if_any
				{
					mount_options_string.push('=');
					mount_options_string.push_str(value);
				}
			}
			CString::new(mount_options_string).expect("mount_options should not contain interior ASCII NULs")
		}
		
		let target = self.mount_point.to_c_string();
		let file_system_type = self.file_system_type.to_c_string();
		let data = to_mount_options_c_string(&self.mount_options);
		
		match unsafe { mount(self.source.as_ptr(), target.as_ptr(), file_system_type.as_ptr(), mount_flags.bits(), data.as_ptr() as *mut c_void) }
		{
			0 => Ok(()),
			
			-1 => match errno().0
			{
				E::EACCES => Err(io::Error::new(NotFound, "Component of mount path to mount does not exist")),
				E::ENOENT => Err(io::Error::new(NotFound, "Mount path had an empty or non-existent component")),
				E::ENOTDIR => Err(io::Error::new(NotFound, "target or source is not a directory")),
				E::ELOOP => Err(io::Error::new(NotFound, "Loops - target is a descendant of source, or too many links in mount path")),
				E::EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),
				E::EBUSY => Err(io::Error::new(TimedOut, "Busy")),
				E::EINVAL => Err(io::Error::new(InvalidData, "One of many possible failures (EINVAL)")),
				
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
}
