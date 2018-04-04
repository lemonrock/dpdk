// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtendedStatisticNames(pub EthernetPort, pub Vec<String>);

impl ExtendedStatisticNames
{
	#[inline(always)]
	pub fn retrieveExtendedStatistics<'a>(&'a self) -> HashMap<&'a str, u64>
	{
		let size = self.1.len();
		if size == 0
		{
			return HashMap::new();
		}
		let mut statistics = Vec::with_capacity(size);
		
		let result = unsafe { ::dpdk_sys::rte_eth_xstats_get(self.0.portIdentifier(), statistics.as_mut_ptr(), size as c_uint)};
		assert!(result == size as c_int, "Call to rte_eth_xstats_get() returned weird result '{}' instead of size '{}'", result, size);
		unsafe { statistics.set_len(size) };
		
		let mut result = HashMap::with_capacity(size);
		for index in 0..size
		{
			let name = (*self.1.get(index).unwrap()).as_ref();
			let value = statistics.get(index).unwrap().value;
			result.insert(name, value);
		}
		result
	}
}
