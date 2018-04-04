// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct ExecutionRoutineGroup<S: SlaveLogicalCoreTask>
{
	canContinue: CanContinue,
	currentQueuePairCount: usize,
	unnaturalDeaths: usize,
	naturalDeaths: usize,
	tasks: ArrayVec<[S; MaximumQueuePairs]>,
	deaths: ArrayVec<[Option<Result<(), ()>>; MaximumQueuePairs]>,
}

impl<S: SlaveLogicalCoreTask> ExecutionRoutineGroup<S>
{
	pub fn new(originalQueuePairCount: usize) -> Arc<Mutex<ExecutionRoutineGroup<S>>>
	{
		Arc::new
		(
			Mutex::new
			(
				ExecutionRoutineGroup
				{
					canContinue: CanContinue::newCanContinue(),
					currentQueuePairCount: originalQueuePairCount,
					unnaturalDeaths: 0,
					naturalDeaths: 0,
					tasks: ArrayVec::new(),
					deaths: ArrayVec::new(),
				}
			)
		)
	}
	
	#[inline(always)]
	pub fn makeStop(&mut self)
	{
		self.canContinue.makeStop()
	}
	
	pub fn canContinueClone(&self) -> CanContinue
	{
		self.canContinue.clone()
	}
	
	pub fn pushAndRunOnSlave(&mut self, task: S)
	{
		self.tasks.push(task);
		self.deaths.push(None);
		self.tasks.last_mut().unwrap().runOnSlave().expect("Could not start to run on slave");
	}
	
	pub fn notifyOfDeath(&mut self, queuePairIdentifier: QueueIdentifier, causeOfDeath: Result<(), ()>)
	{
		if let Some(index) = self.deaths.get_mut(queuePairIdentifier as usize)
		{
			self.currentQueuePairCount -= 1;
			if causeOfDeath.is_ok()
			{
				self.naturalDeaths += 1;
			}
			else
			{
				self.unnaturalDeaths += 1;
			}
			*index = Some(causeOfDeath);
		}
		else
		{
			panic!("No queue pair '{}'", queuePairIdentifier);
		}
	}
	
	pub fn hasUnnaturalDeaths(&mut self) -> bool
	{
		self.unnaturalDeaths != 0
	}
}
