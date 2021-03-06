// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum GenericSystemPeripheral
{
	/// PIC.
	Pic = 0x00,
	
	/// DMAController.
	DmaController = 0x01,
	Timer = 0x02,
	
	/// RTC (real time clock).
	Rtc = 0x03,
	PciHotPlugController = 0x04,
	
	/// SD HostController.
	SdHostController = 0x05,
	
	/// IOMMU.
	Iommu = 0x06,
	
	/// No effective sub class.
	SystemPeripheral = 0x80,
}
