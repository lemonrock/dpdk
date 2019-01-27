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

	Unrecognised(Box<[u8]>)
}

impl FileSystemType
{
	/// To `CString`.
	#[inline(always)]
	pub fn to_c_string(&self) -> CString
	{
		use self::FileSystemType::*;
		
		let ref_value = match *self
		{
			sysfs => "sysfs",
			rootfs => "rootfs",
			ramfs => "ramfs",
			bdev => "bdev",
			_proc => "proc",
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
			
			Unrecognised(ref value) => return CString::new(value.clone()).unwrap(),
		};
		
		CString::new(ref_value.to_owned()).expect("file system type should not contain interior ASCII NULs")
	}
	
	/// From string.
	#[inline(always)]
	pub fn from_c_str(value: &CStr) -> Self
	{
		Self::from_byte_slice(value.to_bytes())
	}
	
	/// From str.
	#[inline(always)]
	pub fn from_byte_slice(value: &[u8]) -> FileSystemType
	{
		use self::FileSystemType::*;
		
		match value
		{
			b"sysfs" => sysfs,
			b"rootfs" => rootfs,
			b"ramfs" => ramfs,
			b"bdev" => bdev,
			b"proc" => _proc,
			b"cpuset" => cpuset,
			b"cgroup" => cgroup,
			b"tmpfs" => tmpfs,
			b"devtmpfs" => devtmpfs,
			b"security" => security,
			b"sockfs" => sockfs,
			b"pipefs" => pipefs,
			b"devpts" => devpts,
			b"hugetlbfs" => hugetlbfs,
			b"pstore" => pstore,
			b"mqueue" => mqueue,
			b"ext2" => ext2,
			b"ext3" => ext3,
			b"ext4" => ext4,
			
			b"anon_inodefs" => anon_inodefs,
			b"binfmt_misc" => binfmt_misc,
			b"debugfs" => debugfs,
			b"ecryptfs" => ecryptfs,
			b"fuse" => fuse,
			b"fuseblk" => fuseblk,
			b"fusectl" => fusectl,
			b"prl_fs" => prl_fs,
			b"securityfs" => securityfs,
			b"vfat" => vfat,
			
			_ => Unrecognised(value.to_vec().into_boxed_slice())
		}
	}
}
