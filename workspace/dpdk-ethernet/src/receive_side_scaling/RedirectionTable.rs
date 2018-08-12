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
