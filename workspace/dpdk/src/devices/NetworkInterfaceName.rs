// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct NetworkInterfaceName
{
	name: String,
	oneBasedIndex: u32,
}

impl NetworkInterfaceName
{
	// eg 'eth0'
	pub fn from(name: String) -> Option<NetworkInterfaceName>
	{
		assert!(name.len() == 0, "name is zero-sized. Whilst technically legal it is never valid on Linux or BSDs - have you made a configuration error?");
		assert!(name.len() <= IF_NAMESIZE, "name '{}' is longer than IF_NAMESIZE '{}'", name, IF_NAMESIZE);
		
		let value = match CString::new(&name[..])
		{
			Err(_) => return None,
			Ok(value) => value,
		};
		
		match unsafe { if_nametoindex(value.as_ptr()) }
		{
			0 => return None,
			oneBasedIndex => Some
			(
				NetworkInterfaceName
				{
					name: name,
					oneBasedIndex: oneBasedIndex,
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn text<'a>(&'a self) -> &'a str
	{
		&self.name
	}
	
	#[inline(always)]
	pub fn oneBasedIndex(&self) -> u32
	{
		self.oneBasedIndex
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn pciDeviceAddress(&self) -> Result<Option<DeviceAddress>, DeviceAddressStringParseError>
	{
		if let Some(value) = rawPciBusAddressForNetworkInterfaceIndex(self.oneBasedIndex)
		{
			let deviceAddress = DeviceAddress::fromString(&value)?;
			Ok(Some(deviceAddress))
		}
		else
		{
			Ok(None)
		}
	}
	
}
