// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait TransmitQueueBufferErrorCallback : Sized
{
	/// Invoked when packets could not be sent.
	/// Return true to have packets freed after the call
	#[inline(always)]
	fn call(&mut self, unsentPackets: &mut [*mut rte_mbuf]) -> bool;
	
	#[inline(always)]
	unsafe extern "C" fn callFromC(unsent: *mut *mut rte_mbuf, count: uint16_t, userdata: *mut c_void)
	{
		let unsentPackets: &mut [*mut rte_mbuf] = from_raw_parts_mut(unsent, count as usize);
		
		let us: &mut Self = &mut *(userdata as *mut Self);
		if us.call(unsentPackets)
		{
			::dpdk_sys::rte_eth_tx_buffer_drop_callback(unsent, count, null_mut());
		}
		forget(unsentPackets);
	}
	
	#[inline(always)]
	fn asFunctionPointer() -> buffer_tx_error_fn
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
