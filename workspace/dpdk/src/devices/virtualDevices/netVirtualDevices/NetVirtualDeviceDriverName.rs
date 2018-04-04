// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum NetVirtualDeviceDriverName
{
	AfPacket,
	Bonding,
	Null,
	PacketCapture,
	Ring,
	VirtIoUser,
	VirtualHost,
	Xen,
}

impl DeviceDriverName for NetVirtualDeviceDriverName
{
	#[inline(always)]
	fn value(&self) -> &'static str
	{
		match *self
		{
			NetVirtualDeviceDriverName::AfPacket => "net_af_packet",
			NetVirtualDeviceDriverName::Bonding => "net_bonding",
			NetVirtualDeviceDriverName::Null => "net_null",
			NetVirtualDeviceDriverName::PacketCapture => "net_pcap",
			NetVirtualDeviceDriverName::Ring => "net_ring",
			NetVirtualDeviceDriverName::VirtIoUser => "net_virtio_user",
			NetVirtualDeviceDriverName::VirtualHost => "net_vhost",
			NetVirtualDeviceDriverName::Xen => "net_xen",
		}
	}
}
