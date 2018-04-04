// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



/*
	Match an ethernet configuration
		- by SupportedDevice (eg BroadcomBnxt_ChipNumber_57301)
		- by PCI Device Address
		- by Driver (eg BroadcomBnxt)
		- by Vendor (eg Broadcom)

		- by NUMA socket
		- by number of ports on the card (everything bar Mellanox)

eth_dev_info returns a struct that include DriverName
this is something like "xen virtio PMD"
	=> we can match on this
*/

// We also need to match virtual devices, too - check rte_devargs

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeviceConfigurationMatch
{
	BySupportedDevices(Vec<SupportedDevice>),
	ByDeviceAddresses(Vec<DeviceAddress>),
	ByDriverIdentifier(DriverIdentifier), // Need to merge with Driver code
	ByVendor(VendorId),
}
