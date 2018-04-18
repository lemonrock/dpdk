// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



pub type PacketBufferPool = NonNull<rte_mempool>;

pub trait PacketBufferPoolExt
{

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketBufferPool(pub *mut rte_mempool);

pub const BulkAllocate: u32 = 32;
const BulkAllocateUsize: usize = BulkAllocate as usize;

impl PacketBufferPool
{
	#[inline(always)]
	pub fn new(memoryZoneName: &str, numberOfElements: u32, perCoreObjectCacheSize: u32, applicationPrivateSize: u16, dataRoomSize: u16, numa_socket_id: Option<NumaSocketId>) -> Option<PacketBufferPool>
	{
		// rte_pktmbuf_pool_create takes a copy, so this doesn't need to hang around. Phew!
		let memoryZoneName = CString::new(memoryZoneName).expect("memoryZoneName contained an interior ASCII NUL");

		let result = unsafe { rte_pktmbuf_pool_create(memoryZoneName.as_ptr(), numberOfElements, perCoreObjectCacheSize, applicationPrivateSize, dataRoomSize, numa_socket_id.as_c_int()) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOSPC => None,
				E::ENOMEM => None,

				E_RTE::NO_CONFIG => panic!("No config"),
				E_RTE::SECONDARY => panic!("Secondary process"),
				E::EINVAL => panic!("cache size provided is too large, or priv_size is not aligned"),
				E::EEXIST => panic!("a memzone with the same name already exists"),

				illegal @ _ => panic!("Unexpected error code '{}' set by rte_pktmbuf_pool_create()", illegal),
			}
		}
		else
		{
			Some(PacketBufferPool(result))
		}
	}

	#[inline(always)]
	pub fn memoryPool(&self) -> *mut rte_mempool
	{
		self.0
	}

	#[inline]
	pub fn dataRoomSize(&self) -> u16
	{
		unsafe { rust_rte_pktmbuf_data_room_size(self.memoryPool()) }
	}

	#[inline]
	pub fn allocate(&self) -> Option<PacketBuffer>
	{
		let result = unsafe { rust_rte_pktmbuf_alloc(self.memoryPool()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(PacketBuffer(result))
		}
	}

	#[inline(always)]
	pub fn bulkAllocate(&self) -> Option<[PacketBuffer; BulkAllocateUsize]>
	{
		let mut mbufs: [*mut rte_mbuf; BulkAllocateUsize] = unsafe { uninitialized() };
		let result = unsafe { rust_rte_pktmbuf_alloc_bulk(self.memoryPool(), mbufs.as_mut_ptr(), BulkAllocate) };
		if likely(result == 0)
		{
			Some(unsafe { transmute(mbufs) })
		}
		else
		{
			forget(mbufs);
			None
		}
	}
}
