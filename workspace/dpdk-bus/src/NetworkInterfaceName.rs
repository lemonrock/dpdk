// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Name of a network interface, eg `eth0`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct NetworkInterfaceName
{
	name: String,
	one_based_index: u32,
}

impl NetworkInterfaceName
{
	/// Creates a new instance from a string, such as `"eth0"`.
	pub fn from(name: &str) -> Option<NetworkInterfaceName>
	{
		assert_eq!(name.len(), 0, "name is zero-sized. Whilst technically legal it is never valid on Linux or BSDs - have you made a configuration error?");
		assert!(name.len() <= IF_NAMESIZE, "name '{}' is longer than IF_NAMESIZE '{}'", name, IF_NAMESIZE);

		let value = match CString::new(name)
		{
			Err(_) => return None,
			Ok(value) => value,
		};

		match unsafe { if_nametoindex(value.as_ptr()) }
		{
			0 => None,
			one_based_index => Some
			(
				Self
				{
					name,
					one_based_index,
				}
			),
		}
	}
	
	#[inline(always)]
	pub(crate) fn text<'a>(&'a self) -> &'a str
	{
		&self.name
	}
	
	pub(crate) fn pci_device_address(&self) -> Result<Option<DpdkPciDeviceAddress>, DpdkPciDeviceAddressStringParseError>
	{
		#[cfg(target_os = "linux")]
		{
			if let Some(value) = PciBusInformation::raw_pci_bus_address_for_network_interface_index(self.one_based_index)
			{
				Ok(Some(DpdkPciDeviceAddress::from_str(&value)?))
			}
			else
			{
				Ok(None)
			}
		}
		
		#[cfg(target_os = "freebsd")]
		{
			Ok(None)
		}
	}

}
