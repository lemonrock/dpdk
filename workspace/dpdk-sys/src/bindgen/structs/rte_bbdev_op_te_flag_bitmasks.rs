// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct rte_bbdev_op_te_flag_bitmasks(pub u32);

impl rte_bbdev_op_te_flag_bitmasks
{
	pub const RTE_BBDEV_TURBO_RV_INDEX_BYPASS: rte_bbdev_op_te_flag_bitmasks = rte_bbdev_op_te_flag_bitmasks(1);
}

impl rte_bbdev_op_te_flag_bitmasks
{
	pub const RTE_BBDEV_TURBO_RATE_MATCH: rte_bbdev_op_te_flag_bitmasks = rte_bbdev_op_te_flag_bitmasks(2);
}

impl rte_bbdev_op_te_flag_bitmasks
{
	pub const RTE_BBDEV_TURBO_CRC_24B_ATTACH: rte_bbdev_op_te_flag_bitmasks = rte_bbdev_op_te_flag_bitmasks(4);
}

impl rte_bbdev_op_te_flag_bitmasks
{
	pub const RTE_BBDEV_TURBO_CRC_24A_ATTACH: rte_bbdev_op_te_flag_bitmasks = rte_bbdev_op_te_flag_bitmasks(8);
}

impl rte_bbdev_op_te_flag_bitmasks
{
	pub const RTE_BBDEV_TURBO_ENC_INTERRUPTS: rte_bbdev_op_te_flag_bitmasks = rte_bbdev_op_te_flag_bitmasks(16);
}

impl rte_bbdev_op_te_flag_bitmasks
{
	pub const RTE_BBDEV_TURBO_ENC_SCATTER_GATHER: rte_bbdev_op_te_flag_bitmasks = rte_bbdev_op_te_flag_bitmasks(32);
}

impl BitOr<rte_bbdev_op_te_flag_bitmasks> for rte_bbdev_op_te_flag_bitmasks
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		rte_bbdev_op_te_flag_bitmasks(self.0 | other.0)
	}
}

impl BitOrAssign for rte_bbdev_op_te_flag_bitmasks
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: rte_bbdev_op_te_flag_bitmasks)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<rte_bbdev_op_te_flag_bitmasks> for rte_bbdev_op_te_flag_bitmasks
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		rte_bbdev_op_te_flag_bitmasks(self.0 & other.0)
	}
}

impl BitAndAssign for rte_bbdev_op_te_flag_bitmasks
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: rte_bbdev_op_te_flag_bitmasks)
	{
		self.0 &= rhs.0;
	}
}
