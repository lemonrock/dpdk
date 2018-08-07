// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum MassStorageController
{
	/// SCSI.
	ScsiStorageController = 0x00,
	
	/// IDE
	IdeInterface = 0x01,
	
	FloppyDiskController = 0x02,
	
	/// IPI bus.
	IpiBusController = 0x03,
	
	/// RAID bus.
	RaidBusController = 0x04,
	
	/// ATA (PATA).
	AtaController = 0x05,
	
	/// SATA
	SataController = 0x06,
	
	/// SAS, Serial Attached SCSI.
	SerialAttachedScsiController = 0x07,
	
	/// NVMe.
	NonVolatileMemoryController = 0x08,
	
	/// No effective sub class.
	MassStorageController = 0x80,
}
