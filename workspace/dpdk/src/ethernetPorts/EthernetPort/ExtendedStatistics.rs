// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn resetExtendedStatistics(&self)
	{
		unsafe { rte_eth_xstats_reset(self.portIdentifier()) }
	}

	#[inline(always)]
	pub fn retrieveExtendedStatisticsNames(&self) -> ExtendedStatisticNames
	{
		let size = match unsafe { rte_eth_xstats_get_names(self.portIdentifier(), null_mut(), 0) }
		{
			0 => return ExtendedStatisticNames(self.clone(), Vec::new()),
			size if size > 0 => size as usize,

			_ => panic!("rte_eth_xstats_get_names() failed - invalid portIdentifier?"),
		};

		let mut names = Vec::with_capacity(size);
		let result = unsafe { rte_eth_xstats_get_names(self.portIdentifier(), names.as_mut_ptr(), size as c_uint) };
		assert_eq!(result, size as c_int, "Second call to rte_eth_xstats_get_names() returned weird result '{}' instead of size '{}'", result, size);
		unsafe { names.set_len(size) };

		let mut namesConverted = Vec::with_capacity(size);
		for name in names
		{
			let asChar: &[c_char] = &name.name;
			let nameLengthExcludingTrailingNul = unsafe { strnlen(asChar.as_ptr(), RTE_ETH_XSTATS_NAME_SIZE) };

			let asBytes: &[u8] = unsafe { transmute(asChar) };
			let nameConverted = String::from_utf8_lossy(&asBytes[0..nameLengthExcludingTrailingNul]).into_owned();
			namesConverted.push(nameConverted);
		}
		ExtendedStatisticNames(self.clone(), namesConverted)
	}
}
