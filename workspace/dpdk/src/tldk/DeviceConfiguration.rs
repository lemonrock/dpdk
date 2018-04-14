// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct DeviceConfiguration
{
	pub deviceOffloadingIsActive: bool,
	pub localSocketAddresses: IpV4AndOrIpV6<AddressWithListOfOpenLocalLayer4Ports<Ipv4Addr>, AddressWithListOfOpenLocalLayer4Ports<Ipv6Addr>>
}

impl DeviceConfiguration
{
	#[inline(always)]
	pub fn offloading(&self, layer4Protocol: Layer4Protocol) -> (DeviceReceiveOffloadCapabilities, DeviceTransmitOffloadCapabilities)
	{
		if self.deviceOffloadingIsActive
		{
			let (receiveOffload, transmitOffload) = match layer4Protocol
			{
				Layer4Protocol::Udp => (DeviceReceiveOffloadCapabilities::UdpChecksum, DeviceTransmitOffloadCapabilities::UdpChecksum | DeviceTransmitOffloadCapabilities::UdpSegmentationOffload),
				Layer4Protocol::Tcp => (DeviceReceiveOffloadCapabilities::TcpChecksum | DeviceReceiveOffloadCapabilities::TcpLargeReceiveOffload, DeviceTransmitOffloadCapabilities::TcpChecksum | DeviceTransmitOffloadCapabilities::TcpSegmentationOffload),
			};
			
			(receiveOffload | DeviceReceiveOffloadCapabilities::IpV4Checksum, transmitOffload | DeviceTransmitOffloadCapabilities::IpV4Checksum)
		}
		else
		{
			(DeviceReceiveOffloadCapabilities::empty(), DeviceTransmitOffloadCapabilities::empty())
		}
	}
	
	#[inline(always)]
	pub fn supportsIpV4(&self) -> bool
	{
		self.localSocketAddresses.hasIpV4()
	}
	
	#[inline(always)]
	pub fn supportsIpV6(&self) -> bool
	{
		self.localSocketAddresses.hasIpV6()
	}
	
	#[inline(always)]
	pub fn ipV4BlockedPortsForTldk(&self) -> TldkBlockedPortsList
	{
		if let Some(ipV4) = self.localSocketAddresses.ipV4()
		{
			ipV4.blockedPortsForTldk()
		}
		else
		{
			Vec::new()
		}
	}
	
	#[inline(always)]
	pub fn ipV6BlockedPortsForTldk(&self) -> TldkBlockedPortsList
	{
		if let Some(ipV6) = self.localSocketAddresses.ipV6()
		{
			ipV6.blockedPortsForTldk()
		}
		else
		{
			Vec::new()
		}
	}
	
	#[inline(always)]
	pub fn in_addr(&self) -> in_addr
	{
		if let Some(ipV4) = self.localSocketAddresses.ipV4()
		{
			ipV4.in_addr()
		}
		else
		{
			// ie INADDR_ANY
			unsafe { zeroed() }
		}
	}
	
	#[inline(always)]
	pub fn in6_addr(&self) -> in6_addr
	{
		if let Some(ipV6) = self.localSocketAddresses.ipV6()
		{
			ipV6.in6_addr()
		}
		else
		{
			// ie IN6ADDR_ANY_INIT
			unsafe { zeroed() }
		}
	}
}
