// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Known extension header types.
///
/// See <https://www.iana.org/assignments/ipv6-parameters/ipv6-parameters.xhtml>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ExtensionHeaderType
{
	/// RFC 8200.
	HopByHopOptions = 0,
	
	/// RFC 8200.
	Routing = 43,
	
	/// RFC 8200.
	Fragment = 44,
	
	/// RFC 4303.
	EncapulatingSecurityPayload = 50,
	
	/// RFC 4302.
	AuthenticationHeader = 51,
	
	/// RFC 8200.
	///
	/// Note that this is not listed by IANA on the web page <https://www.iana.org/assignments/ipv6-parameters/ipv6-parameters.xhtml> but *IS* listed on the web page <https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml>.
	NoNextHeader = 59,
	
	/// RFC 8200.
	DestinationOptions = 60,
	
	/// RFC 6275.
	MobilityHeader = 135,
	
	/// RFC 7401.
	HostIdentityProtocol = 139,
	
	/// RFC 5533.
	Shim6Protocol = 140,
	
	/// RFC 4727.
	Experimentation1 = 253,
	
	/// RFC 4727.
	Experimentation2 = 254,
}
