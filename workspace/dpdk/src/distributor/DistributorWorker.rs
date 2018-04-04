// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct DistributorWorker
{
	distributor: Arc<Distributor>,
	distributorPointer: *mut rte_distributor, // Optimisation over self.distributor.deref().as_mut_ptr()
	identifier: u32, // Actually, no bigger than u8 but we pass it to FFI as u32
}

unsafe impl Send for DistributorWorker
{
}

unsafe impl Sync for DistributorWorker
{
}

impl DistributorWorker
{
	#[inline(always)]
	pub fn new(distributor: &Arc<Distributor>, identifier: u8) -> DistributorWorker
	{
		debug_assert!(identifier < Distributor::RTE_DISTRIB_MAX_WORKERS, "identifier '{}' is larger than (or equal to) the maximum '{}'", identifier, Distributor::RTE_DISTRIB_MAX_WORKERS);
		
		let clone = distributor.clone();
		let pointer = clone.as_mut_ptr();
		
		DistributorWorker
		{
			distributor: clone,
			distributorPointer: pointer,
			identifier: identifier as u32
		}
	}
	
	#[inline(always)]
	pub fn getPacketButNotAfterRequestPacket(&self, previousPacket: Option<*mut rte_mbuf>) -> *mut rte_mbuf
	{
		unsafe { ::dpdk_sys::rte_distributor_get_pkt(self.distributorPointer, self.identifier, Self::optionalPacketToPointer(previousPacket)) }
	}
	
	/// Used instead of getPacket if no more packets are wanted, eg if the worker is shutting down
	#[inline(always)]
	pub fn returnPacket(&self, previousPacket: Option<*mut rte_mbuf>)
	{
		match unsafe { ::dpdk_sys::rte_distributor_return_pkt(self.distributorPointer, self.identifier, Self::optionalPacketToPointer(previousPacket)) }
		{
			0 => (),

			// returns an int but documentation makes no mention of it
			unexpected @ _ => panic!("Unexpected result '{}' from rte_distributor_return_pkt()", unexpected)
		}
	}
	
	#[inline(always)]
	pub fn requestPacket(&self, previousPacket: Option<*mut rte_mbuf>)
	{
		unsafe { ::dpdk_sys::rte_distributor_request_pkt(self.distributorPointer, self.identifier, Self::optionalPacketToPointer(previousPacket)) }
	}
	
	#[inline(always)]
	pub fn getPacketAfterRequestPacket(&self) -> Option<*mut rte_mbuf>
	{
		let result = unsafe { ::dpdk_sys::rte_distributor_poll_pkt(self.distributorPointer, self.identifier) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some(result)
		}
	}
	
	#[inline(always)]
	fn optionalPacketToPointer(packet: Option<*mut rte_mbuf>) -> *mut rte_mbuf
	{
		if unlikely(packet.is_none())
		{
			null_mut()
		}
		else
		{
			packet.unwrap()
		}
	}
}
