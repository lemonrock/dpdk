// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier reference abstracts the way DPDK refers to physical PCI ethernet ports and virtual ethernet ports ('vdev'), such as Linux's AF Packet interface, TUN/TAP, bonded ethernet ports, and the like, which are software abstractions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum EthernetPortIdentifierReference
{
	/// PCI device.
	Pci(IndirectPciDeviceIdentifier),
	
	/// Virtual device.
	Virtual(NetVirtualDeviceName),
}

impl EthernetPortIdentifierReference
{
	/// Ethernet port identifier.
	///
	/// Only works after configuration of the DPDK environment.
	#[inline(always)]
	pub(crate) fn ethernet_port_identifier(&self) -> EthernetPortIdentifier
	{
		panic!();
	}
}
