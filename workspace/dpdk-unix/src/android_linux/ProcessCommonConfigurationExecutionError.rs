// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.
//


/// A process common configuration execution error.
#[derive(Debug)]
pub enum ProcessCommonConfigurationExecutionError
{
	/// Process niceness adjustment failed.
	ProcessNicenessAdjustmentFailed(ProcessNicenessAdjustmentError),

	/// Process affinity setting failed.
	CouldNotSetCurrentProcessAffinity(io::Error),

	/// Could not load kernel modules (explanation in tuple argument).
	CouldNotLoadKernelModules(String),

	/// Could not write system control values.
	CouldNotWriteSystemControlValues(io::Error),

	/// Rescan of all PCI buses and devices failed.
	RescanOfAllPciBusesAndDevices(io::Error),

	/// CPU features failed validation (explanation in tuple argument).
	CpuFeaturesValidationFailed(String),

	/// Linux kernel command line failed validation (explanation in tuple argument).
	LinuxKernelCommandLineValidationFailed(String),

	/// Could not set work queue hyper thread affinity to online shared hyper threads.
	///
	/// Shared hyper threads are those shared with the operating system and other processes (ie not isolated).
	CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads(io::Error),

	/// Could not force watchdog hyper thread affinity to online shared hyper threads.
	///
	/// Shared hyper threads are those shared with the operating system and other processes (ie not isolated).
	CouldNotForceWatchdogHyperThreadAffinityToOnlineSharedHyperThreads(io::Error),

	/// Could not disable Transparent Huge Pages (THP).
	CouldNotDisableTransparentHugePages(DisableTransparentHugePagesError),

	/// Execution failed (with description of reason).
	ExecutionFailed(String),
}

impl Display for ProcessCommonConfigurationExecutionError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ProcessCommonConfigurationExecutionError as Debug>::fmt(self, f)
	}
}

impl error::Error for ProcessCommonConfigurationExecutionError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::ProcessCommonConfigurationExecutionError::*;

		match self
		{
			&ProcessNicenessAdjustmentFailed(ref error) => Some(error),

			&CouldNotSetCurrentProcessAffinity(ref error) => Some(error),

			&CouldNotLoadKernelModules(_) => None,

			&CouldNotWriteSystemControlValues(ref error) => Some(error),

			&RescanOfAllPciBusesAndDevices(ref error) => Some(error),

			&CpuFeaturesValidationFailed(_) => None,

			&LinuxKernelCommandLineValidationFailed(_) => None,

			&CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads(ref error) => Some(error),

			&CouldNotForceWatchdogHyperThreadAffinityToOnlineSharedHyperThreads(ref error) => Some(error),

			&CouldNotDisableTransparentHugePages(ref error) => Some(error),

			&ExecutionFailed(..) => None,
		}
	}
}

impl From<ProcessNicenessAdjustmentError> for ProcessCommonConfigurationExecutionError
{
	#[inline(always)]
	fn from(error: ProcessNicenessAdjustmentError) -> Self
	{
		ProcessCommonConfigurationExecutionError::ProcessNicenessAdjustmentFailed(error)
	}
}

impl From<DisableTransparentHugePagesError> for ProcessCommonConfigurationExecutionError
{
	#[inline(always)]
	fn from(error: DisableTransparentHugePagesError) -> Self
	{
		ProcessCommonConfigurationExecutionError::CouldNotDisableTransparentHugePages(error)
	}
}

impl From<String> for ProcessCommonConfigurationExecutionError
{
	#[inline(always)]
	fn from(explanation: String) -> Self
	{
		ProcessCommonConfigurationExecutionError::ExecutionFailed(explanation)
	}
}
