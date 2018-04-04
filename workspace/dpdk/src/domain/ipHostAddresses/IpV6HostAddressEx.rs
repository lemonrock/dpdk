// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait IpV6HostAddressEx
{
	#[inline(always)]
	fn from_Ipv6Addr_to_in6_addr(ipv6Addr: &Ipv6Addr) -> in6_addr;
	
	#[inline(always)]
	fn fromIpv6Addr(ipv6Addr: &Ipv6Addr) -> Self;
	
	#[inline(always)]
	fn to_in6_addr(self) -> in6_addr;
	
	#[inline(always)]
	fn as_u128(&self) -> u128;
	
	#[inline(always)]
	fn isNotValidUnicast(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isUnspecified(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isLoopback(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isDeprecatedIpV4Compatible(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isIpV4Mapped(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isLinkLocalUnicast(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isDeprecatedSiteLocalUnicast(&self) -> bool;
	
	// RFC 4291
	// NOTE: Does not validate that flags and scope are valid
	#[inline(always)]
	fn isMulticast(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isMulticastAllNodesInterfaceLocal(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isMulticastAllNodesLinkLocal(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isMulticastAllRoutersInterfaceLocal(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isMulticastAllRoutersLinkLocal(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isMulticastAllRoutersSiteLocal(&self) -> bool;
	
	// RFC 4291
	#[inline(always)]
	fn isMulticastSolicitedNode(&self) -> bool;
	
	// RFC 3849
	#[inline(always)]
	fn isDocumentation(&self) -> bool;
}

impl IpV6HostAddressEx for IpV6HostAddress
{
	#[inline(always)]
	fn from_Ipv6Addr_to_in6_addr(ipv6Addr: &Ipv6Addr) -> in6_addr
	{
		Self::fromIpv6Addr(ipv6Addr).to_in6_addr()
	}
	
	#[inline(always)]
	fn fromIpv6Addr(ipv6Addr: &Ipv6Addr) -> Self
	{
		ipv6Addr.octets()
	}
	
	#[inline(always)]
	fn to_in6_addr(self) -> in6_addr
	{
		let mut value: in6_addr = unsafe { uninitialized() };
		value.s6_addr = self;
		value
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn as_u128(&self) -> u128
	{
		u128::from_be(unsafe { *(self as *const _ as *const u128) })
	}
	
	#[inline(always)]
	fn isNotValidUnicast(&self) -> bool
	{
		self.isUnspecified() || self.isLoopback() || self.isMulticast() || self.isDocumentation()
	}
	
	#[inline(always)]
	fn isUnspecified(&self) -> bool
	{
		static Unspecified: IpV6HostAddress = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
		
		self == &Unspecified
	}
	
	#[inline(always)]
	fn isLoopback(&self) -> bool
	{
		static Loopback: IpV6HostAddress = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
		
		self == &Loopback
	}
	
	#[inline(always)]
	fn isDeprecatedIpV4Compatible(&self) -> bool
	{
		let masked = self.as_u128() | 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_00000000;
		masked <= (u32::max_value() as u128) && !(masked as u32).isNotGloballyUniqueUnicast()
	}
	
	#[inline(always)]
	fn isIpV4Mapped(&self) -> bool
	{
		unsafe
		{
			getFirst64Bits(self) == 0x0000_0000_0000_0000 && getThird32Bits(self) == 0x0000_FFFF
		}
	}
	
	#[inline(always)]
	fn isLinkLocalUnicast(&self) -> bool
	{
		unsafe
		{
			getFirst64Bits(self) == 0xFE80000000000000
		}
	}
	
	#[inline(always)]
	fn isDeprecatedSiteLocalUnicast(&self) -> bool
	{
		unsafe
		{
			getFirst64Bits(self) == 0xFEC0000000000000
		}
	}
	
	#[inline(always)]
	fn isMulticast(&self) -> bool
	{
		unsafe
		{
			getFirst8Bits(self) == 0xFF
		}
	}
	
	#[inline(always)]
	fn isMulticastAllNodesInterfaceLocal(&self) -> bool
	{
		self.as_u128() == 0xFF01_0000_0000_0000_0000_0000_0000_0001
	}
	
	#[inline(always)]
	fn isMulticastAllNodesLinkLocal(&self) -> bool
	{
		self.as_u128() == 0xFF02_0000_0000_0000_0000_0000_0000_0001
	}
	
	#[inline(always)]
	fn isMulticastAllRoutersInterfaceLocal(&self) -> bool
	{
		self.as_u128() == 0xFF01_0000_0000_0000_0000_0000_0000_0002
	}
	
	#[inline(always)]
	fn isMulticastAllRoutersLinkLocal(&self) -> bool
	{
		self.as_u128() == 0xFF02_0000_0000_0000_0000_0000_0000_0002
	}
	
	#[inline(always)]
	fn isMulticastAllRoutersSiteLocal(&self) -> bool
	{
		self.as_u128() == 0xFF05_0000_0000_0000_0000_0000_0000_0002
	}
	
	#[inline(always)]
	fn isMulticastSolicitedNode(&self) -> bool
	{
		let value = self.as_u128();
		value >= 0xFF05_0000_0000_0000_0000_0000_FF00_0000 && value <= 0xFF05_0000_0000_0000_0000_0000_FFFF_FFFF
	}
	
	#[inline(always)]
	fn isDocumentation(&self) -> bool
	{
		unsafe
		{
			getFirst32Bits(self) == 0x20010DB8
		}
	}
}

#[allow(trivial_casts)]
#[inline(always)]
unsafe fn getFirst8Bits(value: &IpV6HostAddress) -> u8
{
	*value.get_unchecked(0)
}

#[allow(trivial_casts)]
#[inline(always)]
unsafe fn getFirst64Bits(value: &IpV6HostAddress) -> u64
{
	u64::from_be(*(value.get_unchecked(0) as *const _ as *const u64))
}

#[allow(trivial_casts)]
#[inline(always)]
unsafe fn getFirst32Bits(value: &IpV6HostAddress) -> u32
{
	u32::from_be(*(value.get_unchecked(0) as *const _ as *const u32))
}

#[allow(trivial_casts)]
#[inline(always)]
unsafe fn getThird32Bits(value: &IpV6HostAddress) -> u32
{
	u32::from_be(*(value.get_unchecked(8) as *const _ as *const u32))
}
