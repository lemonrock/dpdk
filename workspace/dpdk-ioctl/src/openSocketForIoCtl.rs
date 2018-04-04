// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub fn openSocketForIoCtl() -> Result<i32, OpenIoCtlSocketError>
{
	match unsafe { socket(AF_INET, SOCK_DGRAM, IPPROTO_IP) }
	{
		socketFileDescriptor if socketFileDescriptor >= 0 => Ok(socketFileDescriptor),
		
		-1 => match { errno().0 }
		{
			E::EACCES => Err(OpenIoCtlSocketError::PermissionDenied),
			E::EAFNOSUPPORT => Err(OpenIoCtlSocketError::Unsupported("Address family not supported")),
			E::EPROTOTYPE => Err(OpenIoCtlSocketError::Unsupported("The socket type is not supported by the protocol")),
			E::EPROTONOSUPPORT => Err(OpenIoCtlSocketError::Unsupported("The protocol type or the specified protocol is not supported within this domain")),
			
			E::EMFILE => Err(OpenIoCtlSocketError::OutOfMemoryOrResources("The per-process descriptor table is full")),
			E::ENFILE => Err(OpenIoCtlSocketError::OutOfMemoryOrResources("The system file table is full")),
			E::ENOBUFS => Err(OpenIoCtlSocketError::OutOfMemoryOrResources("Insufficient buffer space is available; the socket cannot be created until sufficient resources are freed")),
			E::ENOMEM => Err(OpenIoCtlSocketError::OutOfMemoryOrResources("Insufficient memory was available to fulfill the request")),
			
			illegal @ _ => panic!("socket() had illegal errno '{}'", illegal),
		},
		
		illegal @ _ => panic!("Illegal result '{}' from socket()", illegal),
	}
}
