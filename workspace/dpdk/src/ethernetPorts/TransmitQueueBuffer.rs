// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmitQueueBuffer<E>
{
	buffer: DpdkAllocatedMemory<rte_eth_dev_tx_buffer>,
	queueIdentifier: QueueIdentifier,
	portIdentifier: EthernetPortIdentifier,
	errorCallback: Option<E>,
}

impl<E: TransmitQueueBufferErrorCallback> TransmitQueueBuffer<E>
{
	#[inline(always)]
	fn as_rte_eth_dev_tx_buffer(&self) -> *mut rte_eth_dev_tx_buffer
	{
		self.buffer.0
	}
	
	const_cstr!
	{
		tx_buffer = "tx_buffer";
	}
	
	// maximumPacketBurst is something like 32
	#[inline(always)]
	pub fn newWithoutCallback(transmitQueue: TransmitQueue, maximumPacketBurst: u16) -> Option<TransmitQueueBuffer<NoTransmitQueueBufferErrorCallback>>
	{
		TransmitQueueBuffer::<NoTransmitQueueBufferErrorCallback>::new(transmitQueue, maximumPacketBurst, None)
	}
	
	// maximumPacketBurst is something like 32
	#[inline(always)]
	pub fn new(transmitQueue: TransmitQueue, maximumPacketBurst: u16, errorCallback: Option<E>) -> Option<TransmitQueueBuffer<E>>
	{
		let SIZE = unsafe { rust_RTE_ETH_TX_BUFFER_SIZE(maximumPacketBurst) };
		let buffer = transmitQueue.numaSocketId.zeroAllocate(Some(Self::tx_buffer), SIZE, None);
		if buffer.is_none()
		{
			return None;
		}
		let buffer = buffer.unwrap();
		
		let result = unsafe { ::dpdk_sys::rte_eth_tx_buffer_init(buffer.0, maximumPacketBurst) };
		if likely(result == 0)
		{
			let mut transmitQueueBuffer = TransmitQueueBuffer
			{
				portIdentifier: transmitQueue.portIdentifier,
				queueIdentifier: transmitQueue.queueIdentifier,
				buffer: buffer,
				errorCallback: None,
			};
			
			if let Some(errorCallback) = errorCallback
			{
				transmitQueueBuffer.setErrorCallback(errorCallback);
			}
			
			Some(transmitQueueBuffer)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn flush(&self) -> u16
	{
		unsafe { rust_rte_eth_tx_buffer_flush(self.portIdentifier, self.queueIdentifier, self.as_rte_eth_dev_tx_buffer()) }
	}
	
	#[inline(always)]
	pub fn buffer(&self, packet: *mut rte_mbuf) -> u16
	{
		unsafe { rust_rte_eth_tx_buffer(self.portIdentifier, self.queueIdentifier, self.as_rte_eth_dev_tx_buffer(), packet) }
	}
	
	#[inline(always)]
	pub fn setErrorCallback(&mut self, mut errorCallback: E)
	{
		let result = unsafe { ::dpdk_sys::rte_eth_tx_buffer_set_err_callback(self.as_rte_eth_dev_tx_buffer(), E::asFunctionPointer(), errorCallback.asFunctionArgument()) };
		if likely(result == 0)
		{
			// Take ownership; when we get dropped, errorCallback gets dropped
			self.errorCallback = Some(errorCallback);
		}
		else
		{
			match result
			{
				-1 => panic!("rte_eth_tx_buffer_set_err_callback() failed with error number '{}'", unsafe { rust_rte_errno() }),
				
				_ => panic!("rte_eth_tx_buffer_set_err_callback() returned an invalid result '{}'", result),
			}
		}
	}
}
