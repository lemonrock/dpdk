// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Protocol (IP) version 6 host address.
///
/// Stored internally in network byte order.
///
/// Defaults to `Unspecified`.
///
/// For all unicast addresses, except those that start with the binary value 000, Interface IDs are required to be 64 bits long. If derived from an IEEE MAC-layer address, they must be constructed in Modified EUI-64 format (see Appendix A of RFC 4291 replaced by RFC 7136 section 5.
///
/// Currently globally routable address assignments are at <https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xhtml>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolVersion6HostAddress(pub [u8; Self::Size]);

impl Default
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6HostAddress::Unspecified
	}
}

impl InternetProtocolVersion6HostAddress
{
	/// Size of an Internet Protocol (IP) Version 6 host address.
	pub const Size: usize = 16;
	
	/// Unspecified address.
	pub const Unspecified: Self = InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
	
	/// Loopback address.
	pub const Loopback: Self = InternetProtocolVersion6HostAddress([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
	
	/// RFC 4291.
	pub const MulticastAllNodesInterfaceLocal: Self = InternetProtocolVersion6HostAddress([0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
	
	/// RFC 4291.
	pub const MulticastAllNodesLinkLocal: Self = InternetProtocolVersion6HostAddress([0xFF, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
	
	/// RFC 4291.
	pub const MulticastAllRoutersInterfaceLocal: Self = InternetProtocolVersion6HostAddress([0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
	
	/// RFC 4291.
	pub const MulticastAllRoutersLinkLocal: Self = InternetProtocolVersion6HostAddress([0xFF, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
	
	/// RFC 4291.
	pub const MulticastAllRoutersSiteLocal: Self = InternetProtocolVersion6HostAddress([0xFF, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
	
	/// RFC7723.
	pub const PortControlProtocolAnycast: Self = InternetProtocolVersion6HostAddress([0x20, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
	
	/// RFC8155.
	pub const TraversalUsingRelaysAroundNatAnycast: Self = InternetProtocolVersion6HostAddress([0x20, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
	
	#[inline(always)]
	pub fn from_ipv6_addr_to_in6_addr(ipv6_addr: &Ipv6Addr) -> in6_addr
	{
		unsafe { transmute_copy(ipv6_addr) }
	}
	
	#[inline(always)]
	pub fn from_ipv6_addr(ipv6_addr: &Ipv6Addr) -> Self
	{
		unsafe { transmute_copy(ipv6_addr) }
	}
	
	#[inline(always)]
	pub fn to_in6_addr(self) -> in6_addr
	{
		in6_addr
		{
			s6_addr: self.0,
		}
	}
	
	#[inline(always)]
	pub fn as_native_endian_u128(&self) -> u128
	{
		u128::from_be(self.as_network_endian_u128())
	}
	
	#[inline(always)]
	pub fn as_network_endian_u128(&self) -> u128
	{
		unsafe { transmute(self.0) }
	}
	
	#[inline(always)]
	pub fn is_not_valid_unicast(&self) -> bool
	{
		self.is_unspecified() || self.is_loopback() || self.is_multicast() || self.is_documentation()
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_unspecified(&self) -> bool
	{
		self == Self::Unspecified
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_loopback(&self) -> bool
	{
		self == Self::Loopback
	}
	
	/// Globally routable unicast addresses.
	///
	/// Assumes prefix `2000::/3`.
	///
	/// Excludes addresses that are `is_documentation()` or `is_link_local_unicast()`.
	#[inline(always)]
	pub fn is_globally_routable_unicast(&self)
	{
		USE THE IPV6NetworkAddress here.
		
		
		let value = unsafe { *self.0.get_unchecked(0) };
		if value & 0b11100000 == 0x20
		{
			if self.is_documentation()
			{
				return false
			}
			
			if self.is_link_local_unicast()
			{
				return false
			}
			
			true
		}
		else
		{
			false
		}
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_internet_protocol_version_4_mapped(&self) -> bool
	{
		&self.0[ .. 12] == [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF]
	}
	
	/// RFC 8215: Globally routable IPv4 addresses.
	///
	/// Uses 'well-known prefix' of `64:ff9b:1::/96`.
	///
	/// Returns a valid address if this is an embedded address.
	#[inline(always)]
	pub fn get_internet_protocol_version_4_embedded_rfc8215(&self) -> Option<InternetProtocolVersion4HostAddress>
	{
		if self.is_internet_protocol_version_4_embedded_rfc8215()
		{
			Some(self.internet_protocol_version_4_host_address())
		}
		else
		{
			None
		}
	}
	
	/// RFC 8215: Globally routable IPv4 addresses.
	///
	/// Uses 'well-known prefix' of `64:ff9b:1::/96`.
	#[inline(always)]
	pub fn is_internet_protocol_version_4_embedded_rfc8215(&self) -> bool
	{
		&self.0[ .. 12] == [0x00, 0x64, 0xFF, 0x9B, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
	}
	
	/// RFC 6052: Globally routable IPv4 addresses.
	///
	/// Uses 'well-known prefix' of `64:ff9b::/96`.
	///
	/// Returns a valid address if this is an embedded address.
	#[inline(always)]
	pub fn get_internet_protocol_version_4_embedded_rfc6052(&self) -> Option<InternetProtocolVersion4HostAddress>
	{
		if self.is_internet_protocol_version_4_embedded_rfc6052()
		{
			Some(self.internet_protocol_version_4_host_address())
		}
		else
		{
			None
		}
	}
	
	/// RFC 6052: Globally routable IPv4 addresses.
	///
	/// Uses 'well-known prefix' of `64:ff9b::/96`.
	#[inline(always)]
	pub fn is_internet_protocol_version_4_embedded_rfc6052(&self) -> bool
	{
		&self.0[ .. 12] == [0x00, 0x64, 0xFF, 0x9B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
	}
	
	/// RFC 4291.
	///
	/// Returns a valid address if this is a mapped address.
	#[inline(always)]
	pub fn get_internet_protocol_version_4_mapped(&self) -> Option<InternetProtocolVersion4HostAddress>
	{
		if self.is_internet_protocol_version_4_mapped()
		{
			Some(self.internet_protocol_version_4_host_address())
		}
		else
		{
			None
		}
	}
	
	/// RFC 4291.
	///
	/// Returns a valid address if this is a deprecated compatible address.
	///
	/// Note that there is no corresponding `is_deprecated_internet_protocol_version_4_compatible()` method unlike `get_internet_protocol_version_4_mapped()`, as creation on the result is required to evaluate validity in any event.
	#[inline(always)]
	pub fn get_deprecated_internet_protocol_version_4_compatible(&self) -> Option<InternetProtocolVersion4HostAddress>
	{
		if &self.0[ .. 12] == [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
		{
			let internet_protocol_version_4_host_address = self.internet_protocol_version_4_host_address();
			if internet_protocol_version_4_host_address.is_not_globally_unicast_unique()
			{
				Some(internet_protocol_version_4_host_address)
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn could_be_deprecated_internet_protocol_version_4_compatible(&self) -> bool
	{
		&self.0[ .. 12] == [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
	}
	
	/// RFC 3056: 6to4.
	///
	/// Uses prefix `2002::/16`.
	#[inline(always)]
	pub fn is_6to4(&self) -> bool
	{
		&self.0[ .. 2] == [0x20, 0x02]
	}
	
	/// RFC 7532: Direct Delegation AS112 Service.
	///
	/// Uses prefix `2620:4f:8000::/48`.
	#[inline(always)]
	pub fn is_direct_delegation_as112_service(&self) -> bool
	{
		&self.0[ .. 6] == [0x26, 0x20, 0x00, 0x4F, 0x80, 0x00]
	}
	
	/// RFC 6666: Discard only.
	///
	/// Uses prefix `100::/64`.
	#[inline(always)]
	pub fn is_discard_only(&self) -> bool
	{
		&self.0[ .. 8] == [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
	}
	
	/// RFC 4193 & RFC 8190.
	///
	/// Even though this prefix falls within those that are globally routed, these addresses are not globally routed...
	///
	/// Uses prefix` fc00::/7` but bit 8 is (currently) always specified to be 1.
	/// If matches, then returns `Some((L bit, global identifier, subnet identifier, interface identifier))`.
	#[inline(always)]
	pub fn get_unique_local_unicast(&self) -> Option<(bool, &[u8; 5], &[u8; 2], &[u8; 8])>
	{
		let value = unsafe { *self.0.get_unchecked(0) };
		let l_bit = match value
		{
			0xFC => false,
			0xFD => true,
			_ => None,
		};
		
		Some((l_bit, &self.0[1 .. 6], &self.0[6 .. 8], &self.0[8 .. 16]))
	}
	
	/// RFC 4291.
	///
	/// Uses prefix `fe80::/10` *but* next 54 bits *must* always be zero, so is actually `fe80::/64`.
	#[inline(always)]
	pub fn is_link_local_unicast(&self) -> bool
	{
		&self.0[0 .. 8] == [0xFE, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn get_link_local_unicast_interface_identifier(&self) -> Option<&[u8; 8]>
	{
		if self.is_link_local_unicast()
		{
			Some(&self.0[9..16])
		}
		else
		{
			None
		}
	}
	
	/// RFC2928.
	///
	/// Uses prefix `2001::/23`.
	#[inline(always)]
	pub fn is_assigned_ietf_protocol(&self) -> bool
	{
		&self.0[0 .. 2] == [0x20, 0x01] && (unsafe { *self.0.get_unchecked(2) } >> 1 == 0b000_0000)
	}
	
	/// RFC4380 and RFC8190.
	///
	/// Uses prefix `2001::/32`.
	pub fn is_teredo(&self) -> bool
	{
		&self.0[0 .. 4] == [0x20, 0x01, 0x00, 0x00]
	}
	
	/// RFC5180 and RFC Errata 1752.
	///
	/// Uses prefix `2001:2::/48`.
	pub fn is_benchmarking(&self) -> bool
	{
		&self.0[0 .. 6] == [0x20, 0x01, 0x00, 0x02, 0x00, 0x00]
	}
	
	/// RFC7450.
	///
	/// Uses prefix `2001:3::/32`.
	pub fn is_amt(&self) -> bool
	{
		&self.0[0 .. 4] == [0x20, 0x01, 0x00, 0x03]
	}
	
	/// RFC7535.
	///
	/// Uses prefix `2001:4:112::/48`.
	pub fn is_as112_v6(&self) -> bool
	{
		&self.0[0 .. 6] == [0x20, 0x01, 0x00, 0x04, 0x00, 0x00]
	}
	
	/// RFC7954.
	///
	/// Uses prefix `2001:5::/32`.
	pub fn is_eid_space_for_lisp(&self) -> bool
	{
		&self.0[0 .. 4] == [0x20, 0x01, 0x00, 0x05]
	}
	
	/// RFC4843.
	///
	/// Uses prefix `2001:10::/28`.
	pub fn is_deprecated_orchid(&self) -> bool
	{
		&self.0[0 .. 3] == [0x20, 0x01, 0x10] && (unsafe { *self.0.get_unchecked(3) } >> 4 == 0b0000)
	}
	
	/// RFC7343.
	///
	/// Uses prefix `2001:20::/28`.
	pub fn is_orchid_v2(&self) -> bool
	{
		&self.0[0 .. 3] == [0x20, 0x01, 0x20] && (unsafe { *self.0.get_unchecked(3) } >> 4 == 0b0000)
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_deprecated_site_local_unicast(&self) -> bool
	{
		// 10-bit prefix, 54-bit subnet id and 64-bit interface id
		(unsafe { *self.0.get_unchecked(0) } == 0b11111110) && ((unsafe { *self.0.get_unchecked(1) }) & 0b11000000 == 0b11000000)
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn get_deprecated_site_local_unicast_interface_identifier(&self) -> Option<&[u8; 8]>
	{
		if self.is_deprecated_site_local_unicast()
		{
			Some(&self.0[9..16])
		}
		else
		{
			None
		}
	}
	
	/// Originally RFC 4291 and 4007, updated by RFC 7346.
	#[inline(always)]
	pub fn is_multicast(&self) -> Option<(InternetProtocolVersion6MulticastAddressLifetime, InternetProtocolVersion6MulticastAddressScope)>
	{
		// * The first 8 bits are set to FF.
		// * The next 4 bits are the lifetime of the address: 0 is permanent and 1 is temporary.
		// * The next 4 bits indicate the scope of the multicast address (how far the packet can travel):
		// * 1 is for a node, 2 is for a link, 5 is for the site, 8 is for the organization, and E is global (the Internet).
		
		use self::InternetProtocolVersion6MulticastAddressLifetime::*;
		use self::InternetProtocolVersion6MulticastAddressScope::*;
		
		if self.has_multicast_prefix()
		{
			let byte1 = unsafe { *self.0.get_unchecked(1) };
			
			let flags = byte1 >> 4;
			
			let reserved_high_order_flag = flags & 0b1000;
			if reserved_high_order_flag == 0b1000
			{
				return None;
			}
			
			// P Flag: RFC3306
			// P = 0 indicates a multicast address that is not assigned based on the network prefix.
			// P = 1 indicates a multicast address that is assigned based on the network prefix, and T = 1.
			
			// R Flag: RFC3956
			
			// T Flag.
			let lifetime = match flags & 0b0001
			{
				0x0 => Permanent,
				0x1 => Temporary,
				_ => return None,
			};
			
			// Originally RFC 4291, updated by RFC 7346.
			// 0x0 is reserved; everything else is unassigned.
			let scope = match byte1 & 0xF0
			{
				0x1 => InterfaceLocal, // Aka Node
				0x2 => LinkLocal,
				0x3 => RealmLocal, // RFC 7346, 4007.
				0x4 => AdminLocal, // ?
				0x5 => SiteLocal,
				0x8 => OrganisationLocal,
				0xE => Global,
				_ => return None,
			};
			
			// Remaining 112 bits are group ID.
			
			Some((lifetime, scope))
		}
		else
		{
			None
		}
	}
	
	
	/// Checks for a multicast prefix but does not validate.
	#[inline(always)]
	pub fn has_multicast_prefix(&self) -> bool
	{
		unsafe { *self.0.get_unchecked(0) == 0xFF }
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_multicast_reserved(&self) -> bool
	{
		/*
		    Reserved Multicast Addresses:-
				FF00:0:0:0:0:0:0:0
				FF01:0:0:0:0:0:0:0
				FF02:0:0:0:0:0:0:0
				FF03:0:0:0:0:0:0:0
				FF04:0:0:0:0:0:0:0
				FF05:0:0:0:0:0:0:0
				FF06:0:0:0:0:0:0:0
				FF07:0:0:0:0:0:0:0
				FF08:0:0:0:0:0:0:0
				FF09:0:0:0:0:0:0:0
				FF0A:0:0:0:0:0:0:0
				FF0B:0:0:0:0:0:0:0
				FF0C:0:0:0:0:0:0:0
				FF0D:0:0:0:0:0:0:0
				FF0E:0:0:0:0:0:0:0
				FF0F:0:0:0:0:0:0:0
		*/
		
		if self.has_multicast_prefix()
		{
			(unsafe { *self.0.get_unchecked(1) } <= 0x0F) && &self.0[2 .. 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
		}
		else
		{
			false
		}
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_multicast_all_nodes_interface_local(&self) -> bool
	{
		self == Self::MulticastAllNodesInterfaceLocal
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_multicast_all_nodes_link_local(&self) -> bool
	{
		self == Self::MulticastAllNodesLinkLocal
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_multicast_all_routers_interface_local(&self) -> bool
	{
		self == Self::MulticastAllRoutersInterfaceLocal
	}
	
	/// RFC 4291.
	#[inline(always)]
	pub fn is_multicast_all_routers_link_local(&self) -> bool
	{
		self == Self::MulticastAllRoutersLinkLocal
	}
	
	/// RFC 4291
	#[inline(always)]
	pub fn is_multicast_all_routers_site_local(&self) -> bool
	{
		self == Self::MulticastAllRoutersSiteLocal
	}
	
	/// RFC 4291
	#[inline(always)]
	pub fn is_multicast_solicited_node(&self) -> bool
	{
		&self.0[0 .. 13] == [0xFF, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xFF]
	}
	
	/// RFC 3849.
	#[inline(always)]
	pub fn is_documentation(&self) -> bool
	{
		&self.0[0 .. 4] == [0x20, 0x01, 0x0D, 0xB8]
	}
	
	#[inline(always)]
	fn internet_protocol_version_4_host_address(&self) -> InternetProtocolVersion4HostAddress
	{
		let mut internet_protocol_version_4_host_address = unsafe { uninitialized() };
		unsafe { copy_nonoverlapping(self.0.get_unchecked(12), &mut internet_protocol_version_4_host_address.0, InternetProtocolVersion4HostAddress::Size) };
		internet_protocol_version_4_host_address
	}
}
