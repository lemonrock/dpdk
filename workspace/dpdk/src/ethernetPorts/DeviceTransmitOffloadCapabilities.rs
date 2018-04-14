// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk; including this file; may be copied; modified; propagated; or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub struct DeviceTransmitOffloadCapabilities: u32
	{
		const TcpChecksum = DEV_TX_OFFLOAD_TCP_CKSUM;
		const UdpChecksum = DEV_TX_OFFLOAD_UDP_CKSUM;
		const SctpChecksum = DEV_TX_OFFLOAD_SCTP_CKSUM;
		const TcpSegmentationOffload = DEV_TX_OFFLOAD_TCP_TSO; // TSO
		const UdpSegmentationOffload = DEV_TX_OFFLOAD_UDP_TSO; // also known as 'UFO'
		const IpV4Checksum = DEV_TX_OFFLOAD_IPV4_CKSUM;
		const OuterIpV4Checksum = DEV_TX_OFFLOAD_OUTER_IPV4_CKSUM;
		const VlanInsert = DEV_TX_OFFLOAD_VLAN_INSERT;
		const QInQInsert = DEV_TX_OFFLOAD_QINQ_INSERT;
	}
}

impl Default for DeviceTransmitOffloadCapabilities
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl DeviceTransmitOffloadCapabilities
{
	#[inline(always)]
	fn doesNotSupport(&self, flag: DeviceTransmitOffloadCapabilities) -> bool
	{
		!self.contains(flag)
	}

	#[inline(always)]
	fn supports(&self, flag: DeviceTransmitOffloadCapabilities) -> bool
	{
		self.contains(flag)
	}

	#[inline(always)]
	pub fn doesNotSupportTcpChecksumOffloading(&self) -> bool
	{
		self.doesNotSupport(DeviceTransmitOffloadCapabilities::TcpChecksum)
	}

	#[inline(always)]
	pub fn doesNotSupportUdpChecksumOffloading(&self) -> bool
	{
		self.doesNotSupport(DeviceTransmitOffloadCapabilities::UdpChecksum)
	}

	#[inline(always)]
	pub fn doesNotSupportSctpChecksumOffloading(&self) -> bool
	{
		self.doesNotSupport(DeviceTransmitOffloadCapabilities::SctpChecksum)
	}

	#[inline(always)]
	pub fn doesNotSupportVlanInsertOrQInQInsertOffloading(&self) -> bool
	{
		self.doesNotSupport(DeviceTransmitOffloadCapabilities::VlanInsert) || self.doesNotSupport(DeviceTransmitOffloadCapabilities::QInQInsert)
	}

	#[inline(always)]
	pub fn doesNotSupportSegmentationOffloadingAtAll(&self) -> bool
	{
		self.doesNotSupport(DeviceTransmitOffloadCapabilities::TcpSegmentationOffload) && self.doesNotSupport(DeviceTransmitOffloadCapabilities::UdpSegmentationOffload)
	}

	#[inline]
	pub fn supportsIpV4ChecksumOffload(&self) -> bool
	{
		self.supports(DeviceTransmitOffloadCapabilities::IpV4Checksum)
	}

	#[inline(always)]
	pub fn supportsTcpSegmentationOffload(&self) -> bool
	{
		self.supports(DeviceTransmitOffloadCapabilities::TcpSegmentationOffload)
	}

	#[inline(always)]
	pub fn supportsUdpSegmentationOffload(&self) -> bool
	{
		self.supports(DeviceTransmitOffloadCapabilities::UdpSegmentationOffload)
	}

	#[inline(always)]
	pub fn supportsTcpAndUdpChecksumOffload(&self) -> bool
	{
		self.supports(DeviceTransmitOffloadCapabilities::TcpChecksum | DeviceTransmitOffloadCapabilities::UdpChecksum)
	}

	#[inline(always)]
	pub fn supportsNoneOfTcpAndUdpChecksumOffload(&self) -> bool
	{
		self.doesNotSupport(DeviceTransmitOffloadCapabilities::TcpChecksum) && self.doesNotSupport(DeviceTransmitOffloadCapabilities::UdpChecksum)
	}
}
