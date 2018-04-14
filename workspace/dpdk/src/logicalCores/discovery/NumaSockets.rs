// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NumaSockets
{
	pub isANumaMachine: bool,
	pub logical_cores_active: LogicalCoresActive,
	pub activeCpusByNumaSocket: NumaSocketMap<HashSet<LogicalCore>>,
	pub master_logical_core_numa_socket_id: NumaSocketId,
	pub master_logical_core: LogicalCore,
}

impl NumaSockets
{
	pub fn isValidNumaSocket(&self, index: usize) -> bool
	{
		self.activeCpusByNumaSocket.isValidNumaSocket(index)
	}

	pub fn iterateUsefulSocketsIfIsANumaMachine<F>(&self, callback: F)
	where F: FnMut(NumaSocketId) -> ()
	{
		if self.isANumaMachine
		{
			self.activeCpusByNumaSocket.iterateSockets(callback);
		}
	}

	fn allocateSlaveAndMasterLogicalCoresAsBestWeCanWhenThereAreTooManyLogicalCoreUsers(&self, logicalCoreUsers: &mut [&mut LogicalCoreUser])
	{
		let availableCores = self.logical_cores_active.asVec();
		let numberOfAvailableCores = availableCores.len();

		let mut nextCoreIndex = 0;
		for logicalCoreUser in logicalCoreUsers
		{
			if nextCoreIndex == numberOfAvailableCores
			{
				nextCoreIndex = 0;
			}
			let logicalCore = availableCores[nextCoreIndex];
			logicalCoreUser.willMakeUseOfForNonLocalNumaNode(logicalCore);
			nextCoreIndex += 1;
		}
	}

	pub fn allocateSlaveLogicalCores<'a>(&self, logicalCoreUsers: &'a mut [&'a mut LogicalCoreUser])
	{
		assert_ne!(logicalCoreUsers.len(), 0, "logicalCoreUsers can not be empty");

		// Are there more logical core users than available slave cores?
		// Then we need to use a different allocation strategy; no one is going to get their first choice of NUMA socket
		if logicalCoreUsers.len() > (self.logical_cores_active.count() - 1)
		{
			self.allocateSlaveAndMasterLogicalCoresAsBestWeCanWhenThereAreTooManyLogicalCoreUsers(logicalCoreUsers);
			return;
		}

		let mut socketCorePairs = Vec::new();
		self.activeCpusByNumaSocket.iterate(|numa_socket_id, activeCpus|
		{
			for logicalCore in activeCpus
			{
				if !(numa_socket_id == self.master_logical_core_numa_socket_id && logicalCore.isMaster())
				{
					socketCorePairs.push((numa_socket_id, *logicalCore))
				}
			}
		});

		let mut iterateFairlyOverLogicalCoreUsersStartingWithNext = CircularIterator::new(logicalCoreUsers);

		let mut unwanted = Vec::new();
		for (numa_socket_id, logicalCore) in socketCorePairs
		{
			let mut unwantedSocketCorePair = true;

			iterateFairlyOverLogicalCoreUsersStartingWithNext.iter_mut(|logicalCoreUser|
			{
				if logicalCoreUser.willMakeUseOf(numa_socket_id, logicalCore)
				{
					unwantedSocketCorePair = false;
					true
				}
				else
				{
					false
				}
			});

			if unwantedSocketCorePair
			{
				unwanted.push(logicalCore)
			}
		}

		for logicalCore in unwanted
		{
			iterateFairlyOverLogicalCoreUsersStartingWithNext.iter_mut(|logicalCoreUser|
			{
				logicalCoreUser.willMakeUseOfForNonLocalNumaNode(logicalCore)
			});
		}
	}

	pub fn detectNumaSockets(sys_path: &Path, numaNodesData: Option<NumaNodesData>) -> Result<Self, NumaSocketsDiscoveryError>
	{
		let mut activeCpusByNumaSocket: NumaSocketMap<HashSet<LogicalCore>> = NumaSocketMap::new();

		let logical_cores_active = LogicalCore::online(&sys_path)?;

		let isANumaMachine = if numaNodesData.is_none()
		{
			activeCpusByNumaSocket.putOnce(NumaSocketId::SocketZeroAlwaysExists, logical_cores_active.asHashSet());

			false
		}
		else
		{
			let mut shouldNotContainAnyLogicalCoresWhenAllNumaNodesConsidered = logical_cores_active.clone();

			let numaNodesData = numaNodesData.unwrap();
			let usefulNumaNodes = numaNodesData.nodesThatAreOnlineWithACpuAndMemory();
			assert!(usefulNumaNodes.hasAtLeastOneActive(), "Apparently, there are no useful NUMA nodes yet we are running as a program...");
			match usefulNumaNodes.iterateEnabledWithEarlyReturn(|numaNodeIndex|
			{
				// Read cpus - not definitive, as may not be online
				let numa_socket_id = NumaSocketId::fromU32(numaNodeIndex as u32).unwrap();
				let logicalCoresPotentiallyActive = numa_socket_id.cpuList(sys_path)?;
				let activeCpus = logicalCoresPotentiallyActive.intersect(&logical_cores_active);

				if activeCpus.hasAtLeastOneActive()
				{
					activeCpus.iterateEnabledWithEarlyReturn(|cpuIndex|
					{
						if shouldNotContainAnyLogicalCoresWhenAllNumaNodesConsidered.isDisabled(cpuIndex)
						{
							return Err(NumaSocketsDiscoveryError::CpuIsInMoreThanOneNumaNode(cpuIndex))
						}
						shouldNotContainAnyLogicalCoresWhenAllNumaNodesConsidered.disable(cpuIndex);

						Ok(())
					})?;

					activeCpusByNumaSocket.putOnce(numa_socket_id, activeCpus.asHashSet());
				}

				Ok(())
			})
			{
				Ok(()) => (),
				Err(error) => return Err(error),
			}

			if shouldNotContainAnyLogicalCoresWhenAllNumaNodesConsidered.hasAtLeastOneActive()
			{
				return Err(NumaSocketsDiscoveryError::UnassignedCpuIndices(shouldNotContainAnyLogicalCoresWhenAllNumaNodesConsidered))
			}

			true
		};

		Ok(Self::new(isANumaMachine, logical_cores_active, activeCpusByNumaSocket))
	}

	fn master_logical_core(activeCpusByNumaSocket: &NumaSocketMap<HashSet<LogicalCore>>) -> (NumaSocketId, LogicalCore)
	{
		let lowestNumaSocket = activeCpusByNumaSocket.lowestKey().expect("There should always be at least one NUMA socket");
		let logical_cores_activeForLowestNumaSocket = activeCpusByNumaSocket.getOrPanic(lowestNumaSocket);

		let mut lowestLogicalCore = None;
		for nextLogicalCore in logical_cores_activeForLowestNumaSocket.iter()
		{
			lowestLogicalCore = match lowestLogicalCore
			{
				None => Some(*nextLogicalCore),
				Some(ref currentLowestLogicalCore) => Some(if currentLowestLogicalCore > nextLogicalCore
				{
					*nextLogicalCore
				}
				else
				{
					*currentLowestLogicalCore
				})
			}
		}

		(lowestNumaSocket, lowestLogicalCore.expect("There should always be at least one CPU"))
	}

	fn new(isANumaMachine: bool, logical_cores_active: LogicalCoresActive, activeCpusByNumaSocket: NumaSocketMap<HashSet<LogicalCore>>) -> Self
	{
		let (master_logical_core_numa_socket_id, master_logical_core) = Self::master_logical_core(&activeCpusByNumaSocket);

		NumaSockets
		{
			isANumaMachine,
			logical_cores_active,
			activeCpusByNumaSocket,
			master_logical_core_numa_socket_id,
			master_logical_core,
		}
	}
}
