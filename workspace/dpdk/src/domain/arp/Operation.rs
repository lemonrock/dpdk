// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//noinspection SpellCheckingInspection
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
	/// AKA Big Endian
	#[inline(always)]
	pub fn toNetworkByteOrder(self) -> u16
	{
		(self as u16).to_be()
	}
	
	#[inline(always)]
	pub fn parseFromNetworkByteOrder(bigEndianValue: u16) -> Result<Operation, ParseError>
	{
		match u16::from_be(bigEndianValue)
		{
			0 => Err(ParseError::Reserved(0)),
			1 => Ok(Operation::REQUEST),
			2 => Ok(Operation::REPLY),
			3 => Ok(Operation::request_Reverse),
			4 => Ok(Operation::reply_Reverse),
			5 => Ok(Operation::DRARP_Request),
			6 => Ok(Operation::DRARP_Reply),
			7 => Ok(Operation::DRARP_Error),
			8 => Ok(Operation::InARP_Request),
			9 => Ok(Operation::InARP_Reply),
			10 => Ok(Operation::ARP_NAK),
			11 => Ok(Operation::MARS_Request),
			12 => Ok(Operation::MARS_Multi),
			13 => Ok(Operation::MARS_MServ),
			14 => Ok(Operation::MARS_Join),
			15 => Ok(Operation::MARS_Leave),
			16 => Ok(Operation::MARS_NAK),
			17 => Ok(Operation::MARS_Unserv),
			18 => Ok(Operation::MARS_SJoin),
			19 => Ok(Operation::MARS_SLeave),
			20 => Ok(Operation::MARS_Grouplist_Request),
			21 => Ok(Operation::MARS_Grouplist_Reply),
			22 => Ok(Operation::MARS_Redirect_Map),
			23 => Ok(Operation::MAPOS_UNARP),
			24 => Ok(Operation::OP_EXP1),
			25 => Ok(Operation::OP_EXP2),
			unassigned @ 26 ... 65534 => Err(ParseError::Unassigned(unassigned)),
			65535 => Err(ParseError::Reserved(65535)),
			unassigned @ _ => Err(ParseError::Unassigned(unassigned)),
		}
	}
	
}
