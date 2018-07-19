// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ether type or legacy ethernet frame size?
#[repr(C, packed)]
pub union EtherTypeOrLegacyEthernetFrameSize
{
	/// Legacy ethernet frame size.
	pub legacy_ethernet_frame_size: LegacyEthernetFrameSize,
	
	/// Ether Type.
	pub ether_type: EtherType,
}

impl Clone for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			legacy_ethernet_frame_size: self.legacy_ethernet_frame_size,
		}
	}
}

impl Copy for EtherTypeOrLegacyEthernetFrameSize
{
}

impl PartialOrd for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.legacy_ethernet_frame_size.partial_cmp(&other.legacy_ethernet_frame_size) }
	}
}

impl Ord for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.legacy_ethernet_frame_size.cmp(&other.legacy_ethernet_frame_size) }
	}
}

impl PartialEq for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.legacy_ethernet_frame_size == other.legacy_ethernet_frame_size }
	}
}

impl Eq for EtherTypeOrLegacyEthernetFrameSize
{
}

impl Hash for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.legacy_ethernet_frame_size.hash(hasher) }
	}
}

impl Debug for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", unsafe { self.legacy_ethernet_frame_size.0.to_native_byte_order_value() })
	}
}

impl Display for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", unsafe { self.legacy_ethernet_frame_size.0.to_native_byte_order_value() })
	}
}

impl EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	pub(crate) fn potentially_invalid_ether_type(&self) -> EtherType
	{
		unsafe { self.ether_type }
	}
}
