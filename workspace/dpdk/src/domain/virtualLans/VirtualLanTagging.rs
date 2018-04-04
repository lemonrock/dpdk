// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualLanTagging
{
	None,
	Single(VirtualLanTrafficClassIndicator),
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
	#[inline(always)]
	pub fn size(&self) -> usize
	{
		match *self
		{
			VirtualLanTagging::None =>
			{
				0
			},
			
			VirtualLanTagging::Single(_) =>
			{
				SizeOfVlanHeader as usize
			},
			
			VirtualLanTagging::Double(_, _) =>
			{
				(SizeOfQinQHeader + SizeOfVlanHeader) as usize
			},
		}
	}
	
	#[inline(always)]
	pub fn writeLayer2HeaderData(&self, buffer: *mut u8, etherType: u16) -> usize
	{
		macro_rules! writeEtherType
		{
			($buffer: ident, $etherType: ident, $offsetToEtherTypeU32: expr) =>
			{
				{
					const offsetToEtherTypeUsize: usize = $offsetToEtherTypeU32 as usize;
					$buffer.offsetUp(offsetToEtherTypeUsize).writeU16AsNetworkByteOrderU16(etherType);
					
					const offset: usize = offsetToEtherTypeUsize + SizeOfEtherType as usize;
					offset
				}
			}
		}
		
		match *self
		{
			VirtualLanTagging::None =>
			{
				writeEtherType!(buffer, etherType, 0)
			},
			
			VirtualLanTagging::Single(innerTrafficClassIndicator) =>
			{
				innerTrafficClassIndicator.writeLayer2HeaderData(buffer, ETHER_TYPE_VLAN);
				writeEtherType!(buffer, etherType, SizeOfVlanHeader)
			},
			
			VirtualLanTagging::Double(outerTrafficClassIndicator, innerTrafficClassIndicator) =>
			{
				const AnOffset: usize = SizeOfQinQHeader as usize;
				
				outerTrafficClassIndicator.writeLayer2HeaderData(buffer, ETHER_TYPE_QINQ);
				innerTrafficClassIndicator.writeLayer2HeaderData(buffer.offsetUp(AnOffset), ETHER_TYPE_VLAN);
				writeEtherType!(buffer, etherType, SizeOfQinQHeader + SizeOfVlanHeader)
			},
		}
	}
	
	#[inline(always)]
	pub fn virtualLanKey(&self) -> VirtualLanKey
	{
		match *self
		{
			VirtualLanTagging::None => (None, None),
			VirtualLanTagging::Single(ref inner) => (None, inner.virtualLanId),
			VirtualLanTagging::Double(ref outer, inner) => (outer.virtualLanId, inner.virtualLanId),
		}
	}
}
