// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
pub struct Networking
{
	pub ethernetPortInformation: EthernetPortInformation,
	pub ethernetPortConfigurationResult: EthernetPortConfigurationResult,
	executionRoutineGroup: Arc<Mutex<ExecutionRoutineGroup<ReceiveTransmitQueuePairSlaveLogicalCoreTask<OurExecutionRoutineCreator>>>>,
}

impl Networking
{
	pub fn makeStop(&self)
	{
		let mut guard = match self.executionRoutineGroup.lock()
		{
			Ok(guard) => guard,
			Err(poisoned) => poisoned.into_inner(),
		};

		guard.makeStop();
	}

	//noinspection SpellCheckingInspection
	pub fn startAllNetworking
	(
		nonEthernetPortLogicalCoreUsers: &mut [&mut LogicalCoreUser],
		numa_sockets: &NumaSockets,
		ethernetPortConfigurations: &mut EthernetPortConfigurations,
	) -> Vec<Networking>
	{
		let (allEthernetPortsExcludingBondedSlaves, _) = EthernetPort::allEthernetPortsExcludingBondedSlavesAndBondedSlaves();

		let mut ethernetPortInformationVec: Vec<EthernetPortInformation> = allEthernetPortsExcludingBondedSlaves.iter().map(|ethernetPort| ethernetPort.information()).collect();

		Self::allocateSlaveLogicalCoresToUsersFairly(numa_sockets, nonEthernetPortLogicalCoreUsers, &mut ethernetPortInformationVec);

		let mut networkings = Vec::with_capacity(allEthernetPortsExcludingBondedSlaves.len());

		for mut ethernetPortInformation in ethernetPortInformationVec.drain(..)
		{
			let (ethernetPortConfigurationResult, executionRoutineGroup) = ethernetPortConfigurations.configureAndStartEthernetPort(&mut ethernetPortInformation);

			networkings.push(Networking
			{
				ethernetPortInformation,
				ethernetPortConfigurationResult,
				executionRoutineGroup,
			})
		}

		networkings.sort_by_key(|value| value.ethernetPortInformation.portIdentifier());
		networkings
	}

	fn allocateSlaveLogicalCoresToUsersFairly(numa_sockets: &NumaSockets, nonEthernetPortLogicalCoreUsers: &mut [&mut LogicalCoreUser], mut ethernetPortInformationSlice: &mut [EthernetPortInformation])
	{
		let mut logicalCoreUsers: Vec<&mut LogicalCoreUser> = Vec::with_capacity(nonEthernetPortLogicalCoreUsers.len() + ethernetPortInformationSlice.len());

		for mut nonEthernetPortLogicalCoreUser in nonEthernetPortLogicalCoreUsers.iter_mut()
		{
			logicalCoreUsers.push(nonEthernetPortLogicalCoreUser);
		}

		for mut ethernetPortInformation in ethernetPortInformationSlice.iter_mut()
		{
			let logicalCoreUser = ethernetPortInformation.useLogicalCoreUser();
			logicalCoreUsers.push(logicalCoreUser);
		}

		numa_sockets.allocateSlaveLogicalCores(logicalCoreUsers.as_mut_slice());
	}
}
