// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct IpV6State
{
	pub sourceIpV6AddressBlackList: IpV6AddressBlackList,
	pub ourIpV6Addresses: HashMap<InternetProtocolVersion6HostAddress, IpAddressInformation>,
}

impl IpV6State
{
	#[inline(always)]
	pub fn ipV6Packet(&mut self, packet: *mut rte_mbuf, layer2HeaderLength: u32)
	{
		macro_rules! nextHeaderAndThisHeaderLength
		{
			($packet: ident, $offset: expr, $layer3Length: ident) =>
			{
				{
					let nextHeaderAndThisHeaderLengthPointer = packetData!($packet, $offset, [u8; 2]);
					let nextHeader = unsafe {*nextHeaderAndThisHeaderLengthPointer};
					let thisHeaderLength = unsafe {*nextHeaderAndThisHeaderLengthPointer.offsetUp(1)} as u32;
					
					discardPacketIf!($packet, $offset + thisHeaderLength > $layer3Length);
					
					(nextHeader, thisHeaderLength)
				}
			}
		}
		
		macro_rules! fragmentNextHeaderAndThisHeaderLength
		{
			($packet: ident, $offset: expr, $layer3Length: ident) =>
			{
				{
					let nextHeaderAndThisHeaderLengthPointer = packetData!($packet, $offset, [u8; 2]);
					let nextHeader = unsafe {*nextHeaderAndThisHeaderLengthPointer};
					let reserved = unsafe {*nextHeaderAndThisHeaderLengthPointer.offsetUp(1)};
					
					discardPacketIf!(packet, reserved != 0); // We are supposed to ignore reserved != 0.
					
					let thisHeaderLength = size_of::<ipv6_extension_fragment>() as u32;
					discardPacketIf!($packet, $offset + thisHeaderLength > $layer3Length);
					
					(nextHeader, thisHeaderLength)
				}
			}
		}
		
		macro_rules! handleTcp
		{
			($packet: ident, $layer2HeaderLength: ident, $layer3Length: ident, $remainingPayloadLength: ident, $thisHeaderLength: ident)
			{
				
				{
					let layer4Length = $remainingPayloadLength - $thisHeaderLength;
					let layer3HeaderLength = layer3Length - layer4Length;
					
					// Drop if Length exceeds 255 bytes (the maximum supported by DPDK)
					discardPacketIf!(packet, layer3HeaderLength > Self::DpdkMaximumLayer3HeaderLength);
			
					let (layer4Protocol, layer4HeaderLength) = fixUpTcpForTldk!(packet, layer2HeaderLength, layer3HeaderLength, RTE_PTYPE_L3_IPV6);
					(layer3HeaderLength, layer4Protocol, layer4HeaderLength)
				}
			}
		}
		
		macro_rules! handleUdp
		{
			($packet: ident, $layer2HeaderLength: ident, $layer3Length: ident, $remainingPayloadLength: ident, $thisHeaderLength: ident)
			{
				{
					let layer4Length = $remainingPayloadLength - $thisHeaderLength;
					let layer3HeaderLength = layer3Length - layer4Length;
					
					// Drop if Length exceeds 255 bytes (the maximum supported by DPDK)
					discardPacketIf!(packet, layer3HeaderLength > Self::DpdkMaximumLayer3HeaderLength);
					
					let (layer4Protocol, layer4HeaderLength) = fixUpUdpForTldk!(packet, layer2HeaderLength, layer3HeaderLength, RTE_PTYPE_L3_IPV6);
					(layer3HeaderLength, layer4Protocol, layer4HeaderLength)
				}
			}
		}
		
		macro_rules! handleIcmpV6
		{
			($packet: ident, $layer2HeaderLength: ident, $layer3Length: ident, $remainingPayloadLength: ident, $thisHeaderLength: ident)
			{
				{
					$packet.free();
					return;
				}
			}
		}
		
		macro_rules! hopByHopOptionsExtensionHeader
		{
			($packet: ident, $remainingPayloadLength: ident, $previousExtensionHeaderOffset: ident, $previousHeaderLength: ident, $layer2HeaderLength: ident, $layer3Length: ident, $isFragmented: ident) =>
			{
				{
					let (remainingPayloadLength, extensionHeaderOffset) = descendExtensionHeader($previousExtensionHeaderOffset, $extensionHeaderOffset, $previousHeaderLength);
					let (nextHeader, thisHeaderLength) = fragmentNextHeaderAndThisHeaderLength!(packet, extensionHeaderOffset);
					
					match nextHeader
					{
						TcpNextHeader => handleTcp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						UdpNextHeader => handleUdp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						IcmpV6NextHeader => handleIcmpV6!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
				
						DestinationOptionsNextHeader => firstDestinationOptionsOrFinalDestinationOptionsExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length),
				
						RoutingNextHeader => routingExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length, $isFragmented),
						
						FragmentNextHeader => fragmentExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length, $isFragmented),
						
						_ => discardPacket!($packet),
					}
				}
			}
		}
		
		macro_rules! firstDestinationOptionsOrFinalDestinationOptionsExtensionHeader
		{
			($packet: ident, $remainingPayloadLength: ident, $previousExtensionHeaderOffset: ident, $previousHeaderLength: ident, $layer2HeaderLength: ident, $layer3Length: ident, $isFragmented: ident) =>
			{
				{
					let (remainingPayloadLength, extensionHeaderOffset) = descendExtensionHeader($previousExtensionHeaderOffset, $extensionHeaderOffset, $previousHeaderLength);
					let (nextHeader, thisHeaderLength) = fragmentNextHeaderAndThisHeaderLength!(packet, extensionHeaderOffset);
					
					match nextHeader
					{
						TcpNextHeader => handleTcp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						UdpNextHeader => handleUdp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						IcmpV6NextHeader => handleIcmpV6!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
				
						RoutingNextHeader => routingExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length, $isFragmented),
						
						FragmentNextHeader => fragmentExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length, $isFragmented),
						
						DestinationOptionsNextHeader => finalDestinationOptionsExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length),
						
						_ => discardPacket!($packet),
					}
				}
			}
		}
		
		macro_rules! routingExtensionHeader
		{
			($packet: ident, $remainingPayloadLength: ident, $previousExtensionHeaderOffset: ident, $previousHeaderLength: ident, $layer2HeaderLength: ident, $layer3Length: ident, $isFragmented: ident) =>
			{
				{
					let (remainingPayloadLength, extensionHeaderOffset) = descendExtensionHeader($previousExtensionHeaderOffset, $extensionHeaderOffset, $previousHeaderLength);
					let (nextHeader, thisHeaderLength) = nextHeaderAndThisHeaderLength!(packet, extensionHeaderOffset);
					
					match nextHeader
					{
						TcpNextHeader => handleTcp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						UdpNextHeader => handleUdp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						IcmpV6NextHeader => handleIcmpV6!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						FragmentNextHeader => fragmentExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length, $isFragmented),
						
						DestinationOptionsNextHeader => finalDestinationOptionsExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, $layer2Length, $thisHeaderLength, $layer2Length, $layer3Length),
						
						_ => discardPacket!($packet),
					}
				}
			}
		}
		
		macro_rules! fragmentExtensionHeader
		{
			($packet: ident, $remainingPayloadLength: ident, $previousExtensionHeaderOffset: ident, $previousHeaderLength: ident, $layer2HeaderLength: ident, $layer3Length: ident, $isFragmented: ident) =>
			{
				{
					$isFragmented = true;
					
					let (remainingPayloadLength, extensionHeaderOffset) = descendExtensionHeader($previousExtensionHeaderOffset, $extensionHeaderOffset, $previousHeaderLength);
					let (nextHeader, thisHeaderLength) = fragmentNextHeaderAndThisHeaderLength!(packet, extensionHeaderOffset);
					
					match nextHeader
					{
						TcpNextHeader => handleTcp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						UdpNextHeader => handleUdp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						IcmpV6NextHeader => handleIcmpV6!($packet, remainingPayloadLength, thisHeaderLength),
						
						DestinationOptionsNextHeader => finalDestinationOptionsExtensionHeader!($packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, $layer2Length, $layer3Length),
						
						_ => discardPacket!($packet),
					}
				}
			}
		}
		
		macro_rules! finalDestinationOptionsExtensionHeader
		{
			($packet: ident, $remainingPayloadLength: ident, $previousExtensionHeaderOffset: ident, $previousHeaderLength: ident, $layer2HeaderLenth: ident, $layer3Length: ident) =>
			{
				{
					let (remainingPayloadLength, extensionHeaderOffset) = descendExtensionHeader($previousExtensionHeaderOffset, $extensionHeaderOffset, $previousHeaderLength);
					let (nextHeader, thisHeaderLength) = nextHeaderAndThisHeaderLength!($packet, extensionHeaderOffset, $layer3Length);
					
					match nextHeader
					{
						TcpNextHeader => handleTcp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						UdpNextHeader => handleUdp!($packet, $layer2HeaderLength, $layer3Length, remainingPayloadLength, thisHeaderLength),
						
						IcmpV6NextHeader => handleIcmpV6!($packet, remainingPayloadLength, thisHeaderLength),
						
						_ => discardPacket!($packet),
					}
				}
			}
		}
		
		#[inline(always)]
		fn descendExtensionHeader(remainingPayloadLength: u32, extensionHeaderOffset: u32, thisHeaderLength: u32) -> (u32, u32)
		{
			let remainingPayloadLength = remainingPayloadLength - thisHeaderLength;
			let extensionHeaderOffset = extensionHeaderOffset + thisHeaderLength;
			(remainingPayloadLength, extensionHeaderOffset)
		}
		
		let destinationEthernetAddress = destinationEthernetAddress!(packet);
		discardPacketIf!(packet, MediaAccessControlAddress::ethernetAddressIsInvalid(destinationEthernetAddress));
		
		let (layer3Length, layer3HeaderLength, layer4Protocol, layer4HeaderLength) =
		{
			let ipV6Header = packetData!(packet, layer2HeaderLength, ipv6_hdr);
			
			let ipV6HeaderData = unsafe { *ipV6Header };
			
			let sizeOfIpv6Header = size_of::<ipv6_hdr>() as u32;
			let payloadLength = u16::from_be(ipV6HeaderData.payload_len) as u32;
			let layer3Length = sizeOfIpv6Header + payloadLength;
			
			let nextHeader = ipV6HeaderData.proto;
			
			// There are a number of possibilities for next header
			// We follow the IPv6 header ordering recommendations in RFC 2460 as if they were a requirement
			const HopByHopOptions: u8 = 0;
			const TcpNextHeader: u8 = 6;
			const UdpNextHeader: u8 = 17;
			const IcmpV6NextHeader: u8 = 58;
			const RoutingNextHeader: u8 = 43;
			const FragmentNextHeader: u8 = 44;
			const DestinationOptionsNextHeader: u8 = 60;
			
			let remainingPayloadLength = payloadLength;
			let extensionHeaderOffset = layer2HeaderLength + sizeOfIpv6Header;
			let thisHeaderLength = 0;
			
			let mut isFragmented = false;
			
			let (layer3HeaderLength, layer4Protocol, layer4HeaderLength) = match nextHeader
			{
				TcpNextHeader => handleTcp!(packet, layer2HeaderLength, layer3Length, remainingPayloadLength, thisHeaderLength),
				
				UdpNextHeader => handleUdp!(packet, layer2HeaderLength, layer3Length, remainingPayloadLength, thisHeaderLength),
				
				IcmpV6NextHeader => handleIcmpV6!(packet, layer2HeaderLength, layer3Length, remainingPayloadLength, thisHeaderLength),
				
				HopByHopOptions => hopByHopOptionsExtensionHeader!(packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, layer3Length, isFragmented),
				
				DestinationOptionsNextHeader => firstDestinationOptionsOrFinalDestinationOptionsExtensionHeader!(packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, layer3Length, isFragmented),
				
				RoutingNextHeader => routingExtensionHeader!(packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, layer3Length, isFragmented),
				
				FragmentNextHeader => fragmentExtensionHeader!(packet, remainingPayloadLength, extensionHeaderOffset, thisHeaderLength, layer3Length, isFragmented),
				
				_ => discardPacket!(packet),
			};
			
			let senderIpV6Address = ipV6HeaderData.src_addr;
			discardPacketIf!(packet, senderIpV6Address.is_not_valid_unicast());
			discardPacketIf!(packet, self.isBlackListedSourceIpV6Address(&senderIpV6Address));
			discardPacketIf!(packet, self.isOneOfOurIpV6Addresses(&senderIpV6Address));
			
			let destinationIpV6Address = ipV6HeaderData.dst_addr;
			discardPacketIf!(packet, self.isNotOneOfOurIpV6Addresses(&destinationIpV6Address, destinationEthernetAddress));
			
			if isFragmented
			{
				// this is a fragment that needs processing
			}
			
			(layer3Length, layer3HeaderLength, layer4Protocol, layer4HeaderLength)
		};
		
		IpState::prepareToSendIpPacketToTldk(packet, layer2HeaderLength, layer3Length, layer3HeaderLength, layer4HeaderLength);
	}
	
	#[inline(always)]
	fn isBlackListedSourceIpV6Address(&self, senderIpV6Address: &InternetProtocolVersion6HostAddress) -> bool
	{
		self.sourceIpV6AddressBlackList.isIpAddressBlackListed(&senderIpV6Address)
	}
	
	#[inline(always)]
	fn isOneOfOurIpV6Addresses(&self, senderIpV6Address: &InternetProtocolVersion6HostAddress) -> bool
	{
		self.ourIpV6Addresses.get(senderIpV6Address).is_some()
	}
	
	#[inline(always)]
	fn isNotOneOfOurIpV6Addresses(&self, destinationIpV6Address: &InternetProtocolVersion6HostAddress, destinationEthernetAddress: *const ether_addr) -> bool
	{
		match self.ourIpV6Addresses.get(destinationIpV6Address)
		{
			None => false,
			Some(rcRefCellIpAddressInformation) => rcRefCellIpAddressInformation.borrow().ourEthernetAddress == unsafe { *destinationEthernetAddress }
		}
	}
}
