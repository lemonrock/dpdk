// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier reference abstracts the way DPDK refers to physical PCI ethernet ports and virtual ethernet ports ('vdev'), such as Linux's AF Packet interface, TUN/TAP, bonded ethernet ports, and the like, which are software abstractions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum EthernetPortIdentifierReference
{
	/// Physical PCI device.
	PhysicalPci(IndirectPciDeviceIdentifier),
	
	/// Virtual device.
	Virtual(NetVirtualDeviceName),
}

impl DeviceName for EthernetPortIdentifierReference
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		use self::EthernetPortIdentifierReference::*;
		
		match *self
		{
			PhysicalPci(ref indirect_pci_device_identifier) => indirect_pci_device_identifier.to_string(),
			
			Virtual(ref net_virtual_device_name) => net_virtual_device_name.to_string(),
		}
	}
}

impl EthernetPortIdentifierReference
{
	/// Ethernet port identifier.
	///
	/// Only works after configuration of the DPDK environment.
	#[inline(always)]
	pub(crate) fn ethernet_port_identifier(&self) -> EthernetPortIdentifier
	{
		let device_name = self.to_device_name();
		
		for potential_ethernet_port_identifier in 0u16 .. RTE_MAX_ETHPORTS as u16
		{
			if let Ok(ethernet_port_identifier) = EthernetPortIdentifier::try_from_u16_unchecked(potential_ethernet_port_identifier)
			{
				let ethernet_port_name = &ethernet_port_identifier.data().name[..];
				
				if (unsafe { strcmp(ethernet_port_name.as_ptr(), device_name.as_ptr()) }) == 0
				{
					return ethernet_port_identifier
				}
			}
		}
		
		panic!("No matching ethernet port identifier")
	}
}
