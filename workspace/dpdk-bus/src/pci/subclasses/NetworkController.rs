// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum NetworkController
{
	EthernetController = 0x00,
	TokenRingNetworkController = 0x01,
	
	/// FDDI ('fibre').
	FDDINetworkController = 0x02,
	
	/// ATM.
	AtmNetworkController = 0x03,
	
	/// ISDN.
	IsdnController = 0x04,
	
	WorldFipController = 0x05,
	
	/// PICMG.
	PicmgController = 0x06,
	InfinibandController = 0x07,
	FabricController = 0x08,
	
	/// No effective sub class.
	NetworkController = 0x80,
}
