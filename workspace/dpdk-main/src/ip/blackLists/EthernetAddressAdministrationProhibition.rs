// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


const LocalAdministerBit: u8 = 0b10;
const LocalAdministerBitSet: u8 = LocalAdministerBit;
const LocalAdministerBitUnset: u8 = 0b00;
const DoNotCare: u8 = 0xFF;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum EthernetAddressAdministrationProhibition
{
	LocallyAdministered = LocalAdministerBitSet,
	UniversallyAdministered = LocalAdministerBitUnset,
	None = DoNotCare,
}

impl Default for EthernetAddressAdministrationProhibition
{
	#[inline(always)]
	fn default() -> Self
	{
		EthernetAddressAdministrationProhibition::None
	}
}

impl EthernetAddressAdministrationProhibition
{
	#[inline(always)]
	fn isProhibited(&self, ethernetAddress: *const ether_addr) -> bool
	{
		(unsafe { *(ethernetAddress as *const u8) }) & LocalAdministerBit == *self as u8
	}
}
