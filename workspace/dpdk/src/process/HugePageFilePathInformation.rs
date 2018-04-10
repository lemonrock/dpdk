// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HugePageFilePathInformation
{
	hugePageFileSystemMountPath: Option<PathBuf>,
	hugePageFileNamePrefix: OsString,
}

#[cfg(any(target_os = "android", target_os = "linux"))]
impl HugePageFilePathInformation
{
	pub fn new(hugePageFileSystemMountPath: Option<PathBuf>) -> Self
	{
		HugePageFilePathInformation
		{
			hugePageFileSystemMountPath: hugePageFileSystemMountPath,
			hugePageFileNamePrefix: Self::hugePageFilePrefixName(),
		}
	}
	
	pub fn hugePageFileSystemMountPathAndSoOn(&self) -> Option<(&Path, Option<&OsStr>)>
	{
		match self.hugePageFileSystemMountPath
		{
			None => None,
			Some(ref path) => Some((path.as_ref(), Some(&self.hugePageFileNamePrefix))),
		}
	}

	fn hugePageFilePrefixName() -> OsString
	{
		Self::prefixToOsString(format!("{}-{}-", get_program_name(), unsafe { ::libc::getpid() }))
	}
	
	fn prefixToOsString(prefix: String) -> OsString
	{
		assert!(!prefix.is_empty(), "prefix can not be empty");
		assert!(prefix.contains('%'), "prefix '{:?}' is invalid because it contains a '%'", prefix);
		
		OsString::from(prefix)
	}
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HugePageFilePathInformation
{
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
impl HugePageFilePathInformation
{
	pub fn new() -> Self
	{
		HugePageFilePathInformation
		{
		}
	}
	
	pub fn hugePageFileSystemMountPathAndSoOn(&self) -> Option<(&Path, Option<&OsStr>)>
	{
		None
	}
}
