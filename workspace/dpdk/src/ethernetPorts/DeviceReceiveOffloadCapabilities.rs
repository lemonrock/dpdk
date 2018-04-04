// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub flags DeviceReceiveOffloadCapabilities: u32
	{
		const IpV4Checksum = DEV_RX_OFFLOAD_IPV4_CKSUM,
		const OuterIpV4Checksum = DEV_RX_OFFLOAD_OUTER_IPV4_CKSUM,
		const QInQStrip = DEV_RX_OFFLOAD_QINQ_STRIP,
		const TcpChecksum = DEV_RX_OFFLOAD_TCP_CKSUM,
		const TcpLargeReceiveOffload = DEV_RX_OFFLOAD_TCP_LRO,
		const UdpChecksum = DEV_RX_OFFLOAD_UDP_CKSUM,
		const VlanStrip = DEV_RX_OFFLOAD_VLAN_STRIP,
	}
}

impl Default for DeviceReceiveOffloadCapabilities
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl DeviceReceiveOffloadCapabilities
{
	#[inline(always)]
	fn supports(&self, flag: DeviceReceiveOffloadCapabilities) -> bool
	{
		self.contains(flag)
	}
	
	#[inline(always)]
	pub fn supportsTcpLargeReceiveOffload(&self) -> bool
	{
		self.supports(DeviceReceiveOffloadCapabilities::TcpLargeReceiveOffload)
	}
	
	#[inline(always)]
	pub fn supportsIpV4TcpAndUdpChecksumOffload(&self) -> bool
	{
		self.supports(DeviceReceiveOffloadCapabilities::IpV4Checksum | DeviceReceiveOffloadCapabilities::TcpChecksum | DeviceReceiveOffloadCapabilities::UdpChecksum)
	}
	
	#[inline(always)]
	pub fn supportsNoneOfIpV4TcpAndUdpChecksumOffload(&self) -> bool
	{
		!self.supports(DeviceReceiveOffloadCapabilities::IpV4Checksum) && !self.supports(DeviceReceiveOffloadCapabilities::TcpChecksum) && !self.supports(DeviceReceiveOffloadCapabilities::UdpChecksum)
	}
	
	// Not 100% sure about QinQ stripping support implying setting rxmode.hw_vlan_strip = 1
	#[inline(always)]
	pub fn supportsVlanOrQinQStripping(&self) -> bool
	{
		self.supports(DeviceReceiveOffloadCapabilities::VlanStrip) || self.supports(DeviceReceiveOffloadCapabilities::QInQStrip)
	}
}
