// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


const TransmitBurstQueueDepth: usize = 64;

#[allow(missing_debug_implementations)]
pub struct TransmitBurstQueue
{
	transmitBurst: TransmitBurst,
	queue: [*mut rte_mbuf; TransmitBurstQueueDepth],
	count: usize,
	highestExclusiveIndex: usize,
}

impl Drop for TransmitBurstQueue
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let highestExclusiveIndex = self.highestExclusiveIndex;
		let mut index = highestExclusiveIndex - self.count;
		let mut queue = self.queue;
		while index < highestExclusiveIndex
		{
			(unsafe { *queue.get_unchecked_mut(index) }).free();
			index += 1;
		}
		
		self.queue = unsafe { zeroed() };
	}
}

macro_rules! makeCapacityAvailable
{
	($self: ident) =>
	{
		{
			// If no capacity available, can we compress?
			if likely($self.canBeCompressed())
			{
				$self.compressUnchecked();
			}
			else
			{
				// If we can't compress, transmit
				if likely($self.transmitAndReportIfAllPacketsTransmittedUnchecked())
				{
				}
				// If we did not transmit everything, is there space for us with compression?
				else if likely($self.canBeCompressed())
				{
					$self.compressUnchecked();
				}
				// This should only be the case if NOTHING was transmitted; the caller can only sleep and try again
				else
				{
					return false;
				}
			}
			$self.capacityAvailable()
		}
	}
}

// A macro is used rather than a function as we do not want to pass a function pointer
macro_rules! fillFromTldkDeviceTransmittingAsRequired
{
	($self: ident, $device: ident, $tldkBulkTransmitFunction: path) =>
	{
		{
			let mut capacityAvailable = $self.capacityAvailable();
			
			if unlikely(capacityAvailable == 0)
			{
				capacityAvailable = makeCapacityAvailable!($self);
			}
			
			loop
			{
				let enqueuedCount = unsafe { $tldkBulkTransmitFunction($device.handle(), $self.queue.get_unchecked_mut($self.highestExclusiveIndex), capacityAvailable as u16) } as usize;
				debug_assert!(enqueuedCount <= capacityAvailable, "enqueueCount '{}' exceeded capacityAvailable '{}'", enqueuedCount, capacityAvailable);
				
				$self.highestExclusiveIndex += enqueuedCount;
				$self.count += enqueuedCount;
				
				// We keep asking for more from TLDK until it no longer fills us up completely
				if enqueuedCount < capacityAvailable
				{
					return true;
				}
				capacityAvailable = makeCapacityAvailable!($self);
			}
		}
	}
}

impl TransmitBurstQueue
{
	#[inline(always)]
	pub fn new(transmitBurst: TransmitBurst) -> Self
	{
		const MaximumTransmitBurstQueueDepth: usize = 65535;
		
		debug_assert!(TransmitBurstQueueDepth != 0, "TransmitBurstQueueDepth can not be zero");
		debug_assert!(TransmitBurstQueueDepth <= MaximumTransmitBurstQueueDepth, "TransmitBurstQueueDepth '{}' is more than the maximum, MaximumTransmitBurstQueueDepth '{}'", TransmitBurstQueueDepth, MaximumTransmitBurstQueueDepth);
		
		Self
		{
			transmitBurst,
			queue: unsafe { uninitialized() },
			count: 0,
			highestExclusiveIndex: 0,
		}
	}
	
	#[inline(always)]
	pub fn pushTransmittingAsRequired(&mut self, packet: *mut rte_mbuf) -> Result<(), *mut rte_mbuf>
	{
		debug_assert!(!packet.is_null(), "packet is null");
		
		if likely(self.hasCapacityAvailableWithoutCompression())
		{
			self.insertUnchecked(packet)
		}
		else
		{
			// If not, can we compress?
			if likely(self.canBeCompressed())
			{
				self.compressUnchecked();
				self.insertUnchecked(packet)
			}
			// If not, can we transmit to make space?
			else
			{
				// Transmit and in all likelihood transmit everything
				if likely(self.transmitAndReportIfAllPacketsTransmittedUnchecked())
				{
					self.insertUnchecked(packet)
				}
				// If not, is there space for us with compression?
				else if likely(self.canBeCompressed())
				{
					self.compressUnchecked();
					self.insertUnchecked(packet)
				}
				// This should only be the case if NOTHING was transmitted; the caller can either sleep and try again or discard the packet
				else
				{
					Err(packet)
				}
			}
		}
	}
	
	// true if managed to fill something; false only if can not (in essence, we are completely full and we did not transmit anything when we tried)
	#[inline(always)]
	pub fn fillFromTldkTcpDeviceTransmittingAsRequired(&mut self, mut tcpDevice: TcpDevice) -> bool
	{
		fillFromTldkDeviceTransmittingAsRequired!(self, tcpDevice, tle_tcp_tx_bulk)
	}
	
	// true if managed to fill something; false only if can not (in essence, we are completely full and we did not transmit anything when we tried)
	#[inline(always)]
	pub fn fillFromTldkUdpDeviceTransmittingAsRequired(&mut self, mut udpDevice: UdpDevice) -> bool
	{
		fillFromTldkDeviceTransmittingAsRequired!(self, udpDevice, tle_udp_tx_bulk)
	}
	
	#[inline(always)]
	pub fn transmitAndReportIfAllPacketsTransmitted(&mut self) -> bool
	{
		if unlikely(self.count == 0)
		{
			return true;
		}
		
		self.transmitAndReportIfAllPacketsTransmittedUnchecked()
	}
	
	#[inline(always)]
	fn capacityAvailable(&self) -> usize
	{
		TransmitBurstQueueDepth - self.highestExclusiveIndex
	}
	
	#[inline(always)]
	fn transmitAndReportIfAllPacketsTransmittedUnchecked(&mut self) -> bool
	{
		let count = self.count;
		let index = self.lowestOffset();
		let numberTransmitted = self.transmitBurst.transmit(unsafe { self.queue.get_unchecked_mut(index) }, count);
		
		if likely(numberTransmitted == count)
		{
			self.count = 0;
			self.highestExclusiveIndex = 0;
			true
		}
		else
		{
			self.count = count - numberTransmitted;
			false
		}
	}
	
	#[inline(always)]
	fn insertUnchecked(&mut self, packet: *mut rte_mbuf) -> Result<(), *mut rte_mbuf>
	{
		unsafe { *self.queue.get_unchecked_mut(self.highestExclusiveIndex) = packet };
		//self.queue[self.highestExclusiveIndex] = packet;
		self.highestExclusiveIndex += 1;
		self.count += 1;
		Ok(())
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn compressUnchecked(&mut self)
	{
		unsafe
		{
			let destination = self.queue.get_unchecked_mut(0) as *mut _;
			let source = destination.offset(self.lowestOffset() as isize);
			let numberToMove = self.count;
			copy(source, destination, numberToMove);
		}
		
		self.highestExclusiveIndex = self.count;
	}
	
	#[inline(always)]
	fn canBeCompressed(&self) -> bool
	{
		self.count != TransmitBurstQueueDepth
	}
	
	#[inline(always)]
	fn lowestOffset(&self) -> usize
	{
		self.highestExclusiveIndex - self.count
	}
	
	#[inline(always)]
	fn hasCapacityAvailableWithoutCompression(&self) -> bool
	{
		self.highestExclusiveIndex != TransmitBurstQueueDepth
	}
}
