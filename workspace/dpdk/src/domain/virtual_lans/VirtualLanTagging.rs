// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Virtual LAN tagging kind.
///
/// Defaults to None (ie, no virtual LAN tagging).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualLanTagging
{
	#[allow(missing_docs)]
	None,
	
	#[allow(missing_docs)]
	Single(VirtualLanTrafficClassIndicator),
	
	#[allow(missing_docs)]
	Double(VirtualLanTrafficClassIndicator, VirtualLanTrafficClassIndicator),
}

impl Default for VirtualLanTagging
{
	#[inline(always)]
	fn default() -> Self
	{
		VirtualLanTagging::None
	}
}

impl VirtualLanTagging
{
	/// Size in bytes occupied in a Layer 2 header.
	#[inline(always)]
	pub fn size(&self) -> usize
	{
		use self::VirtualLanTagging::*;
		
		match *self
		{
			None => 0,
			
			Single(_) => SizeOfVlanHeader as usize,
			
			Double(_, _) => (SizeOfQinQHeader + SizeOfVlanHeader) as usize,
		}
	}
	
	/// Writes Layer 2 header data.
	#[inline(always)]
	pub fn write_layer_2_header_data(&self, buffer: *mut u8, ether_type: u16) -> usize
	{
		macro_rules! write_ether_type
		{
			($buffer: ident, $ether_type: ident, $offset_to_ether_type_u32: expr) =>
			{
				{
					const offset_to_ether_type_usize: usize = $offset_to_ether_type_u32 as usize;
					$buffer.offsetUp(offset_to_ether_type_usize).writeU16AsNetworkByteOrderU16(ether_type);
					
					const offset: usize = offset_to_ether_type_usize + SizeOfEtherType as usize;
					offset
				}
			}
		}
		
		use self::VirtualLanTagging::*;
		
		match *self
		{
			None => write_ether_type!(buffer, ether_type, 0),
			
			Single(innerTrafficClassIndicator) =>
			{
				innerTrafficClassIndicator.write_layer_2_header_data(buffer, ETHER_TYPE_VLAN);
				write_ether_type!(buffer, ether_type, SizeOfVlanHeader)
			}
			
			Double(outerTrafficClassIndicator, innerTrafficClassIndicator) =>
			{
				const AnOffset: usize = SizeOfQinQHeader as usize;
				
				outerTrafficClassIndicator.write_layer_2_header_data(buffer, ETHER_TYPE_QINQ);
				innerTrafficClassIndicator.write_layer_2_header_data(buffer.offsetUp(AnOffset), ETHER_TYPE_VLAN);
				write_ether_type!(buffer, ether_type, SizeOfQinQHeader + SizeOfVlanHeader)
			}
		}
	}
	
	/// Virtual LAN key.
	#[inline(always)]
	pub fn virtual_lan_key(&self) -> VirtualLanKey
	{
		use self::VirtualLanTagging::*;
		
		match *self
		{
			None => (None, None),
			Single(ref inner) => (None, inner.virtual_lan_id),
			Double(ref outer, inner) => (outer.virtual_lan_id, inner.virtual_lan_id),
		}
	}
}
