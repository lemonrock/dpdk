// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	#[derive(Serialize, Deserialize)]
	pub flags UnmountFlags: i32
	{
		const Force = ::libc::MNT_FORCE,
		const Detach = ::libc::MNT_DETACH,
		const Expire = ::libc::MNT_EXPIRE,
		
		// Not in libc crate
		// const NoFollow = ::libc::UMOUNT_NOFOLLOW,
	}
}
