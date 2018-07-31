// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a packet reordering buffer.
#[derive(Debug)]
pub struct PacketReorderer
{
	reorder_buffer: NonNull<rte_reorder_buffer>,
	we_should_destroy_the_reorder_buffer_on_drop: bool,
}

impl Drop for PacketReorderer
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.we_should_destroy_the_reorder_buffer_on_drop
		{
			unsafe { rte_reorder_free(self.handle()) }
		}
	}
}

impl PacketReorderer
{
	/// Find an existing instance.
	#[inline(always)]
	pub fn new(name: &CStr, numa_node_choice: NumaNodeChoice, maximum_number_of_elements: u32) -> Option<Self>
	{
		let result = unsafe { rte_reorder_create(name.as_ptr(), numa_node_choice.into(), maximum_number_of_elements) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					reorder_buffer: unsafe { NonNull::new_unchecked(result) },
					we_should_destroy_the_reorder_buffer_on_drop: true,
				}
			)
		}
	}
	
	/// Find an existing instance.
	#[inline(always)]
	pub fn find_existing(name: &CStr) -> Option<Self>
	{
		let result = unsafe { rte_reorder_find_existing(name.as_ptr()) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					reorder_buffer: unsafe { NonNull::new_unchecked(result) },
					we_should_destroy_the_reorder_buffer_on_drop: false,
				}
			)
		}
	}
	
	/// Resets this this instance with initial values.
	#[inline(always)]
	pub fn reset(&self)
	{
		unsafe { rte_reorder_reset(self.handle()) }
	}
	
	/// Inserts given `packet_buffer` in this reorder buffer in its correct position.
	///
	/// The given `packet_buffer` is to be reordered relative to other `packet_buffer`s in the system.
	/// The `packet_buffer` must contain a sequence number which is then used to place it in the correct position in the reorder buffer.
	/// Reordered packets can later be taken from the reorder buffer using `self.drain()`.
	#[inline(always)]
	pub fn insert(&self, packet_buffer: NonNull<rte_mbuf>) -> Result<(), CouldNotInsertPacketBufferForReordering>
	{
		let result = unsafe { rte_reorder_insert(self.handle(), packet_buffer.as_ptr()) };
		if likely!(result == 0)
		{
			return Ok(())
		}
		
		debug_assert_eq!(result, -1, "result '{}' was not 0 or -1", result);
		
		use self::CouldNotInsertPacketBufferForReordering::*;
		
		match LogicalCore::current_logical_core_error_number()
		{
			E::ENOSPC => Err(CanNotMoveExistingPacketBuffersToAcccommodateUnlessDrainIsCalled),
			E::ERANGE => Err(OutOfWindowRange),
			
			unexpected @ _ => panic!("Unexpected error code '{}' from rte_reorder_insert()", unexpected),
		}
	}
	
	/// Fetch reordered packet buffers.
	///
	/// Returns a set of in-order buffers from the reorder buffer structure.
	/// Gaps may be present in the sequence numbers of the packet buffers if packets have been delayed too long before reaching the reorder window, or have been previously dropped by the system.
	#[inline(always)]
	pub fn drain<A: NonNullUnifiedArrayVecAndVec<rte_mbuf>>(&self, packets_into: &mut A)
	{
		let (pointer, number_of_packets) = packets_into.from_ffi_data_u32();
		let number_written = unsafe { rte_reorder_drain(self.handle(), pointer, number_of_packets) };
		packets_into.set_length(number_written as usize)
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_reorder_buffer
	{
		self.reorder_buffer.as_ptr()
	}
}
