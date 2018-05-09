// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//noinspection SpellCheckingInspection
//noinspection RustEnumVariantNaming
/// File system types.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileSystemType
{
	bdev,
	cgroup,
	cpuset,
	devpts,
	devtmpfs,
	ext2,
	ext3,
	ext4,
	hugetlbfs,
	mqueue,
	pipefs,
	pstore,
	ramfs,
	rootfs,
	security,
	sockfs,
	sysfs,
	tmpfs,
	_proc,

	anon_inodefs,
	binfmt_misc,
	debugfs,
	ecryptfs,
	fuse,
	fuseblk,
	fusectl,
	prl_fs,
	securityfs,
	vfat,

	Unrecognised(String)
}

impl FileSystemType
{
	//noinspection SpellCheckingInspection
	/// To `CString`.
	pub fn to_c_string(&self) -> CString
	{
		use self::FileSystemType::*;
		
		let ref_value = match *self
		{
			sysfs => "sysfs",
			rootfs => "rootfs",
			ramfs => "ramfs",
			bdev => "bdev",
			_proc => "_proc",
			cpuset => "cpuset",
			cgroup => "cgroup",
			tmpfs => "tmpfs",
			devtmpfs => "devtmpfs",
			security => "security",
			sockfs => "sockfs",
			pipefs => "pipefs",
			devpts => "devpts",
			hugetlbfs => "hugetlbfs",
			pstore => "pstore",
			mqueue => "mqueue",
			ext2 => "ext2",
			ext3 => "ext3",
			ext4 => "ext4",
			
			anon_inodefs => "anon_inodefs",
			binfmt_misc => "binfmt_misc",
			debugfs => "debugfs",
			ecryptfs => "ecryptfs",
			fuse => "fuse",
			fuseblk => "fuseblk",
			fusectl => "fusectl",
			prl_fs => "prl_fs",
			securityfs => "securityfs",
			vfat => "vfat",
			
			Unrecognised(ref value) => value,
		};
		
		CString::new(ref_value.to_owned()).expect("file system type should not contain interior ASCII NULs")
	}
	
	//noinspection SpellCheckingInspection
	/// From string.
	pub fn from_string(value: String) -> Self
	{
		use self::FileSystemType::*;
		
		match &value[..]
		{
			"sysfs" => sysfs,
			"rootfs" => rootfs,
			"ramfs" => ramfs,
			"bdev" => bdev,
			"proc" => _proc,
			"cpuset" => cpuset,
			"cgroup" => cgroup,
			"tmpfs" => tmpfs,
			"devtmpfs" => devtmpfs,
			"security" => security,
			"sockfs" => sockfs,
			"pipefs" => pipefs,
			"devpts" => devpts,
			"hugetlbfs" => hugetlbfs,
			"pstore" => pstore,
			"mqueue" => mqueue,
			"ext2" => ext2,
			"ext3" => ext3,
			"ext4" => ext4,
			
			"anon_inodefs" => anon_inodefs,
			"binfmt_misc" => binfmt_misc,
			"debugfs" => debugfs,
			"ecryptfs" => ecryptfs,
			"fuse" => fuse,
			"fuseblk" => fuseblk,
			"fusectl" => fusectl,
			"prl_fs" => prl_fs,
			"securityfs" => securityfs,
			"vfat" => vfat,

			_ => Unrecognised(value)
		}
	}
	
	//noinspection SpellCheckingInspection
	/// From str.
	#[inline(always)]
	fn from_str(value: &str) -> FileSystemType
	{
		use self::FileSystemType::*;
		
		match value
		{
			"sysfs" => sysfs,
			"rootfs" => rootfs,
			"ramfs" => ramfs,
			"bdev" => bdev,
			"proc" => _proc,
			"cpuset" => cpuset,
			"cgroup" => cgroup,
			"tmpfs" => tmpfs,
			"devtmpfs" => devtmpfs,
			"security" => security,
			"sockfs" => sockfs,
			"pipefs" => pipefs,
			"devpts" => devpts,
			"hugetlbfs" => hugetlbfs,
			"pstore" => pstore,
			"mqueue" => mqueue,
			"ext2" => ext2,
			"ext3" => ext3,
			"ext4" => ext4,
			
			"anon_inodefs" => anon_inodefs,
			"binfmt_misc" => binfmt_misc,
			"debugfs" => debugfs,
			"ecryptfs" => ecryptfs,
			"fuse" => fuse,
			"fuseblk" => fuseblk,
			"fusectl" => fusectl,
			"prl_fs" => prl_fs,
			"securityfs" => securityfs,
			"vfat" => vfat,
			
			_ => Unrecognised(value.to_owned())
		}
	}
	
	pub(crate) fn parse(file_path: &Path) -> Result<HashMap<FileSystemType, HasNoAssociatedDevice>, io::Error>
	{
		let mut reader = BufReader::with_capacity(4096, File::open(file_path)?);
		
		let mut file_systems_map = HashMap::new();
		let mut line_number = 0;
		let mut line = String::with_capacity(32);
		while reader.read_line(&mut line)? > 0
		{
			{
				let mut split = line.splitn(2, '\t');
				
				let has_no_associated_device = match split.next().unwrap()
				{
					"" => false,
					"nodev" => true,
					
					unrecognised @ _ => return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero-based line number '{}' has a first column value of '{}' which isn't recognised", line_number, unrecognised.to_owned()))),
				};
				
				let file_system_type = match split.next()
				{
					None => return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero-based line number '{}' does not have second column", line_number))),
					Some(value) => Self::from_str(value),
				};
				
				if let Some(_) = file_systems_map.insert(file_system_type, has_no_associated_device)
				{
					return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero-based line number '{}' is a duplicate", line_number)));
				}
			}
			
			line.clear();
			line_number += 1;
		}
		
		Ok(file_systems_map)
	}
}
