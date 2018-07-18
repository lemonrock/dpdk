// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Known layer 4 protocol numbers.
///
/// This is a deliberately minimal list.
///
/// See <https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Layer4ProtocolNumber
{
	/// Internet Control Message Protocol (ICMP).
	///
	/// RFC 792.
	InternetControlMessageProtocol = 1,
	
	/// Internet Group Management Protocol (IGMP).
	///
	/// RFC 1122.
	InternetGroupManagementProtocol = 2,
	
	/// Transmission Control Protocol (TCP).
	///
	/// RFC 793.
	TransmissionControlProtocol = 6,
	
	/// User Datagram Protocol (UDP).
	///
	/// RFC 768.
	UserDatagramProtocol = 17,
}
