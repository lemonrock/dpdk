// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternetControlMessageProtocolPacketHeader
{
	/// Type.
	pub type_: InternetControlMessageProtocolType,
	
	/// The meaning of code depends on type.
	pub code: u8,
	
	/// The checksum includes the payload.
	pub checksum: InternetChecksum,
	
	/// Rest-of-header.
	pub rest_of_header: RestOfHeader,
}

impl Display for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Into<icmp_hdr> for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> icmp_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a icmp_hdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a icmp_hdr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<NonNull<icmp_hdr>> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<icmp_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut InternetControlMessageProtocolPacketHeader as *mut icmp_hdr) }
	}
}

impl<'a> Into<*const icmp_hdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> *const icmp_hdr
	{
		self as *const InternetControlMessageProtocolPacketHeader as *const _
	}
}

impl<'a> Into<*mut icmp_hdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut icmp_hdr
	{
		self as *mut InternetControlMessageProtocolPacketHeader as *mut _
	}
}

impl<'a> Into<&'a mut icmp_hdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut icmp_hdr
	{
		unsafe { transmute(self) }
	}
}

impl From<icmp_hdr> for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: icmp_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a icmp_hdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: &'a icmp_hdr) -> &'a InternetControlMessageProtocolPacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a mut icmp_hdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut icmp_hdr) -> &'a mut InternetControlMessageProtocolPacketHeader
	{
		unsafe { transmute(value) }
	}
}
