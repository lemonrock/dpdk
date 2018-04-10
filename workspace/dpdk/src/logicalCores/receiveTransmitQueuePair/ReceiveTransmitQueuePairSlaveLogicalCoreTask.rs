// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
pub struct ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC: ExecutionRoutineCreator>
{
	canContinue: CanContinue,
	executionRoutineGroup: Arc<Mutex<ExecutionRoutineGroup<ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>>>>,
	executionRoutineCreator: EC,
	slaveLogicalCoreToExecuteOn: LogicalCore,
	
	underlyingEthernetDevice: rte_eth_dev,
	
	receiveQueueStopFunction: eth_queue_stop_t,
	receiveQueueIdentifier: QueueIdentifier,
	
	transmitQueueStopFunction: eth_queue_stop_t,
	transmitQueueIdentifier: QueueIdentifier,
}

impl<EC: ExecutionRoutineCreator> SlaveLogicalCoreTask for ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>
{
	#[inline(always)]
	fn slaveLogicalCoreToExecuteOn(&self) -> LogicalCore
	{
		self.slaveLogicalCoreToExecuteOn
	}
}

impl<EC: ExecutionRoutineCreator> MutableCallback1<i32> for ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>
{
	#[inline(always)]
	fn call(&mut self) -> i32
	{
		let result = self.callUnwindSafe();
		
		let (valueResult, causeOfDeath) = match result
		{
			Ok(_) => (0, Ok(())),
			Err(boxedAnyCause) =>
			{
				error!("Logical Core '{:?}' ReceiveTransmitQueuePairSlaveLogicalCoreTask '{}' panic'd with '{:?}'", self.slaveLogicalCoreToExecuteOn, self.receiveQueueIdentifier, boxedAnyCause);
				(-1, Err(()))
			}
		};
		
		{
			let mut guard = match self.executionRoutineGroup.lock()
			{
				Ok(guard) => guard,
				Err(poisoned) => poisoned.into_inner(),
			};
			
			guard.notifyOfDeath(self.queueIdentifier(), causeOfDeath);
		}
		
		valueResult
	}
}

impl<EC: ExecutionRoutineCreator> ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>
{
	fn callUnwindSafe(&mut self) -> thread::Result<()>
	{
		catch_unwind(AssertUnwindSafe(move ||
		{
			debug_assert!(LogicalCore::isCurrentSlave(), "Can not call call() on a master logical core");
			debug_assert!(self.isCurrentCorrectLogicalCoreToExecuteOn(), "Can not call call() on a different slave logical core");
			
			#[cfg(any(target_os = "android", target_os = "linux"))] set_current_thread_name(&format!("Slave-{}", self.slaveLogicalCoreToExecuteOn.as_u32())).expect("Could not set thread name");
			
			let mut executionRoutine = self.executionRoutineCreator.createExecutionRoutineWhilstExecutingOnSlaveLogicalCore();
			
			executionRoutine.start();
			
			while likely(self.canContinue())
			{
				if unlikely(executionRoutine.execute())
				{
					break;
				}
			}
			
			executionRoutine.stop();
			
			self.stopOurReceiveQueueAndOurTransmitQueue();
		}))
	}
	
	pub fn new<Creator: ExecutionRoutineCreatorCreator<D, EC>, D>
	(
		canContinue: CanContinue,
		executionRoutineGroup: Arc<Mutex<ExecutionRoutineGroup<ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>>>>,
		executionRoutineCreatorCreator: &Creator,
		data: Arc<D>,
		queuePairIdentifier: QueueIdentifier,
		slaveLogicalCoreToExecuteOn: LogicalCore,
		ethernetPortInformation: &EthernetPortInformation,
	) -> ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>
	{
		debug_assert!(LogicalCore::isCurrentMaster(), "Can not call new() on a slave logical core");
		
		let underlyingEthernetDevice = ethernetPortInformation.underlyingEthernetDevice();
		let deviceOperations = unsafe { *(underlyingEthernetDevice.dev_ops) };
		
		ReceiveTransmitQueuePairSlaveLogicalCoreTask
		{
			canContinue: canContinue,
			executionRoutineGroup: executionRoutineGroup,
			executionRoutineCreator: executionRoutineCreatorCreator.createWhilstOnMasterLogicalCore(data, queuePairIdentifier, slaveLogicalCoreToExecuteOn, ethernetPortInformation),
			slaveLogicalCoreToExecuteOn: slaveLogicalCoreToExecuteOn,
		
			underlyingEthernetDevice: underlyingEthernetDevice.clone(),
			
			receiveQueueStopFunction: deviceOperations.rx_queue_stop,
			receiveQueueIdentifier: queuePairIdentifier,
			
			transmitQueueStopFunction: deviceOperations.tx_queue_stop,
			transmitQueueIdentifier: queuePairIdentifier,
		}
	}
	
	#[inline(always)]
	fn queueIdentifier(&self) -> QueueIdentifier
	{
		self.receiveQueueIdentifier
	}
	
	#[inline(always)]
	fn canContinue(&mut self) -> bool
	{
		debug_assert!(self.isCurrentCorrectLogicalCoreToExecuteOn(), "Can not call canContinue() on a different slave logical core");
		
		self.canContinue.canContinue()
	}
	
	#[inline(always)]
	fn stopOurReceiveQueueAndOurTransmitQueue(&mut self)
	{
		let underlyingEthernetDevice = &mut self.underlyingEthernetDevice;
		
		if let Some(receiveQueueStopFunction) = self.receiveQueueStopFunction
		{
			unsafe { receiveQueueStopFunction(underlyingEthernetDevice, self.receiveQueueIdentifier) };
		}
		
		if let Some(transmitQueueStopFunction) = self.transmitQueueStopFunction
		{
			unsafe { transmitQueueStopFunction(underlyingEthernetDevice, self.transmitQueueIdentifier) };
		}
	}
}
