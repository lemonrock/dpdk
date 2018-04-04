// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct UdpContext<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>>(*mut tle_ctx, Rc<RefCell<IpV4>>, Rc<RefCell<IpV6>>);

impl<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>> Drop for UdpContext<IpV4, IpV6>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self._drop();
	}
}

impl<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>> Context<IpV4, IpV6> for UdpContext<IpV4, IpV6>
{
	const Protocol: Layer4Protocol = Layer4Protocol::Udp;
	
	type Device = UdpDevice;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _opaqueFfiHandle(&mut self) -> *mut tle_ctx
	{
		self.0
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _newContext(opaqueFfiHandle: *mut tle_ctx, addressLookUpForSendToIpV4: Rc<RefCell<IpV4>>, addressLookUpForSendToIpV6: Rc<RefCell<IpV6>>) -> Self
	{
		UdpContext(opaqueFfiHandle, addressLookUpForSendToIpV4, addressLookUpForSendToIpV6)
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn poll(&mut self, number: u32)
	{
	}
}

impl<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>> UdpContext<IpV4, IpV6>
{
}
