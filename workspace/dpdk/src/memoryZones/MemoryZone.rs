// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Wraps a DPDK memory zone.
///
/// If constructed using any of the methods in this struct apart from `look_up()`, will be freed on drop.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryZone
{
	name: ConstCStr,
	handle: NonNull<rte_memzone>,
	free_on_drop: bool,
}

impl Drop for Memory
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.free_on_drop
		{
			match unsafe { rte_memzone_free(self.handle.as_ptr() as *const _) }
			{
				0 => (),
				
				// Can not drop an IVSHMEM memory zone
				NegativeE::EINVAL => (),

				illegal @ _ => panic!("Unexpected result '{}' from rte_memzone_free()", illegal),
			}
		}
	}
}

impl PrintInformation for MemoryZone
{
	#[inline(always)]
	fn print_information_to_stream(stream: *mut FILE)
	{
		unsafe { rte_memzone_dump(stream) };
	}
}

impl MemoryZone
{
	/// Special sentinel value used by DPDK for longest possible reservation.
	pub const SpecialLengthSignifyingLongestPossibleReservation: usize = 0;
	
	/// Find a memory zone using its name.
	#[inline(always)]
	pub fn look_up(name: ConstCStr) -> Option<MemoryZone>
	{
		Self::assert_memory_zone_name_size(name);

		let result = unsafe { rte_memzone_lookup(name.as_ptr()) };
		if unlikely(result.is_null())
		{
			None
		}
		else
		{
			Some(MemoryZone(name, result, false))
		}
	}
	
	/// Reserve the longest possible memory zone with `alignment` set to cache line size (`RTE_CACHE_LINE_SIZE`).
	#[inline(always)]
	pub fn reserve_with_alignment_as_cache_line_size_longest_possible(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memory_zone_reservation_page_choice: MemoryZoneReservationPageChoice) -> Option<MemoryZone>
	{
		Self::reserve_with_alignment_as_cache_line_size(name, numa_socket_id, memory_zone_reservation_page_choice, Self::SpecialLengthSignifyingLongestPossibleReservation)
	}
	
	/// Reserve a memory zone with `alignment` set to cache line size (`RTE_CACHE_LINE_SIZE`).
	///
	/// Note: 0 for `length` is special; it allocates the longest possible item of memory: see `Self::SpecialLengthSignifyingLongestPossibleReservation`. Rather than specifying length as `0`, use `Self::reserve_with_alignment_as_cache_line_size_longest_possible` instead as it documents intention better.
	#[inline(always)]
	pub fn reserve_with_alignment_as_cache_line_size(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memory_zone_reservation_page_choice: MemoryZoneReservationPageChoice, length: usize) -> Option<MemoryZone>
	{
		Self::assert_memory_zone_name_size(name);

		let result = unsafe { rte_memzone_reserve(name.as_ptr(), length, numa_socket_id.as_c_int(), memory_zone_reservation_page_choice.bits()) };
		Self::process_reservation_result(name, result, "rte_memzone_reserve")
	}
	
	/// Reserve the longest possible memory zone with `alignment` specified.
	#[inline(always)]
	pub fn reserve_with_alignment_longest_possible(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memory_zone_reservation_page_choice: MemoryZoneReservationPageChoice, alignment: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		Self::reserve_with_alignment(name, numa_socket_id, memory_zone_reservation_page_choice, Self::SpecialLengthSignifyingLongestPossibleReservation, alignment)
	}
	
	/// Reserve a memory zone with `alignment` specified.
	///
	/// Note: 0 for `length` is special; it allocates the longest possible item of memory: see `Self::SpecialLengthSignifyingLongestPossibleReservation`. Rather than specifying length as `0`, use `Self::reserve_with_alignment_longest_possible` instead as it documents intention better.
	#[inline(always)]
	pub fn reserve_with_alignment(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memory_zone_reservation_page_choice: MemoryZoneReservationPageChoice, length: usize, alignment: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		Self::assert_memory_zone_name_size(name);

		let result = unsafe { rte_memzone_reserve_aligned(name.as_ptr(), length, numa_socket_id.as_c_int(), memory_zone_reservation_page_choice.bits(), alignment as u32) };
		Self::process_reservation_result(name, result, "rte_memzone_reserve_aligned")
	}

	/// Reserve the longest possible memory zone with `alignment` and `boundary` specified.
	#[inline(always)]
	pub fn reserve_with_alignment_and_boundary_longest_possible(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memory_zone_reservation_page_choice: MemoryZoneReservationPageChoice, alignment: PowerOfTwoThirtyTwoBit, boundary: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		Self::reserve_with_alignment_and_boundary(name, numa_socket_id, memory_zone_reservation_page_choice, Self::SpecialLengthSignifyingLongestPossibleReservation as u32, alignment, boundary)
	}
	
	/// Reserve a memory zone with `alignment` and `boundary` specified.
	///
	/// Note: 0 for `length` is special; it allocates the longest possible item of memory: see `Self::SpecialLengthSignifyingLongestPossibleReservation`. Rather than specifying length as `0`, use `Self::reserve_with_alignment_and_boundary_longest_possible` instead as it documents intention better.
	#[inline(always)]
	pub fn reserve_with_alignment_and_boundary(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memory_zone_reservation_page_choice: MemoryZoneReservationPageChoice, length: u32, alignment: PowerOfTwoThirtyTwoBit, boundary: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		Self::assert_memory_zone_name_size(name);

		debug_assert!(length <= boundary as u32, "length '{}' is greater than the boundary '{:?}'", length, boundary);

		let result = unsafe { rte_memzone_reserve_bounded(name.as_ptr(), length as usize, numa_socket_id.as_c_int(), memory_zone_reservation_page_choice.bits(), alignment as u32, boundary as u32) };
		Self::process_reservation_result(name, result, "rte_memzone_reserve_bounded")
	}
	
	#[inline(always)]
	fn assert_memory_zone_name_size(name: ConstCStr)
	{
		debug_assert!(name.to_bytes().len() < RTE_MEMZONE_NAMESIZE, "name '{}' is equal to or greater than RTE_MEMZONE_NAMESIZE, '{}'", name.rustValue, RTE_MEMZONE_NAMESIZE);
	}
	
	#[inline(always)]
	fn process_reservation_result(name: ConstCStr, result: *const rte_memzone, function_name: &str) -> Option<MemoryZone>
	{
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOSPC => None,
				E::ENOMEM => None,
				
				E::EINVAL => panic!("Bad parameters passed to {}()", function_name),
				E::EEXIST => panic!("Memory zone named '{}' already exists", name.to_str()),
				E_RTE::NO_CONFIG => panic!("Could not get a pointer to rte_config in function {}()", function_name),
				E_RTE::SECONDARY => panic!("Function {}() was called from a secondary process instance", function_name),
				
				illegal @ _ => panic!("Unexpected error '{}' from {}()", illegal, function_name),
			}
		}
		else
		{
			Some
			(
				Self
				{
					name,
					handle: unsafe { NonNull::new_unchecked(result) },
					free_on_drop: true,
				}
			)
		}
	}
}
