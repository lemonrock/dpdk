// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// bindgen bindings seem to have omitted this
const LCORE_ID_ANY: c_uint = ::std::u32::MAX;

pub const MaximumLogicalCores: usize = RTE_MAX_LCORE;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct LogicalCore(u32);

impl LogicalCore
{
	#[inline(always)]
	pub fn count() -> u32
	{
		unsafe { *rte_eal_get_configuration() }.lcore_count
	}
}

impl Default for LogicalCore
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Zero
	}
}

impl LogicalCore
{
	pub const Zero: LogicalCore = LogicalCore(0);

	// In a thread not under the control of the Environment Abstraction Layer, current() will return Any
	pub const Any: LogicalCore = LogicalCore(LCORE_ID_ANY);

	pub fn online(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "online")
	}

	/// Not useful, as includes cpus that can never be brought online (see possible)
	pub fn offline(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "offline")
	}

	/// Not reliable, as includes cpus that can never be brought online; simply reports CPUs that could be used by the Kernel upto the CONFIG_? number of CPUs
	pub fn possible(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "possible")
	}

	/// Not reliable, as includes cpus that can never be brought online; simply reports CPUs that could be used by the Kernel upto the CONFIG_? number of CPUs
	pub fn present(sys_path: &Path) -> Result<LogicalCoresActive, ListParseError>
	{
		Self::cpulist(sys_path, "present")
	}

	pub fn kernelMaximumCpuIndex(sys_path: &Path) -> io::Result<u32>
	{
		let file_path = Self::cpusItemSysPath(sys_path, "kernel_max");
		file_path.read_value()
	}

	fn cpulist(sys_path: &Path, fileName: &str) -> Result<LogicalCoresActive, ListParseError>
	{
		let file_path = Self::cpusItemSysPath(sys_path, fileName);
		LogicalCoresActive::parse_from_file_path(&file_path)
	}

	pub fn topologyCoreId(&self, sys_path: &Path) -> io::Result<u64>
	{
		let file_path = self.topologyFilePath(sys_path, "core_id");
		file_path.read_value()
	}

	#[inline(always)]
	fn cpusSysPath(sys_path: &Path) -> PathBuf
	{
		let mut nodesSysPath = PathBuf::from(sys_path);
		nodesSysPath.push("devices/system/cpu");
		nodesSysPath
	}

	#[inline(always)]
	fn cpusItemSysPath(sys_path: &Path, item: &str) -> PathBuf
	{
		let mut nodesItemSysPath = Self::cpusSysPath(sys_path);
		nodesItemSysPath.push(item);
		nodesItemSysPath
	}

	#[inline(always)]
	fn cpuSysPath(&self, sys_path: &Path) -> PathBuf
	{
		if self.isAny()
		{
			panic!("Any logical core does not have a cpuSysPath");
		}

		Self::cpusItemSysPath(sys_path, &format!("cpu{}", self.0))
	}

	#[inline(always)]
	fn topologyFilePath(&self, sys_path: &Path, fileName: &str) -> PathBuf
	{
		let mut path = self.cpuSysPath(sys_path);
		path.push("topology");
		path.push(fileName);
		path
	}

	pub fn allLogicalCores(excludeMasterIeOnlySlaves: bool) -> Vec<LogicalCore>
	{
		// If the master is included, then all cores are valid, and so a valid core can never be of type Any
		let excludeIfLogicalCoreIdentifierMatches = match excludeMasterIeOnlySlaves
		{
			true => Self::getMaster().0,
			false => Self::Any.0,
		};

		let mut list: Vec<LogicalCore> = Vec::with_capacity(Self::count() as usize);

		let mut logicalCoreIdentifier = 0;
		while logicalCoreIdentifier < MaximumLogicalCores as u32
		{
			let logicalPort = LogicalCore(logicalCoreIdentifier);
			if likely(logicalCoreIdentifier != excludeIfLogicalCoreIdentifierMatches)
			{
				if logicalPort.isEnabled()
				{
					list.push(logicalPort);
				}
			}

			logicalCoreIdentifier += 1;
		}

		list
	}

	#[inline(always)]
	pub fn as_u32(&self) -> u32
	{
		self.0
	}

	#[inline(always)]
	pub fn isMaster(&self) -> bool
	{
		*self == Self::getMaster()
	}

	#[inline(always)]
	pub fn isSlave(&self) -> bool
	{
		*self != Self::getMaster()
	}

	#[inline(always)]
	pub fn isAny(&self) -> bool
	{
		*self != Self::Any
	}

	#[inline(always)]
	pub fn current() -> LogicalCore
	{
		LogicalCore(unsafe { rust_rte_lcore_id() })
	}

	#[inline(always)]
	pub fn isCurrent(&self) -> bool
	{
		(unsafe { rust_rte_lcore_id() }) == self.0
	}

	#[inline(always)]
	pub fn isCurrentMaster() -> bool
	{
		Self::current() == Self::getMaster()
	}

	#[inline(always)]
	pub fn isCurrentSlave() -> bool
	{
		Self::current() != Self::getMaster()
	}

	#[inline(always)]
	pub fn isCurrentAny() -> bool
	{
		Self::current() != Self::Any
	}

	#[inline(always)]
	pub fn optionalNumaSocketId(&self) -> Option<NumaSocketId>
	{
		NumaSocketId::fromU32(unsafe { lcore_config[self.0 as usize] }.socket_id)
	}

	#[inline(always)]
	pub fn isEnabled(&self) -> bool
	{
		if self.0 as usize >= MaximumLogicalCores
		{
			false
		}
		else
		{
			unsafe { *rte_eal_get_configuration() }.lcore_role[self.0 as usize] != rte_lcore_role_t::ROLE_OFF
		}
	}

	#[inline(always)]
	pub fn role(&self) -> rte_lcore_role_t
	{
		unsafe { rte_eal_lcore_role(self.0) }
	}

	#[inline(always)]
	pub fn getMaster() -> LogicalCore
	{
		LogicalCore(unsafe { *rte_eal_get_configuration() }.master_lcore )
	}
}

// All of these functions and methods are to be executed by the code RUNNING on the MASTER Logical Core.
impl LogicalCore
{
	#[inline(always)]
	pub fn waitForAllSlaveCoresToEnterWaitState()
	{
		debug_assert!(Self::isCurrentMaster(), "Can not call this waitForAllSlaveCoresToEnterWaitState() on a slave core");

		unsafe { rte_eal_mp_wait_lcore() };
	}

	/// WARNING: If the callback goes out of scope and is dropped, then the pointers passed to C will be come invalid. Be careful!
	/// Err if any SLAVE not in WAIT state (ie BUSY)
	#[inline(always)]
	pub fn runOnAllSlaves<C: Callback1<i32>>(callback: &C, alsoRunOnMaster: bool) -> Result<(), ()>
	{
		debug_assert!(Self::isCurrentMaster(), "Can not call this runOnAllSlaves() on a slave core");

		let callMaster = match alsoRunOnMaster
		{
			true => rte_rmt_call_master_t::CALL_MASTER,
			false => rte_rmt_call_master_t::SKIP_MASTER,
		};

		let result = unsafe { rte_eal_mp_remote_launch(C::asFunctionPointer(), callback.asFunctionArgument(), callMaster) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::EBUSY => Err(()),

				_ => panic!("Unexptected error '{}' from rte_eal_mp_remote_launch()", result),
			}
		}
	}

	#[inline(always)]
	pub fn getState(&self) -> rte_lcore_state_t
	{
		debug_assert!(Self::isCurrentMaster(), "Can not call this getState() on a slave core");

		unsafe { rte_eal_get_lcore_state(self.0) }
	}

	// Result is value of process running on slave
	#[inline(always)]
	pub fn waitForThisSlaveToEnterWaitState(&self) -> i32
	{
		debug_assert!(Self::isCurrentMaster(), "Can not call this waitForThisSlaveToEnterWaitState() on a slave core");

		unsafe { rte_eal_wait_lcore(self.0) }
	}

	/// WARNING: If the callback goes out of scope and is dropped, then the pointers passed to C will be come invalid. Be careful!
	/// Slave must be in WAIT state
	/// Err if SLAVE not in WAIT state (ie BUSY)
	/// Get result with waitForThisSlave
	#[inline(always)]
	pub fn runOnSlave<C: MutableCallback1<i32>>(&self, callback: &mut C) -> Result<(), ()>
	{
		debug_assert!(Self::isCurrentMaster(), "Can not call runOnSlave() on a slave core");

		let result = unsafe { rte_eal_remote_launch(C::asFunctionPointer(), callback.asFunctionArgument(), self.0) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::EBUSY => Err(()),

				_ => panic!("Unexptected error '{}' from rte_eal_remote_launch()", result),
			}
		}
	}
}
