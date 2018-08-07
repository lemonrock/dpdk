// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


include!("InternetProtocolPacketReassemblyTable.rs");
include!("InternetProtocolPacketReassemblyTableConfiguration.rs");

include!("Fields.rs");
include!("FragmentListNode.rs");
include!("InternetProtocolVersion6FirstFragmentDetails.rs");
include!("InternetProtocolVersion6FragmentList.rs");
include!("InternetProtocolVersion4ReassemblyKey.rs");
include!("InternetProtocolVersion6ReassemblyKey.rs");
include!("TimestampReusedAsFields.rs");


pub(crate) struct InternetProtocolVersion6Reassembly
{
	// TODO: Use our tcp-engine collections here.
	xxx;
	lru_cache_with_expiry: HashMap<ReassemblyKey, InternetProtocolVersion6FragmentList>,
}

impl InternetProtocolVersion6Reassembly
{
	#[inline(always)]
	pub(crate) fn process_fragment(&mut self, now_timestamp_in_milliseconds: u64, key: InternetProtocolVersion6ReassemblyKey, internet_protocol_version_6_packet_header: &InternetProtocolVersion6PacketHeader, packet_buffer: PacketBuffer, offset: u16, length: u16, fragmented_payload_offset: u16, has_more_flag_set: bool, fragment_details: InternetProtocolVersion6FirstFragmentDetails) -> Result<Option<PacketBuffer>, ()>
	{
		if let Some(extant) = self.lru_cache_with_expiry.get_mut(&key)
		{
			if extant.expired()
			{
				// TODO delete entry.
				xxxx;
				return Err(())
			}
			
			extant.process_fragment(internet_protocol_version_6_packet_header, packet_buffer, offset, length, fragmented_payload_offset, has_more_flag_set, fragment_details)
		}
		else
		{
			let new = InternetProtocolVersion6FragmentList::new(now_timestamp_in_milliseconds, internet_protocol_version_6_packet_header, packet_buffer, offset, length, fragmented_payload_offset, has_more_flag_set, fragment_details)?;
			
			self.lru_cache_with_expiry.entry(key).or_default(new)
		}
	}
}
