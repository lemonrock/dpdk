// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait TldkDevice
{
	const Protocol: Layer4Protocol;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(opaqueFfiHandle: *mut tle_dev) -> Self;
	
	#[doc(hidden)]
	#[inline(always)]
	fn handle(&mut self) -> *mut tle_dev;

	#[inline(always)]
	fn delete(&mut self)
	{
		let result = unsafe { tle_del_dev(self.handle()) };
		if likely(result == 0)
		{
			return;
		}
		if unlikely(result > 0)
		{
			panic!("tle_del_dev() returned a positive result");
		}
		match result
		{
			NegativeE::EINVAL => panic!("Invalid device from tle_del_dev()"),
		
			illegal @ _ => panic!("Unexpected errno '{}' from tle_del_dev()", -illegal),
		}
	}

	#[inline(always)]
	fn bulkReceive(&mut self, pkt: *mut *mut rte_mbuf, rp: *mut *mut rte_mbuf, rc: *mut i32, num: u16) -> u16;

	#[inline(always)]
	fn bulkTransmit(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16;
}
