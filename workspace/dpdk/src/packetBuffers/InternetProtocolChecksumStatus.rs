// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet protocol (IP) checksum status.
#[deny(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InternetProtocolChecksumStatus
{
	/// No information available about the internet protocol (IP) checksum.
	NoInformationKnown,
	
	/// The internet protocol (IP) checksum in the packet is wrong.
	Bad,
	
	/// The internet protocol (IP) checksum in the packet is valid.
	Good,
	
	/// The internet protocol (IP) checksum is not correct in the packet data, but the integrity of the internet protocol (IP) header was verified.
	IncorrectButInternetProtocolHeaderIntegrityVerified,
}
