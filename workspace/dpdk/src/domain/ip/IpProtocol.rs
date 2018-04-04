// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum IpProtocol
{
	V4 = ::dpdk_sys::ETHER_TYPE_IPv4,
	V6 = ::dpdk_sys::ETHER_TYPE_IPv6,
}

impl IpProtocol
{
	#[inline(always)]
	pub fn asEtherTypeBigEndian(self) -> u16
	{
		(self as u16).to_be()
	}
	
	#[inline(always)]
	pub fn writeLayer3Header(self, buffer: *mut u8, differentiatedServiceCodePoint: DifferentiatedServiceCodePoint, hopLimits: u8, layer4Protocol: Layer4Protocol) -> (usize, usize)
	{
		match self
		{
			IpProtocol::V4 =>
			{
				// Top 4 bits
				const Version: u8 = 4 << 4;
				
				// Bottom 4 bits
				let IpHeaderLengthIn32BitWordsAssumingNoOptions = (size_of::<ipv4_hdr>() as u8 / ::dpdk_sys::IPV4_IHL_MULTIPLIER) as u8; // ie 20 / 4 => 5
				
				let trafficClass = TrafficClass
				{
					differentiatedServiceCodePoint: differentiatedServiceCodePoint,
					explicitCongestionNotification: ExplicitCongestionNotification::NotCapableTransport,
				};
				
				let mut header = unsafe { *(buffer as *mut ipv4_hdr) };
				
				// Commented out fields are overwritten by TLDK without reading them first
				header.version_ihl = Version | IpHeaderLengthIn32BitWordsAssumingNoOptions;
				header.type_of_service = trafficClass.as_u8();
				//header.total_length = 0;
				//header.packet_id = 0;
				header.fragment_offset = 0;
				header.time_to_live = hopLimits;
				header.next_proto_id = layer4Protocol.libcValue();
				header.hdr_checksum = 0;
				//header.src_addr = 0;
				//header.dst_addr = 0;
				
				const TrailingAddressBytesLength: usize = SizeOfIpV4HostAddress * 2;
				
				(size_of::<ipv4_hdr>(), TrailingAddressBytesLength)
			},
			
			IpProtocol::V6 =>
			{
				// Top, Bottom as for Little Endian by analogy to TCI
				
				// Top 4 bits
				const Version: u32 = 6 << 24;
				
				// Middle 8 bits
				let trafficClass = TrafficClass
				{
					differentiatedServiceCodePoint: differentiatedServiceCodePoint,
					explicitCongestionNotification: ExplicitCongestionNotification::NotCapableTransport,
				};
				
				// Bottom 20 bits
				const FlowLabel: u32 = 0;
				
				let mut header = unsafe { *(buffer as *mut ipv6_hdr) };
				
				// Commented out fields are overwritten by TLDK
				header.vtc_flow = (Version | (trafficClass.as_u8() as u32) << 20 | FlowLabel).to_be();
				//header.payload_len = 0;
				header.proto = layer4Protocol.libcValue();
				header.hop_limits = hopLimits;
				//header.src_addr = unsafe { zeroed() };
				//header.dst_addr= unsafe { zeroed() };
				
				const TrailingAddressBytesLength: usize = SizeOfIpV6HostAddress * 2;
				
				(size_of::<ipv6_hdr>(), TrailingAddressBytesLength)
			},
		}
	}
}
