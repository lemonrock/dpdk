// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct SoftAndHardResourceLimit
{
	soft: ResourceLimit,
	hard: ResourceLimit,
}

impl SoftAndHardResourceLimit
{
	pub const BothInfinite: SoftAndHardResourceLimit = SoftAndHardResourceLimit
	{
		soft: ResourceLimit::Infinite,
		hard: ResourceLimit::Infinite,
	};
	
	pub const BothZero: SoftAndHardResourceLimit = SoftAndHardResourceLimit
	{
		soft: ResourceLimit::Finite(0),
		hard: ResourceLimit::Finite(0),
	};
	
	pub fn both(softAndHard: ResourceLimit) -> Self
	{
		Self::new(softAndHard.clone(), softAndHard)
	}
	
	pub fn new(soft: ResourceLimit, hard: ResourceLimit) -> Self
	{
		if soft.isInfinite() && hard.isFinite()
		{
			panic!("softLimit can not be infinite if hard '{}' is finite", hard.unwrap());
		}
		
		if soft.isFinite() && hard.isFinite()
		{
			assert!(soft.unwrap() <= hard.unwrap(), "soft '{:?}' must be less than or the same as hard '{:?}'", soft, hard);
		}
		
		SoftAndHardResourceLimit
		{
			soft: soft,
			hard: hard,
		}
	}
	
	#[inline(always)]
	pub fn hardLimit(&self) -> &ResourceLimit
	{
		&self.hard
	}
	
	pub fn set(&self, resourceIdentifier: i32)
	{
		let value = rlimit64
		{
			rlim_cur: self.soft.unwrap(),
			rlim_max: self.hard.unwrap(),
		};
		
		match unsafe { ::libc::setrlimit64(resourceIdentifier, &value) }
		{
			0 => (),
			
			-1 => match errno().0
			{
				E::EPERM => panic!("Permission denied or tried to increase MaximumNumberOfFileDescriptors above /proc/sys/fs/nr_open"),

				E::EINVAL => panic!("Limit was too large or bad resource id"),
				E::EFAULT => panic!("Bad pointer"),
			
				illegal @ _ => panic!("Illegal errno '{}' from setrlimit64()", illegal),
			},
			
			illegal @ _ => panic!("Illegal result '{}' from setrlimit64()", illegal),
		}
	}
	
	pub fn get(resourceIdentifier: i32) -> Self
	{
		let mut value = rlimit64
		{
			rlim_cur: 0,
			rlim_max: 0,
		};
		
		match unsafe { ::libc::getrlimit64(resourceIdentifier, &mut value) }
		{
			0 => (),
			
			-1 => match errno().0
			{
				E::EPERM => panic!("Permission denied"),

				E::EINVAL => panic!("Bad resource id"),
				E::EFAULT => panic!("Bad pointer"),
			
				illegal @ _ => panic!("Illegal errno '{}' from setrlimit64()", illegal),
			},
			
			illegal @ _ => panic!("Illegal result '{}' from setrlimit64()", illegal),
		};
		
		SoftAndHardResourceLimit
		{
			soft: ResourceLimit::convert(value.rlim_cur),
			hard: ResourceLimit::convert(value.rlim_max),
		}
	}
}
