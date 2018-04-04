// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(trivial_casts)]
pub fn rawPciBusAddressForNetworkInterfaceIndex(networkInterfaceOneBasedIndex: u32) -> Option<String>
{
	debug_assert!(ETHTOOL_BUSINFO_LEN > NumberOfBytesInPciAddressString + 1, "ETHTOOL_BUSINFO_LEN must exceed by at least one (for a Nul byte)");
	
	let socketFileDescriptor = match openSocketForIoCtl()
	{
		Err(_) => return None,
		Ok(socketFileDescriptor) => socketFileDescriptor,
	};
	
	let mut interfaceRequest = ifreq::default();
	
	let mut command = ethtool_drvinfo::default();
	command.cmd = ETHTOOL_GDRVINFO;
	
	// Specify ifr_ifindex 'field'
	unsafe { write(interfaceRequest.ifr_ifru.ifru_ivalue(), networkInterfaceOneBasedIndex as i32) };

	// Specify ifr_data 'field'
	unsafe { write(interfaceRequest.ifr_ifru.ifru_data(), &mut command as * mut _ as *mut c_void) };
	
	let rawPciBusAddress = match unsafe { ioctl(socketFileDescriptor, SIOCETHTOOL, &mut interfaceRequest as *mut _ as *mut c_void) }
	{
		-1 => None,
		_ =>
		{
			// Technically incorrect, as the length can be ETHTOOL_BUSINFO_LEN with no terminating NUL; too bad
			let bytes: &[u8] = unsafe { transmute(&command.bus_info[..]) };
			match CStr::from_bytes_with_nul(bytes)
			{
				Err(_) => None,
				Ok(cstring) => match cstring.to_str()
				{
					Err(_) => None,
					
					Ok(str) => if str.len() != NumberOfBytesInPciAddressString
					{
						None
					}
					else
					{
						Some(str.to_owned())
					}
				},
			}
		},
	};
	
	closeSocketFileDescriptor(socketFileDescriptor);
	
	rawPciBusAddress
}
