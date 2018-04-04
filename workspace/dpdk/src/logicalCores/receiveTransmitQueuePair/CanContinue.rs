// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct CanContinue(Arc<AtomicBool>);

impl CanContinue
{
	#[inline(always)]
	fn newCanContinue() -> Self
	{
		CanContinue(Arc::new(AtomicBool::new(true)))
	}
	
	#[inline(always)]
	fn canContinue(&self) -> bool
	{
		debug_assert!(LogicalCore::isCurrentSlave(), "Can not call canContinue() on a master logical core");
		
		self.0.load(Ordering::SeqCst)
	}
	
	#[inline(always)]
	fn makeStop(&mut self)
	{
		debug_assert!(LogicalCore::isCurrentMaster(), "Can not call makeStop() on a slave logical core");
		
		self.0.store(false, Ordering::SeqCst)
	}
}
