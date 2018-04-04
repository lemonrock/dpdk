// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// We do 2^15, the largest possible power of 2, as numberOfTcpStreamsAccepted is an uint16_t
// const NumberOfTcpStreamsToAccept: usize = 32768;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TcpStream
<
	ReceiveLevelTriggeredData,
	ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback,
	SendLevelTriggeredData,
	SendEdgeTriggeredCallback: EdgeTriggeredCallback,
	ErrorLevelTriggeredData,
	ErrorEdgeTriggeredCallback: EdgeTriggeredCallback
>
(
	*mut tle_stream,
	PhantomData<ReceiveLevelTriggeredData>,
	PhantomData<ReceiveEdgeTriggeredCallback>,
	PhantomData<SendLevelTriggeredData>,
	PhantomData<SendEdgeTriggeredCallback>,
	PhantomData<ErrorLevelTriggeredData>,
	PhantomData<ErrorEdgeTriggeredCallback>
);

impl<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback> Drop for TcpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback>
where ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback, SendEdgeTriggeredCallback: EdgeTriggeredCallback, ErrorEdgeTriggeredCallback: EdgeTriggeredCallback
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let cfg = self.getCfg();
		Self::dropEventsAndCallbacks(cfg);
	}
}

impl<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback> Stream for TcpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback>
where ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback, SendEdgeTriggeredCallback: EdgeTriggeredCallback, ErrorEdgeTriggeredCallback: EdgeTriggeredCallback
{
	const Protocol: Layer4Protocol = Layer4Protocol::Tcp;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(opaqueFfiHandle: *mut tle_stream) -> Self
	{
		TcpStream(opaqueFfiHandle, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _opaqueFfiHandle(&mut self) -> *mut tle_stream
	{
		self.0
	}
	
	// tle_tcp_stream_close can be called by TLDK in tle_tcp_stream_connect()
	#[inline(always)]
	fn close(&mut self)
	{
		let result = unsafe { ::dpdk_sys::tle_tcp_stream_close(self._opaqueFfiHandle()) };

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
			NegativeE::EDEADLK => panic!("Stream already closed from tle_udp_stream_close()"),
			NegativeE::EINVAL => panic!("Invalid stream from tle_udp_stream_close()"),
		
			illegal @ _ => panic!("Unexpected errno '{}' from tle_udp_stream_close()", -illegal),
		}
	}
}

impl<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback> TcpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback>
where ReceiveEdgeTriggeredCallback: EdgeTriggeredCallback, SendEdgeTriggeredCallback: EdgeTriggeredCallback, ErrorEdgeTriggeredCallback: EdgeTriggeredCallback
{
	pub const DefaultMaximumRetries: u8 = ::dpdk_sys::TLE_TCP_DEFAULT_RETRIES;
	
	#[inline(always)]
	pub fn openStream
	<
		IpV4: AddressLookUpForSendCallback<in_addr>,
		IpV6: AddressLookUpForSendCallback<in6_addr>
	>
	(
		context: &mut TcpContext<IpV4, IpV6>,
		localAddress: sockaddr_storage,
		remoteAddress: sockaddr_storage,
		receive: EventNotificationKind<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback>,
		send: EventNotificationKind<SendLevelTriggeredData, SendEdgeTriggeredCallback>,
		error: EventNotificationKind<ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback>,
		numberOfRetries: u8
	) -> Option<TcpStream<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback>>
	{
		assert!(false, "Need to fix EventNotificationKind to work correctly with lifetimes");
		
		let mut cfg = tle_tcp_stream_cfg
		{
			nb_retries: numberOfRetries,
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
			err_ev: null_mut(),
			err_cb: tle_stream_cb
			{
				func: None,
				data: null_mut()
			},
		};
		
		match receive
		{
			EventNotificationKind::NoNotification => (),
			EventNotificationKind::LevelTriggeredEvent(event) => cfg.recv_ev = event.forget(),
			EventNotificationKind::EdgeTriggeredCallback(callback) => cfg.recv_cb = EdgeTriggeredCallback::_to_tle_stream_cb(callback),
		};
		
		match send
		{
			EventNotificationKind::NoNotification => (),
			EventNotificationKind::LevelTriggeredEvent(event) => cfg.send_ev = event.forget(),
			EventNotificationKind::EdgeTriggeredCallback(callback) => cfg.send_cb = EdgeTriggeredCallback::_to_tle_stream_cb(callback),
		};
		
		match error
		{
			EventNotificationKind::NoNotification => (),
			EventNotificationKind::LevelTriggeredEvent(event) => cfg.err_ev = event.forget(),
			EventNotificationKind::EdgeTriggeredCallback(callback) => cfg.err_cb = EdgeTriggeredCallback::_to_tle_stream_cb(callback),
		};
		
		let parameters = tle_tcp_stream_param
		{
			addr: tle_tcp_stream_addr
			{
				local: localAddress,
				remote: remoteAddress,
			},
			cfg: cfg,
		};
		
		loop
		{
			let result = unsafe { ::dpdk_sys::tle_tcp_stream_open(context._opaqueFfiHandle(), &parameters) };
			if unlikely(result.is_null())
			{
				match unsafe { rust_rte_errno() }
				{
					E::EAGAIN => continue,
					
					E::ENFILE =>
					{
						TcpStream::<ReceiveLevelTriggeredData, ReceiveEdgeTriggeredCallback, SendLevelTriggeredData, SendEdgeTriggeredCallback, ErrorLevelTriggeredData, ErrorEdgeTriggeredCallback>::dropEventsAndCallbacks(cfg);
						
						return None
					},
			
					E::EINVAL => panic!("Supplied an invalid value"),
			
					illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_stream_open()", illegal),
				}
			}
			else
			{
				return Some(TcpStream::_new(result));
			}
		}
	}
	
	#[inline(always)]
	fn dropEventsAndCallbacks(cfg: tle_tcp_stream_cfg)
	{
		Event::<ReceiveLevelTriggeredData>::dropEvent(cfg.recv_ev);
		ReceiveEdgeTriggeredCallback::_dropCallback(cfg.recv_cb);
		Event::<SendLevelTriggeredData>::dropEvent(cfg.send_ev);
		SendEdgeTriggeredCallback::_dropCallback(cfg.send_cb);
		Event::<ErrorLevelTriggeredData>::dropEvent(cfg.err_ev);
		ErrorEdgeTriggeredCallback::_dropCallback(cfg.err_cb);
	}
	
	#[inline(always)]
	fn getCfg(&mut self) -> tle_tcp_stream_cfg
	{
		panic!("tle_tcp_stream_get_param is not yet implemented by TLDK");
	}
	
	#[inline(always)]
	pub fn receive(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16
	{
		return unsafe { ::dpdk_sys::tle_tcp_stream_recv(self._opaqueFfiHandle(), pkt, num) }
	}
	
	#[inline(always)]
	pub fn send(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16
	{
		return unsafe { ::dpdk_sys::tle_tcp_stream_send(self._opaqueFfiHandle(), pkt, num) }
	}
	
	// After listening, wait for a read event; use a ReceiveEdgeTriggeredCallback; then call tle_tcp_stream_accept() from inside the read event
	#[inline(always)]
	pub fn listen(&mut self)
	{
		let result = unsafe { ::dpdk_sys::tle_tcp_stream_listen(self._opaqueFfiHandle()) };
		if likely(result == 0)
		{
			return;
		}

		if unlikely(result > 0)
		{
			panic!("tle_tcp_stream_listen() returned a positive result");
		}
		
		match result
		{
			NegativeE::EDEADLK => panic!("Deadlock detected when creating TCP stream connect"),
			
			NegativeE::EINVAL => panic!("Invalid value passed to tle_tcp_stream_listen()"),
		
			illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_stream_listen()", -illegal),
		}
	}
	
	// #[inline(always)]
	// pub fn accept(&mut self, tcpStreams: ArrayVec<[Self; NumberOfTcpStreamsToAccept]>)
	// {
	// 	let numberOfTcpStreamsAccepted = unsafe { ::dpdk_sys::tle_tcp_stream_accept(self._opaqueFfiHandle(), tcpStreams.as_mut_ptr(), NumberOfTcpStreamsToAccept as u32) };
	// 	unsafe { tcpStreams.set_len(numberOfTcpStreamsAccepted as usize) };
	// }
	
	// After accept, aren't we going to have to do one of these to get callbacks / event handlers in place?
	/*
	pub fn tle_tcp_stream_get_addr(s: *const tle_stream, addr: *mut tle_tcp_stream_addr) -> c_int;
	pub fn tle_tcp_stream_update_cfg(ts: *mut *mut tle_stream, prm: *mut tle_tcp_stream_cfg, num: uint32_t) -> uint32_t;
	*/
	
	// tle_tcp_stream_connect calls tle_tcp_stream_close if an error occurs
	#[inline(always)]
	pub fn connect(&mut self, destinationIpAddressAndPort: &sockaddr)
	{
		let result = unsafe { ::dpdk_sys::tle_tcp_stream_connect(self._opaqueFfiHandle(), destinationIpAddressAndPort) };
		if likely(result == 0)
		{
			return;
		}

		if unlikely(result > 0)
		{
			panic!("tle_tcp_stream_connect() returned a positive result");
		}
		
		match result
		{
			NegativeE::EDEADLK => panic!("Deadlock detected when creating TCP stream connect"),
			
			NegativeE::EINVAL => panic!("Invalid value passed to tle_tcp_stream_connect()"),
		
			illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_stream_connect()", -illegal),
		}
	}
	
	#[inline(always)]
	pub fn bulkClose(mut tcpStreams: ArrayVec<[Self; 4096]>) -> (ArrayVec<[Self; 4096]>, Result<(), usize>)
	{
		let length = tcpStreams.len() as u32;
		
		let numberClosed = unsafe { ::dpdk_sys::tle_tcp_stream_close_bulk(tcpStreams.as_mut_ptr() as *mut *mut tle_stream, length) };
		if likely(numberClosed == length)
		{
			tcpStreams.clear();
			(tcpStreams, Ok(()))
		}
		else
		{
			match unsafe { rust_rte_errno() }
			{
				E::EDEADLK => (tcpStreams, Err(numberClosed as usize)),
		
				E::EINVAL => panic!("Supplied an invalid value"),
		
				illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_stream_close_bulk()", illegal),
			}
		}
	}
}
