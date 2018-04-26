// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents common internet control message protocol (ICMP) types.
///
/// Deprecated, unassigned, reserved and experimental types are not provided for.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternetControlMessageProtocolType(pub u8);

impl InternetControlMessageProtocolPacketType
{
	/// Echo Reply ('pong').
	pub const EchoReply: Self = InternetControlMessageProtocolType(0);
	
	/// Destination Unreachable
	pub const DestinationUnreachable: Self = InternetControlMessageProtocolType(3);
	
	/// Redirect Message.
	pub const RedirectMessage: Self = InternetControlMessageProtocolType(5);
	
	/// Echo Request ('ping').
	pub const EchoRequest: Self = InternetControlMessageProtocolType(8);
	
	/// Router Advertisement.
	pub const RouterAdvertisement: Self = InternetControlMessageProtocolType(9);
	
	/// Router Solicitation (request).
	pub const RouterSolicitation: Self = InternetControlMessageProtocolType(10);
	
	/// Time exceeded (TTL).
	pub const TimeExceeded: Self = InternetControlMessageProtocolType(11);
	
	/// Bad Internet Protocol (IP) version 4 header.
	pub const BadInternetProtocolVersion4Header: Self = InternetControlMessageProtocolType(12);
	
	/// Timestamp Request.
	pub const TimestampRequest: Self = InternetControlMessageProtocolType(13);
	
	/// Timestamp Reply.
	pub const TimestampReply: Self = InternetControlMessageProtocolType(14);
}
