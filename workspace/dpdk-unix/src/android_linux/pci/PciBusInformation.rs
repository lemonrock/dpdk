// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PciBusInformation(RawFd);

impl Drop for PciBusInformation
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let socket_file_descriptor = self.0;
		
		loop
		{
			match unsafe { close(socket_file_descriptor) }
			{
				0 => return,
				
				-1 => match errno().0
				{
					E::EINTR => continue,
					E::EIO => break,
					E::EBADF => panic!("File descriptor '{}' is not a valid socket file descriptor", socket_file_descriptor),
					
					error_number @ _ => panic!("Could not close() socket_file_descriptor '{}' got error number '{}'", socket_file_descriptor, error_number),
				},
				
				illegal @ _ => panic!("Illegal result '{}' from close() for socket_file_descriptor '{}'", illegal, socket_file_descriptor),
			}
		}
	}
}

impl PciBusInformation
{
	/// Number of bytes in a PCI address string such as `XXXX:XX:XX.XX`.
	pub const NumberOfBytesInPciAddressString: usize = 13;
	
	#[inline(always)]
	fn open_socket_for_ioctl() -> Result<Self, OpenPciBusInformationError>
	{
		match unsafe { socket(AF_INET, SOCK_DGRAM, IPPROTO_IP) }
		{
			socket_file_descriptor if socket_file_descriptor >= 0 => Ok(PciBusInformation(socket_file_descriptor)),
			
			-1 => match { errno().0 }
			{
				E::EACCES => Err(OpenPciBusInformationError::PermissionDenied),
				E::EAFNOSUPPORT => Err(OpenPciBusInformationError::Unsupported("Address family not supported")),
				E::EPROTOTYPE => Err(OpenPciBusInformationError::Unsupported("The socket type is not supported by the protocol")),
				E::EPROTONOSUPPORT => Err(OpenPciBusInformationError::Unsupported("The protocol type or the specified protocol is not supported within this domain")),
				
				E::EMFILE => Err(OpenPciBusInformationError::OutOfMemoryOrResources("The per-process descriptor table is full")),
				E::ENFILE => Err(OpenPciBusInformationError::OutOfMemoryOrResources("The system file table is full")),
				E::ENOBUFS => Err(OpenPciBusInformationError::OutOfMemoryOrResources("Insufficient buffer space is available; the socket cannot be created until sufficient resources are freed")),
				E::ENOMEM => Err(OpenPciBusInformationError::OutOfMemoryOrResources("Insufficient memory was available to fulfill the request")),
				
				illegal @ _ => panic!("socket() had illegal errno '{}'", illegal),
			},
			
			illegal @ _ => panic!("Illegal result '{}' from socket()", illegal),
		}
	}
	
	//noinspection SpellCheckingInspection
	/// On Android or Linux, obtains a raw PCI bus address string in the format `XXXX:XX:XX.XX`.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn raw_pci_bus_address_for_network_interface_index(network_interface_one_based_index: u32) -> Option<String>
	{
		debug_assert!(ETHTOOL_BUSINFO_LEN > Self::NumberOfBytesInPciAddressString + 1, "ETHTOOL_BUSINFO_LEN must exceed by at least one (for a Nul byte)");
		
		let socket_file_descriptor = Self::open_socket_for_ioctl().unwrap();
		
		let mut interface_request = ifreq::default();
		
		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;
		
		// Specify ifr_ifindex 'field'.
		unsafe { write(interface_request.ifr_ifru.ifru_ivalue(), network_interface_one_based_index as i32) };
	
		// Specify ifr_data 'field'.
		unsafe { write(interface_request.ifr_ifru.ifru_data(), &mut command as * mut _ as *mut c_void) };
		
		let raw_pci_bus_address = match unsafe { ioctl(socket_file_descriptor.0, SIOCETHTOOL, &mut interface_request as *mut _ as *mut c_void) }
		{
			-1 => None,
			_ =>
			{
				// Technically incorrect, as the length can be ETHTOOL_BUSINFO_LEN with no terminating NUL; too bad.
				let bytes: &[u8] = unsafe { transmute(&command.bus_info[..]) };
				match CStr::from_bytes_with_nul(bytes)
				{
					Err(_) => None,
					Ok(c_string) => match c_string.to_str()
					{
						Err(_) => None,
						
						Ok(str) => if str.len() != Self::NumberOfBytesInPciAddressString
						{
							None
						}
						else
						{
							Some(str.to_owned())
						}
					},
				}
			}
		};
		
		raw_pci_bus_address
	}
}
