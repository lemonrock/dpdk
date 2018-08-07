// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Configuration for a longest prefix match table of denied (or allowed) host addresses.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct HostAddressesSetConfiguration<L: LongestPrefixMatchTable>
{
	name: String,
	number_of_table8s_to_allocate: u32,
	network_addresses: HashSet<L::NetworkAddress>,
	#[serde(default)] phantom_data: PhantomData<L>,
}

impl<L: LongestPrefixMatchTable> LongestPrefixMatchTableConfiguration<L>
{
	#[inline(always)]
	fn configure(mut self, numa_node_choice: NumaNodeChoice) -> L
	{
		assert!(self.network_addresses.len() <= (::std::u32::MAX as usize), "Tooy many rules");
		
		let table = LongestPrefixMatchTable::new(&self.name, self.network_addresses.len() as u32, self.number_of_table8s_to_allocate, numa_node_choice);
		
		for network_address in self.network_addresses.iter()
		{
			table.add_rule(network_address, 0).unwrap();
		}
		
		table
	}
}
