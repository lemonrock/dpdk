// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn setFlowControl(&self, flowControl: FlowControl) -> Result<bool, UnsupportedByHardwareError>
	{
		let mut flowControl = flowControl.as_rte_eth_fc_conf();
		
		let result = unsafe { ::dpdk_sys::rte_eth_dev_flow_ctrl_set(self.portIdentifier, &mut flowControl) };
		if likely(result == 0)
		{
			Ok(true)
		}
		else
		{
			match result
			{
				NegativeE::EIO => Ok(false),
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),
			
				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid - how if it worked for the first call to rte_eth_dev_get_reg_info()?", self.portIdentifier()),
				NegativeE::EINVAL => panic!("Bad flowControl parameter '{:?}'", flowControl),
			
				_ => panic!("Unexpected error code '{}' from second call to rte_eth_dev_flow_ctrl_set()", result),
			}
		}
	}
	
	#[inline(always)]
	pub fn setDataCentreBridgingPriorityFlowControl(&self, dataCentreBridgingPriorityFlowControl: DataCentreBridgingPriorityFlowControl) -> Result<bool, UnsupportedByHardwareError>
	{
		let mut dataCentreBridgingPriorityFlowControl = dataCentreBridgingPriorityFlowControl.as_rte_eth_pfc_conf();
		
		let result = unsafe { ::dpdk_sys::rte_eth_dev_priority_flow_ctrl_set(self.portIdentifier, &mut dataCentreBridgingPriorityFlowControl) };
		if likely(result == 0)
		{
			Ok(true)
		}
		else
		{
			match result
			{
				NegativeE::EIO => Ok(false),
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),
			
				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid - how if it worked for the first call to rte_eth_dev_get_reg_info()?", self.portIdentifier()),
				NegativeE::EINVAL => panic!("Bad dataCentreBridgingPriorityFlowControl parameter '{:?}'", dataCentreBridgingPriorityFlowControl),
			
				_ => panic!("Unexpected error code '{}' from second call to rte_eth_dev_flow_ctrl_set()", result),
			}
		}
	}
}
