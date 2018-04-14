// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransmitAdvancedConfiguration
{
	None,
	DataCentreBridging(rte_eth_dcb_tx_conf),
	VmdQ(rte_eth_vmdq_tx_conf),
	DataCentreBridgingAndVmdQ(rte_eth_vmdq_dcb_tx_conf),
}

impl TransmitAdvancedConfiguration
{
	#[allow(trivial_casts)]
	pub fn as_eth_conf__bindgen_ty_2(&self) -> rte_eth_conf__bindgen_ty_2
	{
		use self::TransmitAdvancedConfiguration::*;
		
		match *self
		{
			None => unsafe { uninitialized() },
			DataCentreBridging(ref data) => unsafe { transmute_copy(data) },
			VmdQ(ref data) => unsafe { transmute_copy(data) },
			DataCentreBridgingAndVmdQ(ref data) => unsafe { transmute_copy(data) },
		}
	}
}

impl Default for TransmitAdvancedConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		TransmitAdvancedConfiguration::None
	}
}
