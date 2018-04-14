// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryZone(ConstCStr, *const rte_memzone, bool);

impl Drop for MemoryZone
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.2
		{
			match unsafe { rte_memzone_free(self.1) }
			{
				0 => (),

				NegativeE::EINVAL =>
				{
					// Can not drop an IVSHMEM memory zone
					()
				},

				illegal @ _ => panic!("Unexpected result '{}' from rte_memzone_free()", illegal),
			}
		}
	}
}

impl MemoryZone
{
	pub const SpecialLengthSignifyingLongestPossibleReservation: usize = 0;

	#[inline(always)]
	pub fn lookUp(name: ConstCStr) -> Option<MemoryZone>
	{
		debug_assert!(name.to_bytes().len() < RTE_MEMZONE_NAMESIZE, "name '{}' is equal to or greater than RTE_MEMZONE_NAMESIZE, '{}'", name.rustValue, RTE_MEMZONE_NAMESIZE);

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

	#[inline(always)]
	pub fn dumpAllMemoryZonesToStandardError()
	{
		unsafe { rte_memzone_dump(stderr as *mut FILE) }
	}

	/// Defaults alignment to RTE_CACHE_LINE_SIZE
	#[inline(always)]
	pub fn reserveAlignedToCacheLineSizeLongestPossible(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memoryZoneReservationPageChoice: MemoryZoneReservationPageChoice) -> Option<MemoryZone>
	{
		Self::reserveAlignedToCacheLineSize(name, numa_socket_id, memoryZoneReservationPageChoice, Self::SpecialLengthSignifyingLongestPossibleReservation)
	}

	/// Defaults alignment to RTE_CACHE_LINE_SIZE
	/// Note: 0 is special; it allocates the longest possible item of memory.
	/// ie Self::SpecialLengthSignifyingLongestPossibleReservation
	#[inline(always)]
	pub fn reserveAlignedToCacheLineSize(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memoryZoneReservationPageChoice: MemoryZoneReservationPageChoice, length: usize) -> Option<MemoryZone>
	{
		debug_assert!(name.to_bytes().len() < RTE_MEMZONE_NAMESIZE, "name '{}' is equal to or greater than RTE_MEMZONE_NAMESIZE, '{}'", name.rustValue, RTE_MEMZONE_NAMESIZE);

		let result = unsafe { rte_memzone_reserve(name.as_ptr(), length, numa_socket_id.as_c_int(), memoryZoneReservationPageChoice.bits()) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOSPC => None,
				E::ENOMEM => None,

				E::EINVAL => panic!("Bad parameters passed to rte_memzone_reserve()"),
				E::EEXIST => panic!("Memory zone named '{}' already exists", name.to_str()),
				E_RTE::NO_CONFIG => panic!("Could not get a pointer to rte_config in function rte_memzone_reserve()"),
				E_RTE::SECONDARY => panic!("Function rte_memzone_reserve() was called from a secondary process instance"),

				illegal @ _ => panic!("Unexpected error '{}' from rte_memzone_reserve()", illegal),
			}
		}
		else
		{
			Some(MemoryZone(name, result, true))
		}
	}

	#[inline(always)]
	pub fn reserveWithAlignmentLongestPossible(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memoryZoneReservationPageChoice: MemoryZoneReservationPageChoice, alignment: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		Self::reserveWithAlignment(name, numa_socket_id, memoryZoneReservationPageChoice, Self::SpecialLengthSignifyingLongestPossibleReservation, alignment)
	}

	/// Note: 0 is special; it allocates the longest possible item of memory.
	/// ie Self::SpecialLengthSignifyingLongestPossibleReservation
	#[inline(always)]
	pub fn reserveWithAlignment(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memoryZoneReservationPageChoice: MemoryZoneReservationPageChoice, length: usize, alignment: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		debug_assert!(name.to_bytes().len() < RTE_MEMZONE_NAMESIZE, "name '{}' is equal to or greater than RTE_MEMZONE_NAMESIZE, '{}'", name.rustValue, RTE_MEMZONE_NAMESIZE);

		let result = unsafe { rte_memzone_reserve_aligned(name.as_ptr(), length, numa_socket_id.as_c_int(), memoryZoneReservationPageChoice.bits(), alignment as u32) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOSPC => None,
				E::ENOMEM => None,

				E::EINVAL => panic!("Bad parameters passed to rte_memzone_reserve()"),
				E::EEXIST => panic!("Memory zone named '{}' already exists", name.to_str()),
				E_RTE::NO_CONFIG => panic!("Could not get a pointer to rte_config in function rte_memzone_reserve()"),
				E_RTE::SECONDARY => panic!("Function rte_memzone_reserve() was called from a secondary process instance"),

				illegal @ _ => panic!("Unexpected error '{}' from rte_memzone_reserve()", illegal),
			}
		}
		else
		{
			Some(MemoryZone(name, result, true))
		}
	}

	#[inline(always)]
	pub fn reserveWithAlignmentAndBoundaryLongestPossible(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memoryZoneReservationPageChoice: MemoryZoneReservationPageChoice, alignment: PowerOfTwoThirtyTwoBit, boundary: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		Self::reserveWithAlignmentAndBoundary(name, numa_socket_id, memoryZoneReservationPageChoice, Self::SpecialLengthSignifyingLongestPossibleReservation as u32, alignment, boundary)
	}

	/// Note: 0 is special; it allocates the longest possible item of memory.
	/// ie Self::SpecialLengthSignifyingLongestPossibleReservation
	#[inline(always)]
	pub fn reserveWithAlignmentAndBoundary(name: ConstCStr, numa_socket_id: Option<NumaSocketId>, memoryZoneReservationPageChoice: MemoryZoneReservationPageChoice, length: u32, alignment: PowerOfTwoThirtyTwoBit, boundary: PowerOfTwoThirtyTwoBit) -> Option<MemoryZone>
	{
		debug_assert!(name.to_bytes().len() < RTE_MEMZONE_NAMESIZE, "name '{}' is equal to or greater than RTE_MEMZONE_NAMESIZE, '{}'", name.rustValue, RTE_MEMZONE_NAMESIZE);

		debug_assert!(length <= boundary as u32, "length '{}' is greater than the boundary '{:?}'", length, boundary);

		let result = unsafe { rte_memzone_reserve_bounded(name.as_ptr(), length as usize, numa_socket_id.as_c_int(), memoryZoneReservationPageChoice.bits(), alignment as u32, boundary as u32) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E::ENOSPC => None,
				E::ENOMEM => None,

				E::EINVAL => panic!("Bad parameters passed to rte_memzone_reserve()"),
				E::EEXIST => panic!("Memory zone named '{}' already exists", name.to_str()),
				E_RTE::NO_CONFIG => panic!("Could not get a pointer to rte_config in function rte_memzone_reserve()"),
				E_RTE::SECONDARY => panic!("Function rte_memzone_reserve() was called from a secondary process instance"),

				illegal @ _ => panic!("Unexpected error '{}' from rte_memzone_reserve()", illegal),
			}
		}
		else
		{
			Some(MemoryZone(name, result, true))
		}
	}
}
