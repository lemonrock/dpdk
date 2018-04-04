// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub fn closeSocketFileDescriptor(socketFileDescriptor: c_int)
{
	loop
	{
		match unsafe { close(socketFileDescriptor) }
		{
			0 => return,
		
			-1 => match errno().0
			{
				E::EINTR => continue,
				E::EIO => break,
				E::EBADF => panic!("File descriptor '{}' is not a valid socket", socketFileDescriptor),
			
				errorNumber @ _ => panic!("Could not close() socketFileDescriptor '{}' got error number '{}'", socketFileDescriptor, errorNumber),
			},
		
			illegal @ _ => panic!("Illegal result '{}' from close() for socketFileDescriptor '{}'", illegal, socketFileDescriptor),
		}
	}
}
