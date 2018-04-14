// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UdpDevice(*mut tle_dev);

impl Device for UdpDevice
{
	const Protocol: Layer4Protocol = Layer4Protocol::Udp;

	#[doc(hidden)]
	#[inline(always)]
	fn _new(opaqueFfiHandle: *mut tle_dev) -> Self
	{
		UdpDevice(opaqueFfiHandle)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn handle(&mut self) -> *mut tle_dev
	{
		self.0
	}

	#[inline(always)]
	fn bulkReceive(&mut self, pkt: *mut *mut rte_mbuf, rp: *mut *mut rte_mbuf, rc: *mut i32, num: u16) -> u16
	{
		let numberAccepted = unsafe { tle_udp_rx_bulk(self.handle(), pkt, rp, rc, num) };
		debug_assert!(numberAccepted <= num, "numberAccepted '{}' was larger than original num '{}' to send", numberAccepted, num);
		numberAccepted
	}

	#[inline(always)]
	fn bulkTransmit(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16
	{
		unsafe { tle_udp_tx_bulk(self.handle(), pkt, num) }
	}
}
