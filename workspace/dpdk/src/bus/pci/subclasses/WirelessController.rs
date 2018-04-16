// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum WirelessController
{
	/// IRDA.
	IrdaController = 0x00,
	ConsumerIrController = 0x01,
	RfController = 0x10,
	Bluetooth = 0x11,
	Broadband = 0x12,
	
	/// 802.1a.
	_802_1aController = 0x20,
	
	/// 802.1b.
	_802_1bController = 0x21,
	
	/// No effective sub class.
	WirelessController = 0x80,
}
