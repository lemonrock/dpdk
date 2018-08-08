// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An abstraction of a DPDK flow item.
pub trait FlowItem
{
	/// DPDK related struct.
	type DpdkType: Sized;
	
	/// DPDK type.
	const Type: rte_flow_item_type;
	
	/// Is this a DPDK 'META' flow item type?
	const IsMeta: bool;
	
	/// DPDK static mask.
	#[inline(always)]
	fn mask() -> NonNull<Self::DpdkType>;
}

pub struct Raw(rte_flow_item_raw, );

impl Default for Raw
{
	fn default() -> Self
	{
		Raw(Self::mask().as_ref())
	}
}

impl FlowItem for Raw
{
	type DpdkType = rte_flow_item_raw;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_RAW;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> NonNull<Self::DpdkType>
	{
		unsafe { NonNull::new_unchecked(rte_flow_item_raw_mask) }
	}
}



/**
 * RTE_FLOW_ITEM_TYPE_RAW
 *
 * Matches a byte string of a given length at a given offset.
 *
 * Offset is either absolute (using the start of the packet) or relative to
 * the end of the previous matched item in the stack, in which case negative
 * values are allowed.
 *
 * If search is enabled, offset is used as the starting point. The search
 * area can be delimited by setting limit to a nonzero value, which is the
 * maximum number of bytes after offset where the pattern may start.
 *
 * Matching a zero-length pattern is allowed, doing so resets the relative
 * offset for subsequent items.
 *
 * This type does not support ranges (struct rte_flow_item.last).
 */
struct rte_flow_item_raw {
	uint32_t relative:1; /**< Look for pattern after the previous item. */
	uint32_t search:1; /**< Search pattern from offset (see also limit). */
	uint32_t reserved:30; /**< Reserved, must be set to zero. */
	int32_t offset; /**< Absolute or relative offset for pattern. */
	uint16_t limit; /**< Search area limit for start of pattern. */
	uint16_t length; /**< Pattern length. */
	const uint8_t *pattern; /**< Byte string to look for. */
};

/** Default mask for RTE_FLOW_ITEM_TYPE_RAW. */
#ifndef __cplusplus
static const struct rte_flow_item_raw rte_flow_item_raw_mask = {
.relative = 1,
.search = 1,
.reserved = 0x3fffffff,
.offset = 0xffffffff,
.limit = 0xffff,
.length = 0xffff,
.pattern = NULL,
};
