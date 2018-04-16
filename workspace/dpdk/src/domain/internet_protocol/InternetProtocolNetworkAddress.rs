// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An internet protocol (IP) network address, either version 4 or version 6.
pub trait InternetProtocolNetworkAddress
{
	/// Associated Internet Protocol (IP) host address, either version 4 or 6.
	type InternetProtocolHostAddress: Sized;
	
	/// Network information.
	#[inline(always)]
	fn network(&self) -> &Self::InternetProtocolHostAddress;
	
	/// Number of mask bits, one based.
	///
	/// eg `/24` would be `24`.
	#[inline(always)]
	fn mask_bits_as_depth(&self) -> u8;
	
	/// Does this network address contain the given `internet_protocol_host_address`?
	///
	/// In other words, is the given `internet_protocol_host_address` prefixed by this network address?
	#[inline(always)]
	fn contains(&self, internet_protocol_host_address: Self::InternetProtocolHostAddress) -> bool;
}
