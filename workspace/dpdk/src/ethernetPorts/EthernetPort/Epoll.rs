// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// NOTE: This API should be considered highly unstable
impl EthernetPort
{
	// userData could, be say, &self, or a structure holding self
	#[inline(always)]
	pub fn receiveInterruptEpollControl(&self, epollFileDescriptor: Option<i32>, ePollInterruptEvent: EPollInterruptEvent, userData: *mut c_void) -> bool
	{
		let epollFileDescriptor: i32 = epollFileDescriptor.unwrap_or(RTE_EPOLL_PER_THREAD);
		let result = unsafe { ::dpdk_sys::rte_eth_dev_rx_intr_ctl(self.portIdentifier(), epollFileDescriptor, ePollInterruptEvent as i32, userData) };
		if likely(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				negative if negative < 0 => false,
				
				_ => panic!("Illegal result '{}' from rte_eth_dev_rx_intr_ctl()"),
			}
		}
	}
}
