// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum BondingSlave
{
	// Enum constants are listed in the order preferred by the Ethernet Bonding parse code parse_port_id in rte_eth_bond_args.c
	
	ByPciDeviceAddress(DeviceAddress),
	
	ByVirtualDeviceName(NetVirtualDeviceName),
	
	ByEthernetPortIdentifier(EthernetPortIdentifier),
}

impl BondingSlave
{
	pub fn byVirtualDeviceName(netVirtualDeviceName: NetVirtualDeviceName) -> BondingSlave
	{
		assert!(netVirtualDeviceName.isNotBackedByDriverName(NetVirtualDeviceDriverName::Bonding), "A bonding slave can not itself be a bonding device");
		
		BondingSlave::ByVirtualDeviceName(netVirtualDeviceName)
	}
	
	#[inline(always)]
	pub fn asDpdkString(&self) -> String
	{
		match *self
		{
			BondingSlave::ByPciDeviceAddress(ref deviceAddress) => deviceAddress.to_string(),
			
			BondingSlave::ByVirtualDeviceName(ref virtualDeviceName) => virtualDeviceName.to_string(),

			BondingSlave::ByEthernetPortIdentifier(ref ethernetPortIdentifier) => format!("{}", ethernetPortIdentifier),
		}
	}
}
