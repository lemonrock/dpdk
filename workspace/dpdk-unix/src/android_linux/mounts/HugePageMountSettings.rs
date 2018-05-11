// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Settings for mounting a hugetlbfs file system.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct HugePageMountSettings
{
	/// Mount point for huge pages file system.
	///
	/// There is no real standard; we default to `/dev/hugepages`, this being the default used by the Ubuntu official deb package and similar in spirit to `/dev/shm`. Other options include:-
	///
	/// - DPDK `dpdk-setup.sh` uses `/mnt/huge`
	/// - DPDK docs variously also use `/mnt/hugepages` and `/mnt/hugetlbfs`.
	/// - RHEL, Debian tutorials and examples use `/hugetlbfs`.
	/// - Ubuntu official deb package uses `/dev/hugepages` (<https://gerrit.fd.io/r/gitweb?p=deb_dpdk.git;a=blob;f=debian/dpdk-init;h=86eda2cb9c4e802aa07603761a82b312f4bb7fa2;hb=HEAD>).
	/// - At least one admin on a forum uses `/mnt/huge_1gb`.
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
			mount_point: PathBuf::from("/dev/hugepages"),
			user_id: 0,
			group_id: 0,
		
			mode: 0o0700,
			
			maximum_value_of_memory_in_bytes: None,
			minimum_value_of_memory_in_bytes: None,
			maximum_number_of_inodes: None,
			mount_flags: MountFlags::DoNotUpdateAccessTimes | MountFlags::DoNotAllowProgramsToBeExecuted | MountFlags::DoNotHonourSetUidAndSetGidPermissions,
		}
	}
}

impl HugePageMountSettings
{
	/// Creates the required mount point then mounts `hugetlbfs`.
	///
	/// Panics if mount already exists and is not a directory.
	///
	/// Returns `true` if the mount point was created or `false` if it already existed.
	pub fn mount(&self, sys_path: &SysPath) -> bool
	{
		let largest_huge_page_size = HugePageSize::largest_supported_huge_page_size(sys_path);
		
		let was_created = self.create_mount_point_if_required();
		
		let mount_options = self.as_mount_options(Some(largest_huge_page_size));
		Mount::new_where_source_is_file_system_type(self.mount_point.clone(), FileSystemType::hugetlbfs, mount_options).mount(self.mount_flags).expect("Could not mount hugetlbfs");
		
		was_created
	}
	
	#[inline(always)]
	fn create_mount_point_if_required(&self) -> bool
	{
		let mount_point = &self.mount_point;
		
		if mount_point.exists()
		{
			if !mount_point.is_dir()
			{
				panic!("Mount point {:?} for hugeltbfs is not a directory", mount_point);
			}
			false
		}
		else
		{
			create_dir_all(mount_point).expect(&format!("Could not create hugeltbfs mount_point at {:?}", mount_point));
			true
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn as_mount_options(&self, override_default_huge_page_size: Option<HugePageSize>) -> HashMap<String, Option<String>>
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
