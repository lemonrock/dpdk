// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


macro_rules! drop_tcp_callbacks
{
	($parameters: expr, $error: expr) =>
	{
		{
			unsafe
			{
				drop(Box::from_raw($parameters.cfg.recv_cb as *mut ReceiveCallback));
				drop(Box::from_raw($parameters.cfg.send_cb as *mut SendCallback));
				drop(Box::from_raw($parameters.cfg.err_cb as *mut SendCallback));
			}
			Err($error)
		}
	}
}

/// An TCP stream.
///
/// When dropped, the stream is closed.
///
/// Note, we do not support `tle_tcp_stream_close_bulk` as it has no effective performance advantage.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TcpStream<ReceiveCallback: EdgeTriggeredEventCallback, SendCallback: EdgeTriggeredEventCallback, ErrorCallback: EdgeTriggeredEventCallback>(NonNull<tle_stream>, PhantomData<(ReceiveCallback, SendCallback, ErrorCallback)>);

impl<ReceiveCallback: EdgeTriggeredEventCallback, SendCallback: EdgeTriggeredEventCallback, ErrorCallback: EdgeTriggeredEventCallback> Drop for TcpStream<ReceiveCallback, SendCallback, ErrorCallback>
{
	// A handle to a stream is never actually freed, it's just recycled on close.
	#[inline(always)]
	fn drop(&mut self)
	{
		let mut parameters = self.get_parameters();

		let result = unsafe { tle_tcp_stream_close(self.handle()) };

		drop_tcp_callbacks!(parameters, ());

		match result
		{
			0 => (),

			NegativeE::EDEADLK => panic!("Stream already closed"),

			NegativeE::EINVAL => panic!("Invalid stream from tle_tcp_stream_close()"),

			negative if negative < 0 => panic!("Unexpected errno '{}' from tle_tcp_stream_close()", -negative),

			_ => panic!("tle_tcp_stream_close() returned a positive result"),
		}
	}
}

impl<ReceiveCallback: EdgeTriggeredEventCallback, SendCallback: EdgeTriggeredEventCallback, ErrorCallback: EdgeTriggeredEventCallback> Stream for TcpStream<ReceiveCallback, SendCallback, ErrorCallback>
{
	const Protocol: Layer4Protocol = Layer4Protocol::Tcp;

	#[inline(always)]
	fn maximum_segment_size(&self) -> u16
	{
		let maximum_segment_size = unsafe { tle_tcp_stream_get_mss(self.handle()) };
		debug_assert!(maximum_segment_size >= 0, "tle_tcp_stream_get_mss() should never fail");
		debug_assert!(maximum_segment_size <= ::std::u16::MAX_VALUE as u32, "TCP maximum segment size is too large");
		maximum_segment_size as u16
	}
}

impl<ReceiveCallback: EdgeTriggeredEventCallback, SendCallback: EdgeTriggeredEventCallback, ErrorCallback: EdgeTriggeredEventCallback> TcpStream<ReceiveCallback, SendCallback, ErrorCallback>
{
	/// Creates and opens a new TCP stream.
	///
	/// Returns an error on limit on number of streams reached.
	///
	/// If `number_of_retries` is zero (0) then a default (`TLE_TCP_DEFAULT_RETRIES`, currently 3) will be used instead.
	#[inline(always)]
	pub fn new<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>>(context: &mut TcpContext<IpV4, IpV6>, local_address: sockaddr_storage, remote_address: sockaddr_storage, receive: ReceiveCallback, send: SendCallback, error: ErrorCallback, number_of_retries: u8, server_or_client: TcpServerOrClient) -> Result<Self, TcpStreamCreationError>
	{
		use self::StreamCreationError::*;
		use self::TcpStreamCreationError::*;
		use self::TcpServerOrClient::*;

		let parameters = Self::parameters(local_address, remote_address, receive, send, error, number_of_retries);

		let tcp_stream_handle =
		{
			let result = unsafe { tle_tcp_stream_open(context.handle().as_ptr(), &parameters) };
			if likely(result.is_not_null())
			{
				unsafe { NonNull::new_unchecked(result) }
			}
			else
			{
				match unsafe { rust_rte_errno() }
				{
					E::EAGAIN => return drop_tcp_callbacks!(parameters, TryCreationAgain(BecauseThereIsOutstandingDataToSendOrReceiveOnTheStreamBeforeIsCanBeUsed)),

					E::ENFILE => return drop_tcp_callbacks!(parameters, TryCreationAgain(NoMoreStreamsAvailable)),

					E::EINVAL => panic!("Supplied an invalid value"),

					illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_stream_open()", illegal),
				}
			}
		};

		match server_or_client
		{
			Listen =>
			{
				// After listening, wait for a read event; use a ReceiveCallback; then call tle_tcp_stream_accept() from inside the read event
				// There is no send event.
				// After aceept(), call tle_tcp_stream_update_cfg() on each stream to set event callbacks.
				match unsafe { tle_tcp_stream_listen(tcp_stream_handle) }
				{
					0 => (),

					NegativeE::EDEADLK => panic!("Stream already closed"),

					NegativeE::EINVAL => panic!("Invalid value passed to tle_tcp_stream_listen()"),

					negative if negative < 0 => panic!("Unexpected errno '{}' from tle_tcp_stream_listen()", -negative),

					_ => panic!("tle_tcp_stream_listen() returned a positive result"),
				}
			}

			Connect { ref destination_ip_address_and_port } =>
			{
				// TODO: tle_tcp_stream_connect calls tle_tcp_stream_close if an error occurs
				// will raise a send event when connected and an error event if can not connect.
				match unsafe { tle_tcp_stream_connect(tcp_stream_handle, destination_ip_address_and_port) }
				{
					0 => (),

					NegativeE::EDEADLK => panic!("Stream already closed"),

					NegativeE::EINVAL => panic!("Invalid value passed to tle_tcp_stream_connect()"),

					illegal @ _ => panic!("Unexpected errno '{}' from tle_tcp_stream_connect()", -illegal),

					_ => panic!("tle_tcp_stream_connect() returned a positive result"),
				}
			}
		}

		Ok(TcpStream(tcp_stream_handle, PhantomData))
	}


	// read, readv, write, writev

	// #[inline(always)]
	// pub fn accept(&mut self, tcpStreams: ArrayVec<[Self; NumberOfTcpStreamsToAccept]>)
	// {
	// 	let numberOfTcpStreamsAccepted = unsafe { tle_tcp_stream_accept(self.handle(), tcpStreams.as_mut_ptr(), NumberOfTcpStreamsToAccept as u32) };
	// 	unsafe { tcpStreams.set_len(numberOfTcpStreamsAccepted as usize) };
	// }

	// After accept, aren't we going to have to do one of these to get callbacks / event handlers in place?
	/*
		// modifies the callbacks and TCP listen; presumably for a stream obtained via listen or after connect?
	pub fn tle_tcp_stream_update_cfg(ts: *mut *mut tle_stream, prm: *mut tle_tcp_stream_cfg, num: uint32_t) -> uint32_t;
	*/

	#[inline(always)]
	pub fn get_addresses(&self) -> tle_tcp_stream_addr
	{
		let mut addresses = unsafe { uninitialized() };

		let result = unsafe { tle_tcp_stream_get_addr(self.handle(), &mut addresses) };
		debug_assert_eq!(result, 0, "result should always be zero");

		addresses
	}

	#[inline(always)]
	pub fn receive(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16
	{
		return unsafe { tle_tcp_stream_recv(self.handle(), pkt, num) }
	}

	#[inline(always)]
	pub fn send(&mut self, pkt: *mut *mut rte_mbuf, num: u16) -> u16
	{
		return unsafe { tle_tcp_stream_send(self.handle(), pkt, num) }
	}

	unsafe extern "C" fn receive_callback(arg1: *mut c_void, arg2: *mut tle_stream)
	{
		debug_assert!(arg1.is_not_null(), "arg1 was null");
		debug_assert!(arg2.is_not_null(), "arg2 was null");

		let callback = &mut *(arg1 as *mut ReceiveCallback);
		callback.call(NonNull::new_unchecked(arg2));
	}

	unsafe extern "C" fn send_callback(arg1: *mut c_void, arg2: *mut tle_stream)
	{
		debug_assert!(arg1.is_not_null(), "arg1 was null");
		debug_assert!(arg2.is_not_null(), "arg2 was null");

		let callback = &mut *(arg1 as *mut SendCallback);
		callback.call(NonNull::new_unchecked(arg2));
	}

	unsafe extern "C" fn error_callback(arg1: *mut c_void, arg2: *mut tle_stream)
	{
		debug_assert!(arg1.is_not_null(), "arg1 was null");
		debug_assert!(arg2.is_not_null(), "arg2 was null");

		let callback = &mut *(arg1 as *mut ErrorCallback);
		callback.call(NonNull::new_unchecked(arg2));
	}

	#[inline(always)]
	fn parameters(local_address: sockaddr_storage, remote_address: sockaddr_storage, receive: Box<ReceiveCallback>, send: Box<SendCallback>, error: Box<ErrorCallback>, number_of_retries: u8) -> tle_tcp_stream_param
	{
		tle_tcp_stream_param
		{
			addr: tle_tcp_stream_addr
			{
				local: local_address,
				remote: remote_address,
			},
			cfg: tle_tcp_stream_cfg
			{
				nb_retries: number_of_retries,
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
				err_ev: null_mut(),
				err_cb: tle_stream_cb
				{
					func: Self::error_callback,
					data: Box::into_raw(error),
				},
			},
		}
	}

	xxxx
	#[inline(always)]
	fn get_parameters(&self) -> tle_tcp_stream_cfg
	{
		panic!("tle_tcp_stream_get_param is not yet implemented by TLDK");
	}

	#[inline(always)]
	fn handle(&self) -> *mut tle_stream
	{
		self.0.as_ptr()
	}
}
