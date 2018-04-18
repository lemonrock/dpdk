// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_export]
macro_rules! discardPacketIf
{
	($packet: ident, $unlikely: expr) =>
	{
		{
			use ::rust_extra::unlikely;
			
			if unlikely($unlikely)
			{
				$packet.free();
				return;
			}
		}
	}
}

#[macro_export]
macro_rules! discardPacketIfNone
{
	($packet: ident, $possiblyNone: expr) =>
	{
		{
			let temp = $possiblyNone;
			discardPacketIf!($packet, temp.is_none());
			temp.unwrap()
		}
	}
}

#[macro_export]
macro_rules! discardPacketIfDropEligibleBitSetOrTagControlInformationIsInvalid
{
	($packet: ident, $tagControlInformation: ident) =>
	{
		{
			use $crate::domain::virtual_lans::VirtualLanIdentifier;
		
			// Formerly CFI, used to drop Token Ring frames received at Ethernet Ports
			const DropEligibleIndicatorBitFlag: u16 = 4096;
			
			discardPacketIf!($packet, $tagControlInformation & DropEligibleIndicatorBitFlag == DropEligibleIndicatorBitFlag);
			
			let result = VirtualLanIdentifier::extract_from_tag_control_information($tagControlInformation);
			discardPacketIf!($packet, result.is_err());
			result.unwrap()
		}
	}
}

#[macro_export]
macro_rules! packetData
{
	($packet: ident, $offset: expr, $T: ty) =>
	{
		#[allow(trivial_casts)]
		{
			use $crate::dpdk_sys::rust_rte_pktmbuf_read;
		
			#[cfg(debug_assertions)] $packet.debug_assert_self_is_not_null();
			
			let mut buffer: $T = unsafe { ::std::mem::uninitialized() };
			let pointerInsidePacketOrOurBuffer = unsafe { rust_rte_pktmbuf_read($packet, $offset, ::std::mem::size_of::<$T>() as u32, &mut buffer as *mut _ as *mut ::libc::c_void) as *const $T };
			::std::mem::forget(buffer);
			discardPacketIf!($packet, pointerInsidePacketOrOurBuffer.is_null());
			pointerInsidePacketOrOurBuffer
		}
	}
}

#[macro_export]
macro_rules! mutatePacketData
{
	($packet: ident, $offset: expr, $T: ty, $pointerInsidePacketOrOurBuffer: ident, $block: block) =>
	{
		#[allow(trivial_casts)]
		{
			use $crate::dpdk_sys::rust_rte_pktmbuf_read;
			use $crate::dpdk_sys::rust_rte_pktmbuf_write;
		
			#[cfg(debug_assertions)] $packet.debug_assert_self_is_not_null();
			
			let mut buffer: $T = unsafe { ::std::mem::uninitialized() };
			let bufferPointer = &mut buffer as *mut $T;
			let length = ::std::mem::size_of::<$T>() as u32;
			let $pointerInsidePacketOrOurBuffer = unsafe { rust_rte_pktmbuf_read($packet, $offset, length, bufferPointer as *mut ::libc::c_void) as *mut $T };
			::std::mem::forget(buffer);
			discardPacketIf!($packet, $pointerInsidePacketOrOurBuffer.is_null());
			
			// Write back if required
			
			let (modifiedBuffer, result) = $block;
			if unlikely(bufferPointer == $pointerInsidePacketOrOurBuffer)
			{
				if likely(modifiedBuffer)
				{
					let result = unsafe { rust_rte_pktmbuf_write($packet, $offset, length, bufferPointer as *const _) };
					debug_assert!(result == 0, "result from rust_rte_pktmbuf_write was '{}' instead of zero", result)
				}
			}
			result
		}
	}
}

#[macro_export]
macro_rules! discardPacketIfInvalidIpV4CheckSum
{
	($packet: ident, $ipV4Header: ident, $ipV4HeaderData: ident) =>
	{
		let ipV4CheckSumFlags = $packet.ol_flags() & PKT_RX_IP_CKSUM_MASK;
		if likely(ipV4CheckSumFlags != PKT_RX_IP_CKSUM_UNKNOWN)
		{
			discardPacketIf!($packet, ipV4CheckSumFlags != PKT_RX_IP_CKSUM_GOOD);
		}
		else
		{
			let headerCheckSumAsSupplied = $ipV4HeaderData.hdr_checksum;
			$ipV4HeaderData.hdr_checksum = 0;
			
			let asCalculated = unsafe { rust_rte_ipv4_cksum($ipV4Header) };
			
			discardPacketIf!($packet, headerCheckSumAsSupplied != asCalculated);
			
			// Leave packet as we found it
			$ipV4HeaderData.hdr_checksum = asCalculated;
		}
	}
}

#[macro_export]
macro_rules! packetEthernetAddress
{
	($packet: ident, $offset: expr) =>
	{
		packetData!($packet, $offset, ether_addr)
	}
}

#[macro_export]
macro_rules! packetU16
{
	($packet: ident, $offset: expr) =>
	{
		{
			let pointerInsidePacketOrOurBuffer = packetData!($packet, $offset, u16);
			u16::from_be(unsafe { *pointerInsidePacketOrOurBuffer })
		}
	}
}

#[macro_export]
macro_rules! destinationEthernetAddress
{
	($packet: ident) =>
	{
		{
			const OffsetOfDestinationEthernetAddress: u32 = 0;
			packetEthernetAddress!($packet, OffsetOfDestinationEthernetAddress)
		}
	}
}

#[macro_export]
macro_rules! sourceEthernetAddress
{
	($packet: ident) =>
	{
		{
			const OffsetOfSourceEthernetAddress: u32 = 6;
			packetEthernetAddress!($packet, OffsetOfSourceEthernetAddress)
		}
	}
}

#[macro_export]
macro_rules! etherType
{
	($packet: ident, $offset: expr) =>
	{
		packetU16!($packet, $offset)
	}
}

#[macro_export]
macro_rules! tagControlValue
{
	($packet: ident, $offset: expr) =>
	{
		packetU16!($packet, $offset)
	}
}
