// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait IpV4HostAddressEx
{
	fn from_Ipv4Addr_to_in_addr(ipv4Addr: &Ipv4Addr) -> in_addr;
	
	#[inline(always)]
	fn fromIpv4Addr(ipv4Addr: &Ipv4Addr) -> Self;
	
	#[inline(always)]
	fn fromOctets(octets: IpV4HostAddressOctets) -> Self;
	
	#[inline(always)]
	fn to_in_addr(self) -> in_addr;
	
	#[inline(always)]
	fn toOctets(self) -> IpV4HostAddressOctets;
	
	#[inline(always)]
	fn isNotValidUnicast(self) -> bool;
	
	#[inline(always)]
	fn isNotGloballyUniqueUnicast(self) -> bool;
	
	#[inline(always)]
	fn isPrivate(self) -> bool;
	
	#[inline(always)]
	fn isLinkLocal(self) -> bool;
	
	#[inline(always)]
	fn isUnspecified(self) -> bool;
	
	#[inline(always)]
	fn isBroadcast(self) -> bool;
	
	#[inline(always)]
	fn isLoopback(self) -> bool;
	
	#[inline(always)]
	fn isMulticast(self) -> bool;
	
	#[inline(always)]
	fn isDocumentation(self) -> bool;
}

impl IpV4HostAddressEx for IpV4HostAddress
{
	#[inline(always)]
	fn from_Ipv4Addr_to_in_addr(ipv4Addr: &Ipv4Addr) -> in_addr
	{
		Self::fromIpv4Addr(ipv4Addr).to_in_addr()
	}
	
	#[inline(always)]
	fn fromIpv4Addr(ipv4Addr: &Ipv4Addr) -> Self
	{
		Self::fromOctets(ipv4Addr.octets())
	}
	
	#[cfg(target_endian = "little")]
	#[inline(always)]
	fn fromOctets(octets: IpV4HostAddressOctets) -> Self
	{
		unsafe
		{
			transmute::<IpV4HostAddressOctets, IpV4HostAddress>(octets).to_be()
		}
	}
	
	#[cfg(target_endian = "big")]
	#[inline(always)]
	fn fromOctets(octets: IpV4HostAddressOctets) -> Self
	{
		unsafe
		{
			transmute::<IpV4HostAddressOctets, IpV4HostAddress>(octets)
		}
	}
	
	#[inline(always)]
	fn to_in_addr(self) -> in_addr
	{
		in_addr
		{
			s_addr: self.to_be()
		}
	}
	
	#[cfg(target_endian = "little")]
	#[allow(trivial_casts)]
	#[inline(always)]
	fn toOctets(self) -> IpV4HostAddressOctets
	{
		let swapped = self.to_be();
		unsafe
		{
			let mut result: IpV4HostAddressOctets = uninitialized();
			copy_nonoverlapping(&swapped as *const _ as *const u8, result.get_unchecked_mut(0), SizeOfIpV4HostAddress);
			result
		}
	}
	
	#[cfg(target_endian = "big")]
	#[allow(trivial_casts)]
	#[inline(always)]
	fn toOctets(self) -> IpV4HostAddressOctets
	{
		unsafe
		{
			let mut result: IpV4HostAddressOctets = uninitialized();
			copy_nonoverlapping(&self as * const _ as *const u8, result.get_unchecked_mut(0), SizeOfIpV4HostAddress);
			result
		}
	}
	
	#[inline(always)]
	fn isNotValidUnicast(self) -> bool
	{
		self.isUnspecified() ||
		self.isLoopback() ||
		self.isMulticast() ||
		self.isDocumentation() ||
		self.isBroadcast()
	}
	
	#[inline(always)]
	fn isNotGloballyUniqueUnicast(self) -> bool
	{
		self.isNotValidUnicast() || self.isLinkLocal() || self.isPrivate()
	}
	
	#[inline(always)]
	fn isPrivate(self) -> bool
	{
		IpV4NetworkAddress::Private1.contains(self) || IpV4NetworkAddress::Private2.contains(self) || IpV4NetworkAddress::Private3.contains(self)
	}
	
	#[inline(always)]
	fn isLinkLocal(self) -> bool
	{
		IpV4NetworkAddress::LinkLocal.contains(self)
	}
	
	#[inline(always)]
	fn isUnspecified(self) -> bool
	{
		const IpV4AnyAddress: IpV4HostAddress = ipv4HostAddressFromNumbers(0, 0, 0, 0);
		
		self == IpV4AnyAddress
	}
	
	#[inline(always)]
	fn isBroadcast(self) -> bool
	{
		const IpV4BroadcastAddress: IpV4HostAddress = ipv4HostAddressFromNumbers(255, 255, 255, 255);
		
		self == IpV4BroadcastAddress
	}
	
	#[inline(always)]
	fn isLoopback(self) -> bool
	{
		IpV4NetworkAddress::Loopback.contains(self)
	}
	
	#[inline(always)]
	fn isMulticast(self) -> bool
	{
		IpV4NetworkAddress::Multicast.contains(self)
	}
	
	#[inline(always)]
	fn isDocumentation(self) -> bool
	{
		IpV4NetworkAddress::TestNet1.contains(self) || IpV4NetworkAddress::TestNet2.contains(self) || IpV4NetworkAddress::TestNet3.contains(self)
	}
}
