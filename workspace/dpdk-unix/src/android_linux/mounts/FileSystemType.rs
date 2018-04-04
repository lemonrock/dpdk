// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//noinspection RustEnumVariantNaming
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
	pub fn to_CString(&self) -> CString
	{
		let refValue = match *self
		{
			FileSystemType::sysfs => "sysfs",
			FileSystemType::rootfs => "rootfs",
			FileSystemType::ramfs => "ramfs",
			FileSystemType::bdev => "bdev",
			FileSystemType::_proc => "_proc",
			FileSystemType::cpuset => "cpuset",
			FileSystemType::cgroup => "cgroup",
			FileSystemType::tmpfs => "tmpfs",
			FileSystemType::devtmpfs => "devtmpfs",
			FileSystemType::security => "security",
			FileSystemType::sockfs => "sockfs",
			FileSystemType::pipefs => "pipefs",
			FileSystemType::devpts => "devpts",
			FileSystemType::hugetlbfs => "hugetlbfs",
			FileSystemType::pstore => "pstore",
			FileSystemType::mqueue => "mqueue",
			FileSystemType::ext2 => "ext2",
			FileSystemType::ext3 => "ext3",
			FileSystemType::ext4 => "ext4",
			
			FileSystemType::anon_inodefs => "anon_inodefs",
			FileSystemType::binfmt_misc => "binfmt_misc",
			FileSystemType::debugfs => "debugfs",
			FileSystemType::ecryptfs => "ecryptfs",
			FileSystemType::fuse => "fuse",
			FileSystemType::fuseblk => "fuseblk",
			FileSystemType::fusectl => "fusectl",
			FileSystemType::prl_fs => "prl_fs",
			FileSystemType::securityfs => "securityfs",
			FileSystemType::vfat => "vfat",
			
			FileSystemType::Unrecognised(ref value) => value,
		};
		
		CString::new(refValue.to_owned()).expect("fileSystemTypes should not contain interior ASCII NULs")
	}
	
	pub fn from_String(value: String) -> FileSystemType
	{
		match &value[..]
		{
			"sysfs" => FileSystemType::sysfs,
			"rootfs" => FileSystemType::rootfs,
			"ramfs" => FileSystemType::ramfs,
			"bdev" => FileSystemType::bdev,
			"proc" => FileSystemType::_proc,
			"cpuset" => FileSystemType::cpuset,
			"cgroup" => FileSystemType::cgroup,
			"tmpfs" => FileSystemType::tmpfs,
			"devtmpfs" => FileSystemType::devtmpfs,
			"security" => FileSystemType::security,
			"sockfs" => FileSystemType::sockfs,
			"pipefs" => FileSystemType::pipefs,
			"devpts" => FileSystemType::devpts,
			"hugetlbfs" => FileSystemType::hugetlbfs,
			"pstore" => FileSystemType::pstore,
			"mqueue" => FileSystemType::mqueue,
			"ext2" => FileSystemType::ext2,
			"ext3" => FileSystemType::ext3,
			"ext4" => FileSystemType::ext4,
			
			"anon_inodefs" => FileSystemType::anon_inodefs,
			"binfmt_misc" => FileSystemType::binfmt_misc,
			"debugfs" => FileSystemType::debugfs,
			"ecryptfs" => FileSystemType::ecryptfs,
			"fuse" => FileSystemType::fuse,
			"fuseblk" => FileSystemType::fuseblk,
			"fusectl" => FileSystemType::fusectl,
			"prl_fs" => FileSystemType::prl_fs,
			"securityfs" => FileSystemType::securityfs,
			"vfat" => FileSystemType::vfat,

			_ => FileSystemType::Unrecognised(value)
		}
	}
	
	pub fn new(value: &str) -> FileSystemType
	{
		match value
		{
			"sysfs" => FileSystemType::sysfs,
			"rootfs" => FileSystemType::rootfs,
			"ramfs" => FileSystemType::ramfs,
			"bdev" => FileSystemType::bdev,
			"proc" => FileSystemType::_proc,
			"cpuset" => FileSystemType::cpuset,
			"cgroup" => FileSystemType::cgroup,
			"tmpfs" => FileSystemType::tmpfs,
			"devtmpfs" => FileSystemType::devtmpfs,
			"security" => FileSystemType::security,
			"sockfs" => FileSystemType::sockfs,
			"pipefs" => FileSystemType::pipefs,
			"devpts" => FileSystemType::devpts,
			"hugetlbfs" => FileSystemType::hugetlbfs,
			"pstore" => FileSystemType::pstore,
			"mqueue" => FileSystemType::mqueue,
			"ext2" => FileSystemType::ext2,
			"ext3" => FileSystemType::ext3,
			"ext4" => FileSystemType::ext4,
			
			"anon_inodefs" => FileSystemType::anon_inodefs,
			"binfmt_misc" => FileSystemType::binfmt_misc,
			"debugfs" => FileSystemType::debugfs,
			"ecryptfs" => FileSystemType::ecryptfs,
			"fuse" => FileSystemType::fuse,
			"fuseblk" => FileSystemType::fuseblk,
			"fusectl" => FileSystemType::fusectl,
			"prl_fs" => FileSystemType::prl_fs,
			"securityfs" => FileSystemType::securityfs,
			"vfat" => FileSystemType::vfat,
			
			_ => FileSystemType::Unrecognised(value.to_owned())
		}
	}
	
	pub fn parse(procPath: &Path) -> Result<HashMap<FileSystemType, HasNoAssociatedDevice>, io::Error>
	{
		let mut filesystemsFilePath = PathBuf::from(procPath);
		filesystemsFilePath.push("filesystems");
				
		let openFile = try!(File::open(filesystemsFilePath));
		let mut reader = BufReader::with_capacity(4096, openFile);
		
		let mut fileSystemsMap = HashMap::new();
		let mut lineCount = 0;
		let mut line = String::with_capacity(32);
		while try!(reader.read_line(&mut line)) > 0
		{
			{
				let mut split = line.splitn(2, '\t');
				
				let hasNoAssociatedDevice = match split.next().unwrap()
				{
					"" => false,
					"nodev" => true,
					
					unrecognised @ _ => return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero-based line number '{}' has a first column value of '{}' which isn't recognised", lineCount, unrecognised.to_owned()))),
				};
				
				let fileSystemType = match split.next()
				{
					None => return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero-based line number '{}' does not have second column", lineCount))),
					Some(value) => Self::new(value),
				};
				
				if let Some(_) = fileSystemsMap.insert(fileSystemType, hasNoAssociatedDevice)
				{
					return Err(io::Error::new(ErrorKind::InvalidData, format!("Zero-based line number '{}' is a duplicate", lineCount)));
				}
			}
			
			line.clear();
			lineCount += 1;
		}
		
		Ok(fileSystemsMap)
	}
}
