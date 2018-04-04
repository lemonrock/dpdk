// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DriverIdentifier
{
	AmazonEna,
	ChelsioT5,
	CiscoVic,
	BroadcomBnxt,
	BroadcomBnx2x,
	IntelE1000Em,
	IntelE1000Igb,
	IntelFM10K,
	Inteli40ePhysical,
	Inteli40eVirtual,
	IntelIxgbePhysical,
	IntelIxgbePhysicalSubDevice,
	IntelIxgbeVirtual,
	IntelQat,
	MellanoxMlx4,
	MellanoxMlx5,
	NetCopeSzedata2,
	NetronomeNfp6000,
	QLogicQedePhysical,
	QLogicQedeVirtual,
	VirtIoNetwork,
	VmWareVmxNet3,
}

impl DriverIdentifier
{
	pub fn isEthernet(&self) -> bool
	{
		match *self
		{
			DriverIdentifier::IntelQat => false,
			_ => true,
		}
	}
	
	pub fn isCrypto(&self) -> bool
	{
		match *self
		{
			DriverIdentifier::IntelQat => true,
			_ => false,
		}
	}
}
