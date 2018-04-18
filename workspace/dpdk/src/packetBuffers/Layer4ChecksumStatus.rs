// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Layer 4 (TCP, UDP, SCTP) checksum status.
#[deny(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Layer4ChecksumStatus
{
	/// No information available about the layer 4 (TCP, UDP, SCTP) checksum.
	NoInformationKnown,
	
	/// The layer 4 (TCP, UDP, SCTP) checksum in the packet is wrong.
	Bad,
	
	/// The layer 4 (TCP, UDP, SCTP) checksum in the packet is valid.
	Good,
	
	/// The layer 4 (TCP, UDP, SCTP) checksum is not correct in the packet data, but the integrity of the layer 4 data was verified.
	IncorrectButLayer4DataIntegrityVerified,
}
