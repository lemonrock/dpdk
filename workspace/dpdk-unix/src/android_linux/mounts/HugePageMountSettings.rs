// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Settings for mounting a hugetlbfs file system.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct HugePageMountSettings
{
	/// Mount point for huge pages file system.
	///
	/// DPDK dpdk-setup.sh uses /mnt/huge, but:-
	/// - DPDK docs variously also use /mnt/hugepages and /mnt/hugetlbfs
	/// - RHEL, Debian tutorials and examples use /hugetlbfs
	/// - Ubuntu official DEB uses /dev/hugepages (https://gerrit.fd.io/r/gitweb?p=deb_dpdk.git;a=blob;f=debian/dpdk-init;h=86eda2cb9c4e802aa07603761a82b312f4bb7fa2;hb=HEAD)
	/// - At least one admin on a forum uses /mnt/huge_1gb
	pub mount_point: PathBuf,
	
	/// User id (`uid`), eg `0`.
	pub user_id: uid_t,
	
	/// Group id (`gid`), eg `0`.
	pub group_id: gid_t,
	
	/// Permissions mode.
	///
	/// Debian uses 1770 for mode.
	pub mode: mode_t,
	
	/// eg `None`.
	pub maximum_value_of_memory_in_bytes: Option<u64>,
	
	/// eg `None`.
	pub minimum_value_of_memory_in_bytes: Option<u64>,
	
	/// eg `None`.
	pub maximum_number_of_inodes: Option<u64>,
	
	/// eg `MountFlags::DoNotUpdateAccessTimes | MountFlags::DoNotAllowProgramsToBeExecuted | MountFlags::DoNotHonourSetUidAndSetGidPermissions`.
	pub mount_flags: MountFlags,
}

impl Default for HugePageMountSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			mount_point: PathBuf::from("/mnt/huge"),
			user_id: 0,
			group_id: 0,
		
			mode: 0o0755,
			
			maximum_value_of_memory_in_bytes: None,
			minimum_value_of_memory_in_bytes: None,
			maximum_number_of_inodes: None,
			mount_flags: MountFlags::DoNotUpdateAccessTimes | MountFlags::DoNotAllowProgramsToBeExecuted | MountFlags::DoNotHonourSetUidAndSetGidPermissions,
		}
	}
}

impl HugePageMountSettings
{
	//noinspection SpellCheckingInspection
	pub(crate) fn as_mount_options(&self, override_default_huge_page_size: Option<HugePageSize>) -> HashMap<String, Option<String>>
	{
		let mut mount_options = HashMap::with_capacity(8);
		mount_options.insert("uid".to_owned(), Some(format!("{}", self.user_id)));
		mount_options.insert("gid".to_owned(), Some(format!("{}", self.group_id)));
		mount_options.insert("mode".to_owned(), Some(format!("{:04o}", self.mode)));
		
		if let Some(huge_page_size) = override_default_huge_page_size
		{
			mount_options.insert("pagesize".to_owned(), Some(huge_page_size.to_str().to_owned()));
		}
		
		if let Some(maximumvalue_of_memory_in_bytes) = self.maximum_value_of_memory_in_bytes
		{
			mount_options.insert("size".to_owned(), Some(format!("{}", maximumvalue_of_memory_in_bytes)));
		}
		
		if let Some(minimumvalue_of_memory_in_bytes) = self.minimum_value_of_memory_in_bytes
		{
			mount_options.insert("min_size".to_owned(), Some(format!("{}", minimumvalue_of_memory_in_bytes)));
		}
		
		if let Some(maximum_number_of_inodes) = self.maximum_number_of_inodes
		{
			mount_options.insert("nr_inodes".to_owned(), Some(format!("{}", maximum_number_of_inodes)));
		}
		
		mount_options
	}
}
