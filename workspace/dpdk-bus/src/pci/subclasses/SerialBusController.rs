// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum SerialBusController
{
	/// IEEE 1394.
	FireWire = 0x00,
	
	/// ACCESS Bus.
	AccessBus = 0x01,
	
	/// SSA.
	SSA = 0x02,
	
	/// USB.
	UsbController = 0x03,
	
	FibreChannel = 0x04,
	
	/// SMBus
	SmBus = 0x05,
	
	InfiniBand = 0x06,
	
	/// IPMI interface.
	IpmiInterface = 0x07,
	
	/// SERCOS interface.
	SercosInterface = 0x08,
	
	/// CANBUS.
	Canbus = 0x09,
}
