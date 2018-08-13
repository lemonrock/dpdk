// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A populated redirection table.
#[derive(Debug)]
pub enum RedirectionTable
{
	/// Has 1 entry element.
	Entries64([rte_eth_rss_reta_entry64; (ETH_RSS_RETA_SIZE_64 as usize / RTE_RETA_GROUP_SIZE)]),
	
	/// Has 2 entry elements.
	Entries128([rte_eth_rss_reta_entry64; (ETH_RSS_RETA_SIZE_128 as usize / RTE_RETA_GROUP_SIZE)]),
	
	/// Has 4 entry elements.
	Entries256([rte_eth_rss_reta_entry64; (ETH_RSS_RETA_SIZE_256 as usize / RTE_RETA_GROUP_SIZE)]),
	
	/// Has 8 entry elements.
	Entries512([rte_eth_rss_reta_entry64; (ETH_RSS_RETA_SIZE_512 as usize / RTE_RETA_GROUP_SIZE)]),
}

impl RedirectionTable
{
	#[inline(always)]
	pub(crate) fn configure(&mut self, ethernet_port_identifier: EthernetPortIdentifier)
	{
		use self::RedirectionTable::*;
		
		let (redirection_table_pointer, redirection_table_size) = match *self
		{
			Entries64(ref mut entries) => (entries.as_mut_ptr(), 64),
			Entries128(ref mut entries) => (entries.as_mut_ptr(), 128),
			Entries256(ref mut entries) => (entries.as_mut_ptr(), 256),
			Entries512(ref mut entries) => (entries.as_mut_ptr(), 512),
		};
		
		let result = unsafe { rte_eth_dev_rss_reta_update(ethernet_port_identifier.into(), redirection_table_pointer, redirection_table_size) };
		if likely!(result == 0)
		{
			return
		}
		
		match result
		{
			NegativeE::ENOTSUP => panic!("Ethernet port identifier '{}' does not support setting the redirection table (RETA)", ethernet_port_identifier),
			NegativeE::EINVAL => panic!("Bad parameter for rte_eth_dev_rss_reta_update for ethernet port identifier '{}'", ethernet_port_identifier),
			NegativeE::EIO => panic!("Device removed for rte_eth_dev_rss_reta_update for ethernet port identifier '{}'", ethernet_port_identifier),
			_ => panic!("Unknown result '{}' for rte_eth_dev_rss_reta_update", ethernet_port_identifier),
		}
	}
}
