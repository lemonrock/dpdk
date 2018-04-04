// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct MediaAccessControlAddress(pub ether_addr);

impl Display for MediaAccessControlAddress
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		let bytes = self.0.addr_bytes;
		write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5])
	}
}

impl Serialize for MediaAccessControlAddress
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_str(&format!("{}", self))
	}
}

impl Deserialize for MediaAccessControlAddress
{
	fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error>
	{
		struct FromString;
		
		impl Visitor for FromString
		{
			type Value = MediaAccessControlAddress;
			
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				formatter.write_str("string of 6 2-byte hexadecimal values separated by colons, eg 00:AA:BB:CC:DD:EE")
			}
			
			fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E>
			{
				fn next<'a, E: Error>(splits: &mut SplitN<'a, char>) -> Result<u8, E>
				{
					if let Some(hexadecimalByteWithoutPrefix) = splits.next()
					{
						match u8::from_str_radix(hexadecimalByteWithoutPrefix, 16)
						{
							Ok(value) => Ok(value),
							Err(_) => Err(E::custom("Could not convert hexadecimal byte in MediaAccessControlAddress")),
						}
					}
					else
					{
						Err(E::custom("Less than 6 hexadecimal bytes in MediaAccessControlAddress"))
					}
				}
				
				let splits = &mut value.splitn(6, ':');
				
				let inner = ether_addr
				{
					addr_bytes:
					[
						next(splits)?,
						next(splits)?,
						next(splits)?,
						next(splits)?,
						next(splits)?,
						next(splits)?,
					],
				};
				
				if splits.next().is_some()
				{
					Err(E::custom("More than 6 hexadecimal bytes"))
				}
				else
				{
					Ok(MediaAccessControlAddress(inner))
				}
			}
		}
		
		deserializer.deserialize(FromString)
	}
}

impl MediaAccessControlAddress
{
	pub const Zero: MediaAccessControlAddress = MediaAccessControlAddress
	(
		ether_addr
		{
			addr_bytes: [0; 6]
		}
	);
	
	#[inline(always)]
	pub fn ethernetAddressIsInvalid(ethernetAddress: *const ether_addr) -> bool
	{
		isFalse(unsafe { rust_is_valid_assigned_ether_addr(ethernetAddress) })
	}
	
	#[inline(always)]
	pub fn isTargetHardwareAddressNotZero(targetHardwareAddress: *const ether_addr) -> bool
	{
		isFalse(unsafe { rust_is_zero_ether_addr(targetHardwareAddress) })
	}
	
	// Aside from RFC 1122 § 2.3.2.1, which is a minor feature to re-validate cached ARP entries, there is no good reason to receive unicast (or indeed multicast, or anything other than broadcast) ARP requests
	// See first answer at https://security.stackexchange.com/questions/58131/unicast-arp-requests-considered-harmful for a longer discussion
	// Consequently we consider anything other than ARP requests with a broadcast as invalid
	#[inline(always)]
	pub fn destinationEthernetAddressIsInvalidForAnArpRequest(ethernetAddress: *const ether_addr) -> bool
	{
		isFalse(unsafe { rust_is_broadcast_ether_addr(ethernetAddress) })
	}
	
	#[inline(always)]
	pub fn fromBytes(bytes: [u8; 6]) -> Self
	{
		MediaAccessControlAddress
		(
			ether_addr
			{
				addr_bytes: bytes,
			}
		)
	}
	
	#[inline(always)]
	pub fn random() -> MediaAccessControlAddress
	{
		let mut bytes: [uint8_t; 6usize] = unsafe { uninitialized() };
		unsafe { rust_eth_random_addr(bytes.as_mut_ptr()) };
		MediaAccessControlAddress(ether_addr
		{
			addr_bytes: bytes
		})
	}
	
	#[inline(always)]
	pub fn isZero(&self) -> bool
	{
		isTrue(unsafe { rust_is_zero_ether_addr(&self.0) })
	}
	
	#[inline(always)]
	pub fn isUnicast(&self) -> bool
	{
		isTrue(unsafe { rust_is_unicast_ether_addr(&self.0) })
	}
	
	#[inline(always)]
	pub fn isMulticast(&self) -> bool
	{
		isTrue(unsafe { rust_is_multicast_ether_addr(&self.0) })
	}
	
	#[inline(always)]
	pub fn isBroadcast(&self) -> bool
	{
		isTrue(unsafe { rust_is_broadcast_ether_addr(&self.0) })
	}
	
	#[inline(always)]
	pub fn isUniversal(&self) -> bool
	{
		isTrue(unsafe { rust_is_universal_ether_addr(&self.0) })
	}
	
	#[inline(always)]
	pub fn isLocalAdminAssigned(&self) -> bool
	{
		isTrue(unsafe { rust_is_local_admin_ether_addr(&self.0) })
	}
	
	#[inline(always)]
	pub fn isValidAssigned(&self) -> bool
	{
		isTrue(unsafe { rust_is_valid_assigned_ether_addr(&self.0) })
	}
	
	#[inline(always)]
	pub fn isInvalidAssigned(&self) -> bool
	{
		isFalse(unsafe { rust_is_valid_assigned_ether_addr(&self.0) })
	}
}

pub trait InternalList<T>
{
	fn internalMutablePointer(&mut self) -> *mut T;
}

impl<'a> InternalList<ether_addr> for &'a mut [MediaAccessControlAddress]
{
	fn internalMutablePointer(&mut self) -> *mut ether_addr
	{
		unsafe { ::std::mem::transmute(self.as_mut_ptr()) }
	}
}
