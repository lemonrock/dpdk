// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// ARP operations.
///
/// Representation as an u16 is in native endian order.
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operation
{
	REQUEST = 1,
	REPLY = 2,
	request_Reverse = 3,
	reply_Reverse = 4,
	DRARP_Request = 5,
	DRARP_Reply = 6,
	DRARP_Error = 7,
	InARP_Request = 8,
	InARP_Reply = 9,
	ARP_NAK = 10,
	MARS_Request = 11,
	MARS_Multi = 12,
	MARS_MServ = 13,
	MARS_Join = 14,
	MARS_Leave = 15,
	MARS_NAK = 16,
	MARS_Unserv = 17,
	MARS_SJoin = 18,
	MARS_SLeave = 19,
	MARS_Grouplist_Request = 20,
	MARS_Grouplist_Reply = 21,
	MARS_Redirect_Map = 22,
	MAPOS_UNARP = 23,
	OP_EXP1 = 24,
	OP_EXP2 = 25,
}

impl Operation
{
	/// Convert ARP operation to network byte order.
	#[inline(always)]
	pub fn to_network_byte_order(self) -> NetworkByteOrderEndianU16
	{
		NetworkByteOrderEndianU16::from_native_byte_order_value(self as u16)
	}
	
	/// Parse ARP operation from network byte order.
	#[inline(always)]
	pub fn parse_from_network_byte_order(operation: u16) -> Result<Self, ParseError>
	{
		use self::ParseError::*;
		use self::Operation::*;
		
		match operation.to_native_byte_order_value()
		{
			0 => Err(Reserved(0)),
			1 => Ok(REQUEST),
			2 => Ok(REPLY),
			3 => Ok(request_Reverse),
			4 => Ok(reply_Reverse),
			5 => Ok(DRARP_Request),
			6 => Ok(DRARP_Reply),
			7 => Ok(DRARP_Error),
			8 => Ok(InARP_Request),
			9 => Ok(InARP_Reply),
			10 => Ok(ARP_NAK),
			11 => Ok(MARS_Request),
			12 => Ok(MARS_Multi),
			13 => Ok(MARS_MServ),
			14 => Ok(MARS_Join),
			15 => Ok(MARS_Leave),
			16 => Ok(MARS_NAK),
			17 => Ok(MARS_Unserv),
			18 => Ok(MARS_SJoin),
			19 => Ok(MARS_SLeave),
			20 => Ok(MARS_Grouplist_Request),
			21 => Ok(MARS_Grouplist_Reply),
			22 => Ok(MARS_Redirect_Map),
			23 => Ok(MAPOS_UNARP),
			24 => Ok(OP_EXP1),
			25 => Ok(OP_EXP2),
			unassigned @ 26 ... 65534 => Err(Unassigned(unassigned)),
			65535 => Err(Reserved(65535)),
			unassigned @ _ => Err(Unassigned(unassigned)),
		}
	}
	
}
