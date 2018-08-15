// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Net(work) virtual device driver name.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum NetVirtualDeviceDriverName
{
	/// Linux AF_PACKET.
	AfPacket,
	
	/// Bonded across slave network devices.
	Bonding,
	
	/// Linux KNI.
	KernelNativeInterface,
	
	/// Microsoft Hyper-V NetVSC devices.
	NetVSC,
	
	/// Packet capture (pcap).
	PacketCapture,
	
	/// Linux TAP.
	Tap,
	
	/// Linux TUN.
	Tun,
	
	/// virtio hypervisor network interface.
	VirtIoUser,
	
	/// vhost hypervisor network interface.
	VirtualHost,
}

impl DeviceDriverName for NetVirtualDeviceDriverName
{
	#[inline(always)]
	fn value(&self) -> &'static str
	{
		use self::NetVirtualDeviceDriverName::*;
		
		match *self
		{
			AfPacket => "net_af_packet",
			Bonding => "net_bonding",
			KernelNativeInterface => "net_kni",
			NetVSC => "net_vdev_netvsc",
			PacketCapture => "net_pcap",
			Tap => "net_tap",
			Tun => "net_tun",
			VirtIoUser => "net_virtio_user",
			VirtualHost => "net_vhost",
		}
	}
}
