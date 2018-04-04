// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct HugePageMountSettings
{
	pub mountPoint: PathBuf,
	pub userId: uid_t,
	pub groupId: gid_t,
	pub mode: mode_t,
	pub maximumValueOfMemoryInBytes: Option<u64>,
	pub minimumValueOfMemoryInBytes: Option<u64>,
	pub maximumNumberOfINodes: Option<u64>,
	pub mountFlags: MountFlags,
}

impl Default for HugePageMountSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		HugePageMountSettings
		{
			// DPDK dpdk-setup.sh uses /mnt/huge, but:-
			// - DPDK docs variously also use /mnt/hugepages and /mnt/hugetlbfs
			// - RHEL, Debian tutorials and examples use /hugetlbfs
			// - Ubuntu official DEB uses /dev/hugepages (https://gerrit.fd.io/r/gitweb?p=deb_dpdk.git;a=blob;f=debian/dpdk-init;h=86eda2cb9c4e802aa07603761a82b312f4bb7fa2;hb=HEAD)
			// - At least one admin on a forum uses /mnt/huge_1gb
			mountPoint: PathBuf::from("/mnt/huge"),
			userId: 0,
			groupId: 0,
		
			// Debian uses 1770 for mode
			mode: 0o0755,
			
			maximumValueOfMemoryInBytes: None,
			minimumValueOfMemoryInBytes: None,
			maximumNumberOfINodes: None,
			mountFlags: MountFlags::DoNotUpdateAccessTimes | MountFlags::DoNotAllowProgramsToBeExecuted | MountFlags::DoNotHonourSetUidAndSetGidPermissions,
		}
	}
}

impl HugePageMountSettings
{
	pub fn asMountOptions(&self, overrideDefaultHugePageSize: Option<HugePageSize>) -> HashMap<String, Option<String>>
	{
		let mut mountOptions = HashMap::with_capacity(8);
		mountOptions.insert("uid".to_owned(), Some(format!("{}", self.userId)));
		mountOptions.insert("gid".to_owned(), Some(format!("{}", self.groupId)));
		mountOptions.insert("mode".to_owned(), Some(format!("{:04o}", self.mode)));
		
		if let Some(hugePageSize) = overrideDefaultHugePageSize
		{
			mountOptions.insert("pagesize".to_owned(), Some(hugePageSize.to_str().to_owned()));
		}
		
		if let Some(maximumValueOfMemoryInBytes) = self.maximumValueOfMemoryInBytes
		{
			mountOptions.insert("size".to_owned(), Some(format!("{}", maximumValueOfMemoryInBytes)));
		}
		
		if let Some(minimumValueOfMemoryInBytes) = self.minimumValueOfMemoryInBytes
		{
			mountOptions.insert("min_size".to_owned(), Some(format!("{}", minimumValueOfMemoryInBytes)));
		}
		
		if let Some(maximumNumberOfINodes) = self.maximumNumberOfINodes
		{
			mountOptions.insert("nr_inodes".to_owned(), Some(format!("{}", maximumNumberOfINodes)));
		}
		
		mountOptions
	}
	
	pub fn mountFlags(&self) -> MountFlags
	{
		self.mountFlags
	}
}
