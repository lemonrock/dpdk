// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


const ReceiveBurstBufferDepth: usize = 64;

#[allow(missing_debug_implementations)]
pub struct ReceiveBurstBuffer<D: Device>
{
	device: D,
	buffer: [*mut rte_mbuf; ReceiveBurstBufferDepth],
	nextIndex: usize,
}

impl<D: Device> Drop for ReceiveBurstBuffer<D>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if unlikely(self.nextIndex == 0)
		{
			return;
		}
		
		for index in 0..self.nextIndex
		{
			(unsafe { *self.buffer.get_unchecked_mut(index)}).free();
		}
		
		self.buffer = unsafe { zeroed() };
	}
}

impl<D: Device> ReceiveBurstBuffer<D>
{
	pub fn new(device: D) -> Self
	{
		debug_assert!(ReceiveBurstBufferDepth <= 65_535, "ReceiveBurstBufferDepth '{}' is too large", ReceiveBurstBufferDepth);
		
		Self
		{
			device: device,
			buffer: unsafe { uninitialized() },
			nextIndex: 0,
		}
	}
	
	#[inline(always)]
	pub fn bufferAndSendToTldkWhenFull(&mut self, packet: *mut rte_mbuf)
	{
		if unlikely(self.nextIndex == ReceiveBurstBufferDepth)
		{
			if unlikely(self.sendToTldkUnchecked())
			{
				// We are still full; drop the packet
				packet.free();
				return;
			}
		}
		
		unsafe
		{
			*self.buffer.get_unchecked_mut(self.nextIndex) = packet;
		}
		self.nextIndex += 1;
	}
	
	#[inline(always)]
	pub fn sendToTldk(&mut self)
	{
		if unlikely(self.nextIndex == 0)
		{
			return
		}
		
		self.sendToTldkUnchecked();
	}
	
	// true if still full after send (Houston, we have a problem)
	#[inline(always)]
	fn sendToTldkUnchecked(&mut self) -> bool
	{
		let mut rp: [*mut rte_mbuf; ReceiveBurstBufferDepth] = unsafe { uninitialized() };
		let mut rc: [i32; ReceiveBurstBufferDepth] = unsafe { uninitialized() };
		let count = self.nextIndex;
		
		let numberAccepted =
		{
			let pkt = unsafe { self.buffer.get_unchecked_mut(0) };
			self.device.bulkReceive(pkt, rp.as_mut_ptr(), rc.as_mut_ptr(), count as u16) as usize
		};
		
		if likely(numberAccepted == count)
		{
			self.nextIndex = 0;
			false
		}
		else
		{
			// Process rejected packets
			let numberRejected = count - numberAccepted;
			let mut nextIndex = 0;
			for index in 0..numberRejected
			{
				let rejectedPacket = unsafe { *rp.get_unchecked_mut(index) };
				if unsafe { *rc.get_unchecked_mut(index) } == E::ENOBUFS
				{
					// Something, somewhere is full (eg UDP rx stream ring)
					// Try again later
					unsafe { *self.buffer.get_unchecked_mut(nextIndex)  = rejectedPacket };
					nextIndex += 1;
				}
				else
				{
					// Documented: ENOENT => No stream to send packet to
					// Undocumented: EINVAL => There is a stream to send the packet to but the packet is bad
					rejectedPacket.free()
				}
			}
			self.nextIndex = nextIndex;
			
			forget(rc);
			forget(rp);
			
			// Report if full after trying to send, ie all packets were rejected
			nextIndex == ReceiveBurstBufferDepth
		}
	}
}
