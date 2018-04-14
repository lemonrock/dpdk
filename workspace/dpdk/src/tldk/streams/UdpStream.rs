// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


macro_rules! drop_udp_callbacks
{
	($parameters: expr, $error: expr) =>
	{
		{
			unsafe
			{
				drop(Box::from_raw($parameters.recv_cb as *mut ReceiveCallback));
				drop(Box::from_raw($parameters.send_cb as *mut SendCallback));
			}
			Err($error)
		}
	}
}

/// An UDP stream.
///
/// When dropped, the stream is closed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UdpStream<ReceiveCallback: EdgeTriggeredCallback<Self>, SendCallback: EdgeTriggeredCallback<Self>>(NonNull<tle_stream>, PhantomData<(ReceiveCallback, SendCallback)>);

impl<ReceiveCallback: EdgeTriggeredCallback<Self>, SendCallback: EdgeTriggeredCallback<Self>> Drop for UdpStream<ReceiveCallback, SendCallback>
{
	// A handle to a stream is never actually freed, it's just recycled on close.
	#[inline(always)]
	fn drop(&mut self)
	{
		let mut parameters = self.get_parameters();
		
		let result = unsafe { tle_udp_stream_close(self.handle()) };
		
		drop_udp_callbacks!(parameters, ());
		
		match result
		{
			0 => (),
			
			NegativeE::EDEADLK => panic!("Stream already closed from tle_udp_stream_close()"),
			
			NegativeE::EINVAL => panic!("Invalid stream from tle_udp_stream_close()"),
			
			negative if negative < 0 => panic!("Unexpected errno '{}' from tle_udp_stream_close()", -negative),
			
			_ => panic!("tle_udp_stream_close() returned a positive result"),
		}
	}
}

impl<ReceiveCallback: EdgeTriggeredCallback<Self>, SendCallback: EdgeTriggeredCallback<Self>> Stream for UdpStream<ReceiveCallback, SendCallback>
{
	const Protocol: Layer4Protocol = Layer4Protocol::Udp;
	
	#[inline(always)]
	fn maximum_segment_size(&self) -> u16
	{
		RTE_MBUF_DEFAULT_DATAROOM - TLE_DST_MAX_HDR as u16
	}
}

impl<ReceiveCallback: EdgeTriggeredCallback<Self>, SendCallback: EdgeTriggeredCallback<Self>> UdpStream<ReceiveCallback, SendCallback>
{
	/// Creates and opens a new UDP stream.
	///
	/// Returns an error on limit on number of streams reached.
	#[inline(always)]
	pub fn new<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>>(context: &mut UdpContext<IpV4, IpV6>, local_address: sockaddr_storage, remote_address: sockaddr_storage, receive: Box<ReceiveCallback>, send: Box<SendCallback>) -> Result<Self, StreamCreationError>
	{
		use self::StreamCreationError::*;
		
		let udp_stream_handle =
		{
			let parameters = Self::parameters(local_address, remote_address, receive, send);
			
			let result = unsafe { tle_udp_stream_open(context.handle().as_ptr(), &parameters) };
			if likely(!result.is_null())
			{
				unsafe { NonNull::new_unchecked(result) }
			}
			else
			{
				match unsafe { rust_rte_errno() }
				{
					E::EAGAIN => return drop_udp_callbacks!(parameters, BecauseThereIsOutstandingDataToSendOrReceiveOnTheStreamBeforeIsCanBeUsed),
					
					E::ENFILE => return drop_udp_callbacks!(parameters, NoMoreStreamsAvailable),
					
					E::EINVAL => panic!("Supplied an invalid value"),
					
					illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_stream_open()", illegal),
				}
			}
		};
		
		Ok(UdpStream(udp_stream_handle, PhantomData))
	}
	
	#[inline(always)]
	pub fn receive(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16
	{
		return unsafe { tle_udp_stream_recv(self.handle(), pkt, num) }
	}

	#[inline(always)]
	pub fn send(&mut self, pkt: *mut *mut rte_mbuf, num: u16, dst_addr: *const sockaddr) -> u16
	{
		return unsafe { tle_udp_stream_send(self.handle(), pkt, num, dst_addr) }
	}
	
	unsafe extern "C" fn receive_callback(arg1: *mut c_void, arg2: *mut tle_stream)
	{
		debug_assert!(!arg1.is_null(), "arg1 was null");
		debug_assert!(!arg2.is_null(), "arg2 was null");
		
		let callback = &mut *(arg1 as *mut ReceiveCallback);
		callback.call(NonNull::new_unchecked(arg2));
	}
	
	unsafe extern "C" fn send_callback(arg1: *mut c_void, arg2: *mut tle_stream)
	{
		debug_assert!(!arg1.is_null(), "arg1 was null");
		debug_assert!(!arg2.is_null(), "arg2 was null");
		
		let callback = &mut *(arg1 as *mut SendCallback);
		callback.call(NonNull::new_unchecked(arg2));
	}
	
	#[inline(always)]
	fn parameters(local_address: sockaddr_storage, remote_address: sockaddr_storage, receive: Box<ReceiveCallback>, send: Box<SendCallback>) -> tle_udp_stream_param
	{
		tle_udp_stream_param
		{
			local_addr: local_address,
			remote_addr: remote_address,
			recv_ev: null_mut(),
			recv_cb: tle_stream_cb
			{
				func: Self::receive_callback,
				data: Box::into_raw(receive),
			},
			send_ev: null_mut(),
			send_cb: tle_stream_cb
			{
				func: Self::send_callback,
				data: Box::into_raw(send),
			},
		}
	}

	#[inline(always)]
	fn get_parameters(&self) -> tle_udp_stream_param
	{
		let mut parameters = unsafe { uninitialized() };
		let result = unsafe { tle_udp_stream_get_param(self.handle(), &mut parameters) };
		debug_assert_ne!(result, 0, "tle_udp_stream_get_param() should never fail");
		parameters
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut tle_stream
	{
		self.0.as_ptr()
	}
}
