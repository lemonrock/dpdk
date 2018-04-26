// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Represents a `MemoryZone` reservation page size choice.
	///
	/// Defaults to `ReservedFrom256KbPages`.
	pub struct MemoryZoneReservationPageChoice: u32
	{
		/// Reserved from 256Kb pages.
		const ReservedFrom256KbPages = RTE_MEMZONE_256KB;
		
		/// Reserved from 2Mb pages.
		const ReservedFrom2MbPages = RTE_MEMZONE_2MB;
		
		/// Reserved from 16Mb pages.
		const ReservedFrom16MbPages = RTE_MEMZONE_16MB;
		
		/// Reserved from 256Mb pages.
		const ReservedFrom256MbPages = RTE_MEMZONE_256MB;
		
		/// Reserved from 512Mb pages.
		const ReservedFrom512MbPages = RTE_MEMZONE_512MB;
		
		/// Reserved from 1Gb pages.
		const ReservedFrom1GbPages = RTE_MEMZONE_1GB;
		
		/// Reserved from 4Gb pages.
		const ReservedFrom4GbPages = RTE_MEMZONE_4GB;
		
		/// Reserved from 16Gb pages.
		const ReservedFrom16GbPages = RTE_MEMZONE_16GB;
		
		/// Add this flag using `or` to hint to the memory zone reservation that the page size choice is only a preference.
		const IsSizeHintOnly = RTE_MEMZONE_SIZE_HINT_ONLY;
	}
}

impl Default for MemoryZoneReservationPageChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::IsSizeHintOnly | Self::ReservedFrom256KbPages
	}
}
