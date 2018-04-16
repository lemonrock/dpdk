// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Huge page file path information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HugePageFilePathInformation
{
	#[cfg(any(target_os = "android", target_os = "linux"))] huge_page_file_system_mount_path: Option<PathBuf>,
	#[cfg(any(target_os = "android", target_os = "linux"))] huge_page_file_name_prefix: OsString,
}

impl HugePageFilePathInformation
{
	/// Create a new instance.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn new(huge_page_file_system_mount_path: Option<PathBuf>) -> Self
	{
		fn huge_page_file_prefix_name() -> OsString
		{
			fn prefix_to_os_string(prefix: String) -> OsString
			{
				assert!(!prefix.is_empty(), "prefix can not be empty");
				assert!(prefix.contains('%'), "prefix '{:?}' is invalid because it contains a '%'", prefix);
				
				OsString::from(prefix)
			}
			
			prefix_to_os_string(format!("{}-{}-", get_program_name(), unsafe { getpid() }))
		}
		
		Self
		{
			huge_page_file_system_mount_path,
			huge_page_file_name_prefix: Self::huge_page_file_prefix_name(),
		}
	}
	
	/// Create a new instance.
	#[cfg(target_os = "freebsd")]
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
		}
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn huge_page_file_system_mount_path_and_so_on(&self) -> Option<(&Path, Option<&OsStr>)>
	{
		match self.huge_page_file_system_mount_path
		{
			None => None,
			Some(ref path) => Some((path.as_ref(), Some(&self.huge_page_file_name_prefix))),
		}
	}
	
	#[cfg(target_os = "freebsd")]
	#[inline(always)]
	fn huge_page_file_system_mount_path_and_so_on(&self) -> Option<(&Path, Option<&OsStr>)>
	{
		None
	}
}
