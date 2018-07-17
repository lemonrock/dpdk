// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a PCI device type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PciDeviceType
{
	class: PciDeviceClass,
	vendor_and_device: PciVendorAndDevice,
	subsystem_vendor_and_device: Option<PciVendorAndDevice>,
}

impl PciDeviceType
{
	/// New instance.
	#[inline(always)]
	pub fn new(class: PciDeviceClass, vendor_and_device: PciVendorAndDevice, subsystem_vendor_and_device: Option<PciVendorAndDevice>) -> Self
	{
		Self
		{
			class,
			vendor_and_device,
			subsystem_vendor_and_device,
		}
	}
	
	/// To DPDK type.
	#[inline(always)]
	pub fn to_rte_pci_id(&self) -> rte_pci_id
	{
		let (subsystem_vendor_id, subsystem_device_id) = match self.subsystem_vendor_and_device
		{
			None => (0xFFFF, 0xFFFF),
			Some(subsystem_vendor_and_device) => (subsystem_vendor_and_device.vendor, subsystem_vendor_and_device.device)
		};
		
		rte_pci_id
		{
			class_id: self.class.to_u32(),
			vendor_id: self.vendor_and_device.vendor,
			device_id: self.vendor_and_device.device,
			subsystem_vendor_id,
			subsystem_device_id,
		}
	}
}
