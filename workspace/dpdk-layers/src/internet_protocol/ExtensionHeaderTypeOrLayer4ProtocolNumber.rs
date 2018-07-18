// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension header type or Layer 4 protocol number (the range overlaps; yuck).
#[repr(C, packed)]
pub union ExtensionHeaderTypeOrLayer4ProtocolNumber
{
	/// A known extension header type.
	pub extension_header_type: ExtensionHeaderType,
	
	/// A known layer 4 protocol number.
	pub layer_4_protocol_number: Layer4ProtocolNumber,
	
	/// An unknown extension header type or layer 4 protocol number.
	pub unknown: u8,
}
