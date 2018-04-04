// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait EthernetPortEventCallback : Sized
{
	#[inline(always)]
	fn call(&mut self, portIdentifier: EthernetPortIdentifier);
	
	#[allow(unused_variables)]
	#[inline(always)]
	unsafe extern "C" fn callFromC(port_id: uint8_t, event: rte_eth_event_type, cb_arg: *mut c_void)
	{
		let us: &mut Self = &mut *(cb_arg as *mut Self);
		us.call(port_id);
	}
	
	#[inline(always)]
	fn asFunctionPointer() -> rte_eth_dev_cb_fn
	{
		Some(Self::callFromC)
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn asFunctionArgument(&mut self) -> *mut c_void
	{
		self as *mut _ as *mut c_void
	}
}
