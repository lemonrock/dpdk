// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AesNiGcmCryptoVirtualDevice
{
	index: u5,
	maximNumberOfQueuePairs: u31,
	maximNumberOfSessions: u31,
	numaSocketId: NumaSocketId,
}

impl VirtualDevice for AesNiGcmCryptoVirtualDevice
{
	type V = CryptoVirtualDeviceDriverName;
	
	const DriverName: CryptoVirtualDeviceDriverName = CryptoVirtualDeviceDriverName::AesNiGcm;
	
	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String
	{
		self.formattedVirtualDeviceArgumentsWithLeadingCommaCommon()
	}
}

impl CryptoVirtualDevice for AesNiGcmCryptoVirtualDevice
{
	#[inline(always)]
	fn maximNumberOfQueuePairs(&self) -> u31
	{
		self.maximNumberOfQueuePairs
	}
	
	#[inline(always)]
	fn maximNumberOfSessions(&self) -> u31
	{
		self.maximNumberOfSessions
	}
	
	#[inline(always)]
	fn numaSocketId(&self) -> NumaSocketId
	{
		self.numaSocketId
	}
}

impl AesNiGcmCryptoVirtualDevice
{
	#[inline(always)]
	pub fn defaultish(index: u5, numaSocketId: NumaSocketId) -> Self
	{
		Self::new(index, numaSocketId, Self::DefaultMaximumNumberOfQueuePairs, Self::DefaultMaximumNumberOfSessions)
	}
	
	#[inline(always)]
	pub fn new(index: u5, numaSocketId: NumaSocketId, maximNumberOfQueuePairs: u31, maximNumberOfSessions: u31) -> Self
	{
		assert!(index < VirtualDeviceName::<CryptoVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<CryptoVirtualDeviceDriverName>::MaximumIndex);
		
		AesNiGcmCryptoVirtualDevice
		{
			index: index,
			maximNumberOfQueuePairs: maximNumberOfQueuePairs,
			maximNumberOfSessions: maximNumberOfSessions,
			numaSocketId: numaSocketId,
		}
	}
}
