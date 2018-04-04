// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct TcpContext<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>>(*mut tle_ctx, Rc<RefCell<IpV4>>, Rc<RefCell<IpV6>>);

impl<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>> Drop for TcpContext<IpV4, IpV6>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self._drop();
	}
}

impl<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>> Context<IpV4, IpV6> for TcpContext<IpV4, IpV6>
{
	const Protocol: Layer4Protocol = Layer4Protocol::Tcp;
	
	type Device = TcpDevice;
	
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
		TcpContext(opaqueFfiHandle, addressLookUpForSendToIpV4, addressLookUpForSendToIpV6)
	}
	
	#[inline(always)]
	fn poll(&mut self, number: u32)
	{
		let result = unsafe { ::dpdk_sys::tle_tcp_process(self._opaqueFfiHandle(), number) };
		if likely(result == 0)
		{
			return;
		}
		if unlikely(result > 0)
		{
			panic!("tle_tcp_process() returned a positive result");
		}
		match result
		{
			NegativeE::EINVAL => panic!("Supplied an invalid value"),
			
			illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_process()", -illegal),
		}
	}
}
