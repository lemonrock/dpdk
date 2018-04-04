// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(non_camel_case_types)]
pub trait rte_mbufEx
{
	#[inline(always)]
	fn wasVlanTagControlInformationStripped(self) -> bool;
	
	#[inline(always)]
	fn strippedVlanTagControlInformation(self) -> u16;
	
	#[inline(always)]
	fn wasVlanQinQTagControlInformationStripped(self) -> bool;
	
	#[inline(always)]
	fn strippedVlanQInQTagControlInformation(self) -> u16;
	
	#[inline(always)]
	fn hasOlFlag(self, flag: u64) -> bool;
	
	#[inline(always)]
	fn ol_flags(self) -> u64;
	
	#[inline(always)]
	fn packetTypeFlags(self) -> u32;
	
	#[inline(always)]
	fn length(self) -> u32;
	
	#[inline(always)]
	fn ignore(self);
	
	#[inline(always)]
	fn free(self);
	
	#[inline(always)]
	fn debugAssertSelfIsNotNull(self);
}

impl rte_mbufEx for *mut rte_mbuf
{
	#[inline(always)]
	fn wasVlanTagControlInformationStripped(self) -> bool
	{
		self.hasOlFlag(PKT_RX_VLAN_STRIPPED)
	}
	
	#[inline(always)]
	fn strippedVlanTagControlInformation(self) -> u16
	{
		self.debugAssertSelfIsNotNull();
		
		(unsafe {*self}).vlan_tci
	}
	
	#[inline(always)]
	fn wasVlanQinQTagControlInformationStripped(self) -> bool
	{
		self.hasOlFlag(PKT_RX_QINQ_STRIPPED)
	}
	
	#[inline(always)]
	fn strippedVlanQInQTagControlInformation(self) -> u16
	{
		self.debugAssertSelfIsNotNull();
		
		(unsafe {*self}).vlan_tci_outer
	}
	
	#[inline(always)]
	fn hasOlFlag(self, flag: u64) -> bool
	{
		(self.ol_flags() & flag) == flag
	}
	
	#[inline(always)]
	fn ol_flags(self) -> u64
	{
		self.debugAssertSelfIsNotNull();
		
		(unsafe { *self }).ol_flags
	}
	
	#[inline(always)]
	fn packetTypeFlags(self) -> u32
	{
		self.debugAssertSelfIsNotNull();
		
		(unsafe {*self}).packet_type
	}
	
	#[inline(always)]
	fn length(self) -> u32
	{
		self.debugAssertSelfIsNotNull();
		
		(unsafe {*self}).pkt_len
	}
	
	#[inline(always)]
	fn ignore(self)
	{
		self.debugAssertSelfIsNotNull();
		
		(unsafe {*self}).packet_type = RTE_PTYPE_UNKNOWN
	}
	
	#[inline(always)]
	fn free(self)
	{
		self.debugAssertSelfIsNotNull();
		
		unsafe { rust_rte_pktmbuf_free(self) };
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn debugAssertSelfIsNotNull(self)
	{
		debug_assert!(!self.is_null(), "self (rte_mbuf packet) is null");
	}
	
	#[cfg(not(debug_assertions))]
	#[inline(always)]
	fn debugAssertSelfIsNotNull(self)
	{
	}
}
