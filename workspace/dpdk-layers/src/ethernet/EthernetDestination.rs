// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Contains the essential properties needed for an outgoing ethernet destination.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EthernetDestination
{
	/// The destination's media access control address.
	pub media_access_control_address: MediaAccessControlAddress,
	
	/// The destination's ethernet frame length.
	#[serde(default)] pub ethernet_frame_length: EthernetFrameLength,
}

impl Display for EthernetDestination
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}, {}", self.media_access_control_address, self.ethernet_frame_length)
	}
}
