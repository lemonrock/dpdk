// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
struct InternetProtocolVersion6FirstFragmentDetails
{
	offset_of_ethernet_packet_header: usize,
	per_fragment_extension_headers_length: usize,
	offset_of_next_header_to_change_relative_to_start_of_internet_protocol_version_6_packet_header: usize,
	next_header: u8,
}
