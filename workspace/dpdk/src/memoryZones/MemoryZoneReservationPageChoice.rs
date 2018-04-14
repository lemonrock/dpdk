// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub struct MemoryZoneReservationPageChoice: u32
	{
		const ReservedFrom256KbPages = RTE_MEMZONE_256KB;
		const ReservedFrom2MbPages = RTE_MEMZONE_2MB;
		const ReservedFrom16MbPages = RTE_MEMZONE_16MB;
		const ReservedFrom256MbPages = RTE_MEMZONE_256MB;
		const ReservedFrom512MbPages = RTE_MEMZONE_512MB;
		const ReservedFrom1GbPages = RTE_MEMZONE_1GB;
		const ReservedFrom4GbPages = RTE_MEMZONE_4GB;
		const ReservedFrom16GbPages = RTE_MEMZONE_16GB;
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
