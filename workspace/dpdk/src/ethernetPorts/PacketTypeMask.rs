// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Refer to rte_mbuf.h, lines 228 - 721
bitflags!
{
	pub flags PacketTypeMask: u32
	{
		const Layer2Mask = RTE_PTYPE_L2_MASK,
		const Layer3Mask = RTE_PTYPE_L3_MASK,
		const Layer4Mask = RTE_PTYPE_L4_MASK,
		const TunnelMask = RTE_PTYPE_TUNNEL_MASK,
		const InnerLayer2Mask = RTE_PTYPE_INNER_L2_MASK,
		const InnerLayer3Mask = RTE_PTYPE_INNER_L3_MASK,
		const InnerLayer4Mask = RTE_PTYPE_INNER_L4_MASK,

		const TunnelPacketMask = RTE_PTYPE_TUNNEL_MASK | RTE_PTYPE_INNER_L2_MASK | RTE_PTYPE_INNER_L3_MASK | RTE_PTYPE_INNER_L4_MASK,
		
		const All = RTE_PTYPE_ALL_MASK,
		
		const Layers2To4Mask = RTE_PTYPE_L2_MASK | RTE_PTYPE_L3_MASK | RTE_PTYPE_L4_MASK,
	}
}

impl Default for PacketTypeMask
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl PacketTypeMask
{
	// Duplicated, because rust lacks proper const functions and struggles to reference constants
	pub const Layers2To4MaskBits: u32 = RTE_PTYPE_L2_MASK | RTE_PTYPE_L3_MASK | RTE_PTYPE_L4_MASK;
	
	// Equivalent to DPDK's RTE_ETH_IS_TUNNEL_PKT(ptype)
	#[inline(always)]
	pub fn isTunnelPacket(self) -> bool
	{
		self.bits & PacketTypeMask::TunnelPacketMask.bits != 0
	}
}
