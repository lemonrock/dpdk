// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// List of supported file systems.
#[derive(Debug)]
pub struct FileSystemTypeList(HashMap<FileSystemType, HasNoAssociatedDevice>);

impl FileSystemTypeList
{
	/// Panics if `hugetlbfs` is not supported or has associated devices (ie is not `nodev`).
	#[inline(always)]
	pub fn verify_hugetlbfs_is_supported(&self)
	{
		match self.0.get(&FileSystemType::hugetlbfs)
		{
			None => panic!("Linux kernel does not support file system 'hugetlbfs'"),
			
			Some(has_no_associated_device) => if !has_no_associated_device
			{
				panic!("File system 'hugetlbfs' has associated devices (ie is not 'nodev')");
			},
		}
	}
	
	pub(crate) fn parse(file_path: &Path) -> Result<Self, io::Error>
	{
		use self::ErrorKind::InvalidData;
		
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
					
					unrecognised @ _ => return Err(io::Error::new(InvalidData, format!("Zero-based line number '{}' has a first column value of '{}' which isn't recognised", line_number, unrecognised.to_owned()))),
				};
				
				let file_system_type = match split.next()
				{
					None => return Err(io::Error::new(InvalidData, format!("Zero-based line number '{}' does not have second column", line_number))),
					Some(value) => FileSystemType::from_str(value),
				};
				
				if let Some(_) = file_systems_map.insert(file_system_type, has_no_associated_device)
				{
					return Err(io::Error::new(InvalidData, format!("Zero-based line number '{}' is a duplicate", line_number)));
				}
			}
			
			line.clear();
			line_number += 1;
		}
		
		Ok(FileSystemTypeList(file_systems_map))
	}
}
