// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Protocol (IP) version.
///
/// Representation as an u16 is in native endian order.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u16)]
pub enum InternetProtocolVersion
{
	V4 = ETHER_TYPE_IPv4,
	V6 = ETHER_TYPE_IPv6,
}

impl InternetProtocolVersion
{
	/// To EtherType.
	#[inline(always)]
	pub fn to_ether_type(self) -> NetworkByteOrderEndianU16
	{
		NetworkByteOrderEndianU16::from_native_byte_order_value(self as u16)
	}
	
	/// Writes a Layer 3 (internet protocol (IP)) header to a `buffer`.
	///
	/// `buffer` should be either an `ipv4_hdr` or an `ipv6_hdr`.
	#[inline(always)]
	pub fn write_layer_3_header(self, buffer: NonNull<u8>, differentiated_service_code_point: DifferentiatedServiceCodePoint, hop_limits: u8, layer_4_protocol: Layer4Protocol) -> (usize, usize)
	{
		use self::InternetProtocolVersion::*;
		
		let traffic_class = TrafficClass
		{
			differentiated_service_code_point,
			explicit_congestion_notification: ExplicitCongestionNotification::default(),
		};
		
		match self
		{
			V4 =>
			{
				// Top 4 bits
				const Version: u8 = 4;

				// Bottom 4 bits
				let IpHeaderLengthIn32BitWordsAssumingNoOptions = (size_of::<ipv4_hdr>() as u8 / IPV4_IHL_MULTIPLIER) as u8; // ie 20 / 4 => 5
				
				// Commented out fields are overwritten by TLDK without reading them first
				let header = unsafe { &mut * (buffer.as_ptr() as *mut ipv4_hdr) };
				header.version_ihl = Version << 4 | IpHeaderLengthIn32BitWordsAssumingNoOptions;
				header.type_of_service = traffic_class.as_u8();
				//header.total_length = 0;
				//header.packet_id = 0;
				header.fragment_offset = 0;
				header.time_to_live = hop_limits;
				header.next_proto_id = layer_4_protocol.libc_value();
				header.hdr_checksum = 0;
				//header.src_addr = 0;
				//header.dst_addr = 0;

				const TrailingAddressBytesLength: usize = InternetProtocolVersion4HostAddress::Size * 2;

				(size_of::<ipv4_hdr>(), TrailingAddressBytesLength)
			},

			V6 =>
			{
				// Top, Bottom as for Little Endian by analogy to TCI.

				// Top 4 bits
				const Version: u32 = 6;

				// Bottom 20 bits
				const FlowLabel: u32 = 0;

				// Commented out fields are overwritten by TLDK
				let header = unsafe { &mut * (buffer.as_ptr() as *mut ipv6_hdr) };
				header.vtc_flow = (Version << 24 | traffic_class.as_u32() << 20 | FlowLabel).to_be();
				//header.payload_length_including_extension_headers = 0;
				header.proto = layer_4_protocol.libc_value();
				header.hop_limits = hop_limits;
				//header.src_addr = unsafe { zeroed() };
				//header.dst_addr= unsafe { zeroed() };

				const TrailingAddressBytesLength: usize = InternetProtocolVersion6HostAddress::Size * 2;

				(size_of::<ipv6_hdr>(), TrailingAddressBytesLength)
			},
		}
	}
}
