// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Nice settings for the current process.
///
/// Defaults to aggresive promotion of the current process.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize)]
#[serde(default)]
pub struct ProcessNiceness
{
	/// Downgrade all other process for the current user to this value.
	pub all_other_processes_for_current_user: Nice,

	/// Downgrade all other process for the process group to this value.
	pub all_other_processes_in_process_group: Nice,

	/// Boost this process to this value.
	pub our_process: Nice,

	/// If autogroups are enabled, should we take as close to 100% of all CPU cycles in the autogroup?
	pub share_of_cpu_cycles_in_autogroup: Option<Nice>,
}

impl Default for ProcessNiceness
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
			{
				all_other_processes_for_current_user: Nice::Positive_19,
				all_other_processes_in_process_group: Nice::Positive_19,
				our_process: Nice::Negative_20,
				share_of_cpu_cycles_in_autogroup: Some(Nice::Negative_20),
			}
	}
}

impl ProcessNiceness
{
	/// Adjusts in favour of the current process.
	pub fn adjust(&self, proc_path: &ProcPath) -> Result<(), ProcessNicenessAdjustmentError>
	{
		use self::ProcessNicenessAdjustmentError::*;

		if let Err(_) = self.all_other_processes_for_current_user.set_current_real_effective_user_priority()
		{
			return Err(CouldNotSetCurrentRealEffectiveUserPriorityNiceness)
		}

		if let Err(_) = self.all_other_processes_in_process_group.set_current_process_group_priority()
		{
			return Err(CouldNotSetCurrentProcessGroupPriorityNiceness)
		}

		if let Err(_) = self.our_process.set_current_process_priority()
		{
			return Err(CouldNotSetCurrentProcessPriorityNiceness)
		}

		Nice::set_autogroup_for_current_process_if_desired(self.share_of_cpu_cycles_in_autogroup, proc_path)?;

		Ok(())
	}
}

