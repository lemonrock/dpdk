// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UdpStream
<
	ReceiveLevelTriggeredData,
	ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback,
	SendLevelTriggeredData,
	SendEdgeTriggeredCallback: EdgeTriggeredCallback
>
(
	*mut tle_stream,
	PhantomData<ReceiveLevelTriggeredData>,
	PhantomData<ReceiveEdgeTriggeredCallback>,
	PhantomData<SendLevelTriggeredData>,
	PhantomData<SendEdgeTriggeredCallback>
);

impl<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback> Drop for UdpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback>
where ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback, SendEdgeTriggeredCallback: EdgeTriggeredCallback
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let parameters = self.getParameters();
		Self::dropEventsAndCallbacks(parameters);
	}
}

impl<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback> Stream for UdpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback>
where ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback, SendEdgeTriggeredCallback: EdgeTriggeredCallback
{
	const Protocol: Layer4Protocol = Layer4Protocol::Udp;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(opaqueFfiHandle: *mut tle_stream) -> Self
	{
		UdpStream(opaqueFfiHandle, PhantomData, PhantomData, PhantomData, PhantomData)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _opaqueFfiHandle(&mut self) -> *mut tle_stream
	{
		self.0
	}
	
	#[inline(always)]
	fn close(&mut self)
	{
		let result = unsafe { ::dpdk_sys::tle_udp_stream_close(self._opaqueFfiHandle()) };

		if likely(result == 0)
		{
			return;
		}
		if unlikely(result > 0)
		{
			panic!("tle_udp_stream_close() returned a positive result");
		}
		match result
		{
			NegativeE::EINVAL => panic!("Invalid stream from tle_udp_stream_close()"),
		
			illegal @ _ => panic!("Unexpected errno '{}' from tle_udp_stream_close()", -illegal),
		}
	}
}

impl<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback> UdpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback>
where ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback, SendEdgeTriggeredCallback: EdgeTriggeredCallback
{
	#[inline(always)]
	pub fn openStream
	<
		IpV4: AddressLookUpForSendCallback<in_addr>,
		IpV6: AddressLookUpForSendCallback<in6_addr>
	>
	(
		context: &mut UdpContext<IpV4, IpV6>,
		localAddress: sockaddr_storage,
		remoteAddress: sockaddr_storage,
		receive: EventNotificationKind<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback>,
		send: EventNotificationKind<SendLevelTriggeredData, SendEdgeTriggeredCallback>
	) -> Option<UdpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback>>
	{
		assert!(false, "Need to fix EventNotificationKind to work correctly with lifetimes");
		
		let mut parameters = tle_udp_stream_param
		{
			local_addr: localAddress,
			remote_addr: remoteAddress,
			recv_ev: null_mut(),
			recv_cb: tle_stream_cb
			{
				func: None,
				data: null_mut()
			},
			send_ev: null_mut(),
			send_cb: tle_stream_cb
			{
				func: None,
				data: null_mut()
			},
		};
		
		match receive
		{
			EventNotificationKind::NoNotification => (),
			EventNotificationKind::LevelTriggeredEvent(event) => parameters.recv_ev = event.forget(),
			EventNotificationKind::EdgeTriggeredCallback(callback) => parameters.recv_cb = EdgeTriggeredCallback::_to_tle_stream_cb(callback),
		};
		
		match send
		{
			EventNotificationKind::NoNotification => (),
			EventNotificationKind::LevelTriggeredEvent(event) => parameters.send_ev = event.forget(),
			EventNotificationKind::EdgeTriggeredCallback(callback) => parameters.send_cb = EdgeTriggeredCallback::_to_tle_stream_cb(callback),
		};
				
		loop
		{
			let result = unsafe { ::dpdk_sys::tle_udp_stream_open(context._opaqueFfiHandle(), &parameters) };
			if unlikely(result.is_null())
			{
				match unsafe { rust_rte_errno() }
				{
					E::EAGAIN => continue,
					
					E::ENFILE =>
					{
						UdpStream::<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback>::dropEventsAndCallbacks(parameters);
						
						return None
					},
			
					E::EINVAL => panic!("Supplied an invalid value"),
			
					illegal @ _ => panic!("Unexpected errno '{}' from tle_udp_stream_open()", illegal),
				}
			}
			else
			{
				return Some(UdpStream::_new(result));
			}
		}
	}
	
	#[inline(always)]
	fn dropEventsAndCallbacks(parameters: tle_udp_stream_param)
	{
		Event::<ReceiveLevelTriggeredData>::dropEvent(parameters.recv_ev);
		ReceiveEdgeTriggeredCallback::_dropCallback(parameters.recv_cb);
		Event::<SendLevelTriggeredData>::dropEvent(parameters.send_ev);
		SendEdgeTriggeredCallback::_dropCallback(parameters.recv_cb);
	}
	
	#[inline(always)]
	pub fn receive(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16
	{
		return unsafe { ::dpdk_sys::tle_udp_stream_recv(self._opaqueFfiHandle(), pkt, num) }
	}
	
	#[inline(always)]
	pub fn send(&mut self, pkt: *mut *mut rte_mbuf, num: u16, dst_addr: *const sockaddr) -> u16
	{
		return unsafe { ::dpdk_sys::tle_udp_stream_send(self._opaqueFfiHandle(), pkt, num, dst_addr) }
	}
	
	#[inline(always)]
	fn getParameters(&mut self) -> tle_udp_stream_param
	{
		let mut parameters = unsafe { uninitialized() };
		
		let result = unsafe { ::dpdk_sys::tle_udp_stream_get_param(self._opaqueFfiHandle(), &mut parameters) };
		
		if likely(result == 0)
		{
			return parameters;
		}
		
		forget(parameters);
		
		if unlikely(result > 0)
		{
			panic!("tle_udp_stream_close() returned a positive result");
		}
		
		match result
		{
			NegativeE::EINVAL => panic!("Invalid stream from tle_udp_stream_get_param()"),
		
			illegal @ _ => panic!("Unexpected errno '{}' from tle_udp_stream_get_param()", -illegal),
		}
	}
}
