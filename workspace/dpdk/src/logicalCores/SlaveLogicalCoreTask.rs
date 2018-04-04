// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait SlaveLogicalCoreTask : MutableCallback1<i32>
{
	/// Field accessors, yuck...
	#[inline(always)]
	fn slaveLogicalCoreToExecuteOn(&self) -> LogicalCore;
	
	/// Should only be called from master logical core
	#[inline(always)]
	fn runOnSlave(&mut self) -> Result<(), ()>
	{
		debug_assert!(LogicalCore::isCurrentMaster(), "Can not call runOnSlave() on a slave logical core");
		
		self.slaveLogicalCoreToExecuteOn().runOnSlave(self)
	}
	
	#[inline(always)]
	fn isCurrentCorrectLogicalCoreToExecuteOn(&self) -> bool
	{
		self.slaveLogicalCoreToExecuteOn().isCurrent()
	}
}
