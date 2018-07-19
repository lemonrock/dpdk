// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


macro_rules! drop
{
	($reason: expr, $packet_processing_or_packet_processing_by_virtual_lan: ident, $packet: ident) =>
	{
		{
			$packet_processing_or_packet_processing_by_virtual_lan.dropped_packet($reason);
			$packet.free_direct_contiguous_packet();
			return
		}
	}
}
