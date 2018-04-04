// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn updateRetaIndirectionTable(&self, receiveSideScalingRetaIndirectionTable: &mut ReceiveSideScalingRetaIndirectionTable) -> Result<(), UnsupportedByHardwareError>
	{
		match unsafe { ::dpdk_sys::rte_eth_dev_rss_reta_update(self.portIdentifier(), receiveSideScalingRetaIndirectionTable.as_rte_eth_rss_reta_entry64(), receiveSideScalingRetaIndirectionTable.retaSize()) }
		{
			0 => Ok(()),
			NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),
			
			NegativeE::EINVAL => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
			result @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_rss_reta_update()", result),
		}
	}
	
	#[inline(always)]
	pub fn queryRetaIndirectionTable(&self, size: PowerOfTwoSixteenBit) -> Result<ReceiveSideScalingRetaIndirectionTable, UnsupportedByHardwareError>
	{
		let mut receiveSideScalingRetaIndirectionTable = ReceiveSideScalingRetaIndirectionTable::empty(size);
		
		match unsafe { ::dpdk_sys::rte_eth_dev_rss_reta_query(self.portIdentifier(), receiveSideScalingRetaIndirectionTable.as_rte_eth_rss_reta_entry64(), receiveSideScalingRetaIndirectionTable.retaSize()) }
		{
			0 => Ok(receiveSideScalingRetaIndirectionTable),
			NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),
			
			NegativeE::EINVAL => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
			result @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_rss_reta_query()", result),
		}
	}
}
