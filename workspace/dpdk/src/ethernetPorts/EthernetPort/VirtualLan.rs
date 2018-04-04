// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// TODO: Some functions are unimplemented, and there is not support for them in EthernetConfiguration

/*
	rte_eth_dev_vlan_filter(port_id: uint8_t, vlan_id: uint16_t, on: c_int) -> c_int;
	
		rte_vlan_type => Inner/Single or Outer
		tag_type => Tag Protocol Id => ?
	rte_eth_dev_set_vlan_ether_type(port_id: uint8_t, vlan_type: rte_vlan_type, tag_type: uint16_t) -> c_int;

	rte_eth_dev_set_vlan_pvid(port_id: uint8_t, pvid: uint16_t, on: c_int) -> c_int;
*/
impl EthernetPort
{
	#[inline(always)]
	pub fn setVirtualLanOffloading(&self, virtualLanOffloadFeatures: VirtualLanOffloadFeatures) -> Result<(), UnsupportedByHardwareError>
	{
		match unsafe { rte_eth_dev_set_vlan_offload(self.portIdentifier(), virtualLanOffloadFeatures.bits()) }
		{
			result if result >= 0 => Ok(()),
			NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),
			
			NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
		
			unexpected @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_set_vlan_offload()", unexpected),
		}
	}
	
	#[inline(always)]
	pub fn getVirtualLanOffloading(&self) -> VirtualLanOffloadFeatures
	{
		match unsafe { rte_eth_dev_get_vlan_offload(self.portIdentifier()) }
		{
			result if result >= 0 => VirtualLanOffloadFeatures::from_bits(result).expect("Unknown bits from rte_eth_dev_get_vlan_offload()"),
			
			NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
		
			unexpected @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_get_vlan_offload()", unexpected),
		}
	}
}
