// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Status statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProcessStatusStatistics
{
	/// Process name.
	///
	/// Known as `Name`.
	pub process_name: Option<Box<[u8]>>,

	/// File creation mode mask (`umask`).
	///
	/// Known as `Umask`.
	pub file_mode_creation_mask: Option<mode_t>,

	/// State.
	///
	/// Known as `State`.
	///
	/// Note that <> ***does not*** document all possible states.
	pub state: Option<ProcessState>,

	/// Thread group identifier.
	///
	/// Known as `Tgid`.
	pub thread_group_identifier: Option<pid_t>,

	/// NUMA group identifier.
	///
	/// Known as `Ngid`.
	///
	/// Zero if no NUMA is not supported.
	pub numa_group_identifier: Option<NumaNode>,

	/// Process identifier.
	///
	/// Known as `Pid`.
	pub process_identifier: Option<pid_t>,

	/// Parent process identifier.
	///
	/// Known as `PPid`.
	pub parent_process_identifier: Option<pid_t>,

	/// Usually zero, implying no tracer process.
	///
	/// Known as `TracerPid`.
	pub tracer_process_identifier: Option<pid_t>,

	/// User identifiers.
	///
	/// Known as `Uid`.
	pub user_identifiers: Option<ProcessUserIdentifiers>,

	/// Group identifiers.
	///
	/// Known as `Gid`.
	pub group_identifiers: Option<ProcessGroupIdentifiers>,

	/// Number of file descriptor slots currently allocated.
	///
	/// Known as `FDSize`.
	///
	/// eg `64`.
	pub number_of_file_descriptor_slots_currently_allocated: Option<u64>,

	/// Other group memberships.
	///
	/// Known as `Groups`.
	///
	/// Seems to always contain at least one member, which is the same as the primary group of the user.
	pub groups: Option<BTreeSet<gid_t>>,

	/// Descendant namespace thread group identifiers.
	///
	/// Known as `NStgid`.
	pub descendant_namespace_thread_group_identifier: Option<BTreeSet<pid_t>>,

	/// Descendant namespace process identifiers.
	///
	/// Known as `NSpid`.
	pub descendant_namespace_process_identifier: Option<BTreeSet<pid_t>>,

	/// Descendant namespace process group identifiers.
	///
	/// Known as `NSpgid`.
	pub descendant_namespace_process_group_identifier: Option<BTreeSet<pid_t>>,

	/// Descendant namespace session identifiers.
	///
	/// Known as `NSsid`.
	pub descendant_namespace_session_identifier: Option<BTreeSet<pid_t>>,

	/// Peak virtual memory size.
	///
	/// Known as `VmPeak`.
	pub peak_virtual_memory_size: Option<Kilobyte>,

	/// Total program size.
	///
	/// Known as `VmSize`.
	pub total_program_size: Option<Kilobyte>,

	/// Locked memory size.
	///
	/// Known as `VmLck`.
	///
	/// See `man 3 lock`.
	pub locked_memory_size: Option<Kilobyte>,

	/// Pinned memory size (since Linux 3.2).
	///
	/// Known as `VmPin`.
	///
	/// These are pages that can't be moved because something needs to directly access physical memory.
	pub pinned_memory_size: Option<Kilobyte>,

	/// Peak resident set size ("High Water Mark").
	///
	/// Known as `VmHWM`.
	pub peak_resident_set_size: Option<Kilobyte>,

	/// The sum of `anonymous_resident_set_memory_size`, `resident_set_file_mappings_memory_size` and `resident_set_shared_memory_size`.
	///
	/// Known as `VmRSS`.
	pub resident_set_memory_size: Option<Kilobyte>,

	/// Size of resident set anonymous memory (since Linux 4.5).
	///
	/// Known as `RssAnon`.
	pub anonymous_resident_set_memory_size: Option<Kilobyte>,

	/// Size of resident set file mappings (since Linux 4.5).
	///
	/// Known as `RssFile`.
	pub resident_set_file_mappings_memory_size: Option<Kilobyte>,

	/// Size of resident set shared memory (`shmem`) (since Linux 4.5).
	///
	/// Known as `RssShmem`.
	///
	/// Includes Sys_v `shm`, any mappings from `tmpfs` and shared anonymous mappings.
	pub resident_set_shared_memory_size: Option<Kilobyte>,

	/// Size of private data segments.
	///
	/// Known as `VmData`.
	pub private_data_segments_size: Option<Kilobyte>,

	/// Size of stack segments.
	///
	/// Known as `VmStk`.
	pub stack_segments_size: Option<Kilobyte>,

	/// Size of text segment.
	///
	/// Known as `VmExe`.
	pub text_segment_size: Option<Kilobyte>,

	/// Size of shared library code.
	///
	/// Known as `VmLib`.
	pub dynamically_loaded_shared_library_size: Option<Kilobyte>,

	/// Size of page table entries (since Linux 2.6.10).
	///
	/// Known as `VmPTE`.
	pub page_table_entries_size: Option<Kilobyte>,

	/// Size of second-level page tables (since Linux 4.0).
	///
	/// Known as `VmPMD`.
	///
	/// Undocumented in <https://github.com/torvalds/linux/blob/master/Documentation/filesystems/proc.txt>.
	pub vm_pmd: Option<Kilobyte>,

	/// The amount of swap used by anonymous private data (since Linux 2.6.34).
	///
	/// Known as `VmSwap`.
	///
	/// Shared memory `shmem` swap usage is not included.
	pub swap_memory_size: Option<Kilobyte>,

	/// Size of `hugetlb` memory portions.
	///
	/// Known as `HugetlbPages`.
	pub huge_tlb_pages_memory_size: Option<Kilobyte>,

	/// Number of threads.
	///
	/// Known as `Threads`.
	pub threads: Option<u64>,

	/// Signal queue status.
	///
	/// Known as `SigQ`.
	pub signal_queue: Option<SignalQueueStatus>,

	/// Pending signals for the thread.
	///
	/// Known as `SigPnd`.
	pub thread_pending_signals: Option<Bitmask>,

	/// Shared pending signals for the process.
	///
	/// Known as `ShdPnd`.
	pub process_shared_pending_signals: Option<Bitmask>,

	/// Blocked signals.
	///
	/// Known as `SigBlk`.
	pub blocked_signals: Option<Bitmask>,

	/// Ignored signals.
	///
	/// Known as `SigIgn`.
	pub ignored_signals: Option<Bitmask>,

	/// Caught signals.
	///
	/// Known as `SigCgt`.
	pub caught_signals: Option<Bitmask>,

	/// Inheritable capabilities.
	///
	/// Known as `CapInh`.
	pub inheritable_capabilities: Option<Bitmask>,

	/// Permitted capabilities.
	///
	/// Known as `CapPrm`.
	pub permitted_capabilities: Option<Bitmask>,

	/// Effective capabilities.
	///
	/// Known as `CapEff`.
	pub effective_capabilities: Option<Bitmask>,

	/// Capabilities bounding set.
	///
	/// Known as `CapBnd`.
	pub capabilities_bounding_set: Option<Bitmask>,

	/// Ambient capabilities.
	///
	/// Known as `CapAmb`.
	pub ambient_capabilities: Option<Bitmask>,

	/// Thread's `no_new_privs` bit (see `man 2 prctl` description for `PR_GET_NO_NEW_PRIVS`).
	///
	/// Known as `NoNewPrivs`.
	pub thread_no_new_privileges_bit: Option<bool>,

	/// Seccomp mode.
	///
	/// Known as `Seccomp`.
	pub seccomp_mode: Option<SeccompMode>,

	/// Speculation store ('Spectre' vulnerability) bypass status.
	///
	/// Known as `Speculation_Store_Bypass`.
	pub speculation_store_bypass: Option<SpeculationStoreBypassStatus>,

	/// CPUs (actually, hyper threaded cores) allowed for the current process.
	///
	/// Known as `Cpus_allowed`.
	///
	/// May have bits set well beyond those than the number of cores on the system.
	pub cpus_allowed_bitmasks: Option<Vec<HyperThreadBitmask>>,

	/// CPUs (actually, hyper threaded cores) allowed for the current process.
	///
	/// Known as `Cpus_allowed_list`.
	///
	/// May have cores available beyond those than the number of cores on the system, but usually a much more restricted list than `cpus_allowed_bitmask`.
	pub cpus_allowed_list: Option<BTreeSet<HyperThread>>,

	/// NUMA nodes allowed for the current process.
	///
	/// Known as `Mems_allowed`.
	///
	/// Linux defines the config option `NODES_SHIFT` (aka `CONFIG_NODES_SHIFT`) to be 1 to 10 if defined and 0 if not defined, giving a maximum of 2^10 (1024) NUMA nodes, if defaults to 6 (ie 64 NUMA nodes) on x86-64.
	pub numa_nodes_allowed_bitmasks: Option<Vec<NumaNodeBitmask>>,

	/// NUMA nodes allowed for the current process.
	///
	/// Known as `Mems_allowed_list`.
	///
	/// On a non-NUMA system, defaults to 0.
	pub numa_nodes_allowed_list: Option<BTreeSet<NumaNode>>,

	/// Voluntary context switches.
	///
	/// Known as `voluntary_ctxt_switches`.
	pub voluntary_context_switches: Option<u64>,

	/// Involuntary context switches.
	///
	/// Known as `nonvoluntary_ctxt_switches`.
	pub involuntary_context_switches: Option<u64>,

	unrecognised: HashMap<Box<[u8]>, Box<[u8]>>, // eg b"CoreDumping:" b"THP_enabled:" (which seem to exist in Linux source code but aren't documented).
}

impl ProcessStatusStatistics
{
	/// Get an unrecognised static's value using a `statistic_name` byte string.
	#[inline(always)]
	pub fn unrecognised_statistic(&self, statistic_name: &[u8]) -> Option<&Box<[u8]>>
	{
		self.unrecognised.get(statistic_name)
	}

	/// Parses; returns a zero-based line number and parse error if it fails.
	pub fn parse(reader: BufReader<File>) -> Result<Self, ProcessStatusFileParseError>
	{
		use self::ProcessStatusFileParseError::*;

		let mut this = Self::default();

		let mut zero_based_line_number = 0;
		for line in reader.split(b'\n')
		{
			let mut line = match line
			{
				Err(cause) => return Err(CouldNotReadLine { zero_based_line_number, cause }),
				Ok(line) => line,
			};

			{
				let mut split = splitn(&line, 2, b':');

				let statistic_name = split.next().unwrap();

				match split.next()
				{
					None => return Err(CouldNotParseLine { zero_based_line_number, cause: ProcessStatusStatisticParseError::NoValue }),

					Some(tab_then_statistic_value) =>
					{
						if unlikely!(!tab_then_statistic_value.starts_with(b"\t"))
						{
							return Err(CouldNotParseLine { zero_based_line_number, cause: ProcessStatusStatisticParseError::ValueNotPreceededByHorizontalTab })
						}

						let statistic_value = &tab_then_statistic_value[1 ..];

						match this.parse_line(statistic_name, statistic_value)
						{
							Err(cause) => return Err(CouldNotParseLine { zero_based_line_number, cause }),

							Ok(()) => (),
						}
					}
				};
			}

			zero_based_line_number += 1;
		}

		this.unrecognised.shrink_to_fit();

		Ok(this)
	}

	/// When in doubt, check the source code for status files at <https://github.com/torvalds/linux/blob/f346b0becb1bc62e45495f9cdbae3eef35d0b635/fs/proc/array.c>.
	#[inline]
	fn parse_line(&mut self, statistic_name: &[u8], statistic_value: &[u8]) -> Result<(), ProcessStatusStatisticParseError>
	{
		#[inline(always)]
		fn to_box(value: &[u8]) -> Box<[u8]>
		{
			value.to_vec().into_boxed_slice()
		}

		#[inline(always)]
		fn parse_token(value: &[u8]) -> Result<Box<[u8]>, ProcessStatusStatisticParseError>
		{
			Ok(to_box(value))
		}

		#[inline(always)]
		fn parse_mode(value: &[u8]) -> Result<mode_t, ProcessStatusStatisticParseError>
		{
			if likely!(value.len() == 4)
			{
				Ok(mode_t::from_str_radix(from_utf8(value)?, 8)?)
			}
			else
			{
				Err(ProcessStatusStatisticParseError::InvalidLength)
			}
		}

		#[inline(always)]
		fn parse_process_state(value: &[u8]) -> Result<ProcessState, ProcessStatusStatisticParseError>
		{
			// Values are like `R (running)`.
			if unlikely!(value.len() == 0)
			{
				return Err(ProcessStatusStatisticParseError::InvalidLength)
			}

			use self::ProcessState::*;

			let value = match value[0]
			{
				b'R' => Running,
				b'S' => Sleeping,
				b'D' => SleepingInAnUninterruptibleWait,
				b'T' => TracedOrStopped,
				b't' => TracingStop,
				b'X' => Dead,
				b'Z' => Zombie,
				b'P' => Parked,
				b'I' => Idle,
				_ => return Err(ProcessStatusStatisticParseError::OutOfRange)
			};

			Ok(value)
		}

		#[inline(always)]
		fn parse_pid(value: &[u8]) -> Result<pid_t, ProcessStatusStatisticParseError>
		{
			Ok(pid_t::from_str_radix(from_utf8(value)?, 10)?)
		}

		#[inline(always)]
		fn parse_numa_node(value: &[u8]) -> Result<NumaNode, ProcessStatusStatisticParseError>
		{
			Ok(NumaNode(u16::from_str_radix(from_utf8(value)?, 10)?))
		}

		#[inline(always)]
		fn parse_uid(value: &[u8]) -> Result<uid_t, ProcessStatusStatisticParseError>
		{
			Ok(uid_t::from_str_radix(from_utf8(value)?, 10)?)
		}

		#[inline(always)]
		fn parse_gid(value: &[u8]) -> Result<gid_t, ProcessStatusStatisticParseError>
		{
			Ok(gid_t::from_str_radix(from_utf8(value)?, 10)?)
		}

		#[inline(always)]
		fn parse_user_identifiers(value: &[u8]) -> Result<ProcessUserIdentifiers, ProcessStatusStatisticParseError>
		{
			#[inline(always)]
			fn parse_subsequent<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<uid_t, ProcessStatusStatisticParseError>
			{
				if let Some(effective) = iterator.next()
				{
					parse_uid(effective)
				}
				else
				{
					Err(ProcessStatusStatisticParseError::InvalidSeparator)
				}
			}

			let mut iterator = splitn(value, 4, b'\t');

			Ok
			(
				ProcessUserIdentifiers
				{
					real: parse_uid(iterator.next().unwrap())?,
					effective: parse_subsequent(&mut iterator)?,
					saved_set: parse_subsequent(&mut iterator)?,
					file_system: parse_subsequent(&mut iterator)?,
				}
			)
		}

		#[inline(always)]
		fn parse_group_identifiers(value: &[u8]) -> Result<ProcessGroupIdentifiers, ProcessStatusStatisticParseError>
		{
			#[inline(always)]
			fn parse_subsequent<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<gid_t, ProcessStatusStatisticParseError>
			{
				if let Some(effective) = iterator.next()
				{
					parse_gid(effective)
				}
				else
				{
					Err(ProcessStatusStatisticParseError::InvalidSeparator)
				}
			}

			let mut iterator = splitn(value, 4, b'\t');

			Ok
			(
				ProcessGroupIdentifiers
				{
					real: parse_gid(iterator.next().unwrap())?,
					effective: parse_subsequent(&mut iterator)?,
					saved_set: parse_subsequent(&mut iterator)?,
					file_system: parse_subsequent(&mut iterator)?,
				}
			)
		}

		#[inline(always)]
		fn parse_groups(value: &[u8]) -> Result<BTreeSet<gid_t>, ProcessStatusStatisticParseError>
		{
			let mut groups = BTreeSet::new();
			for value in split(value, b' ')
			{
				let was_added_for_the_first_time = groups.insert(parse_gid(value)?);
				if unlikely!(!was_added_for_the_first_time)
				{
					return Err(ProcessStatusStatisticParseError::DuplicatedStatisticValue)
				}
			}
			Ok(groups)
		}

		#[inline(always)]
		fn parse_pids(value: &[u8]) -> Result<BTreeSet<pid_t>, ProcessStatusStatisticParseError>
		{
			let mut pids = BTreeSet::new();
			for value in split(value, b'\t')
			{
				let was_added_for_the_first_time = pids.insert(parse_pid(value)?);
				if unlikely!(!was_added_for_the_first_time)
				{
					return Err(ProcessStatusStatisticParseError::DuplicatedStatisticValue)
				}
			}
			Ok(pids)
		}

		#[inline(always)]
		fn parse_u64(value: &[u8]) -> Result<u64, ProcessStatusStatisticParseError>
		{
			Ok(u64::from_str_radix(from_utf8(value)?, 10)?)
		}

		#[inline(always)]
		fn parse_kb(value: &[u8]) -> Result<Kilobyte, ProcessStatusStatisticParseError>
		{
			const Ending: &'static [u8] = b" kB";

			if likely!(value.ends_with(b" kB"))
			{
				parse_u64(&value[0 .. value.len() - Ending.len()])
			}
			else
			{
				Err(ProcessStatusStatisticParseError::InvalidEnding)
			}
		}

		#[inline(always)]
		fn parse_signal_queue(value: &[u8]) -> Result<SignalQueueStatus, ProcessStatusStatisticParseError>
		{
			// number of signals queued/max. number for queue
			let mut iterator = splitn(value, 2, b'/');
			let number_of_signals_queued = parse_u64(iterator.next().unwrap())?;
			let maximum_number_of_signals_that_can_be_queued = match iterator.next()
			{
				None => return Err(ProcessStatusStatisticParseError::InvalidSeparator),

				Some(maximum_number_of_signals_that_can_be_queued) => parse_u64(maximum_number_of_signals_that_can_be_queued)?,
			};

			Ok
			(
				SignalQueueStatus
				{
					number_of_signals_queued,
					maximum_number_of_signals_that_can_be_queued
				}
			)
		}

		#[inline(always)]
		fn parse_bitmask(value: &[u8]) -> Result<Bitmask, ProcessStatusStatisticParseError>
		{
			if likely!(value.len() == 16)
			{
				Ok(u64::from_str_radix(from_utf8(value)?, 16)?)
			}
			else
			{
				Err(ProcessStatusStatisticParseError::InvalidLength)
			}
		}

		#[inline(always)]
		fn parse_bool(value: &[u8]) -> Result<bool, ProcessStatusStatisticParseError>
		{
			if likely!(value.len() == 1)
			{
				match value[0]
				{
					b'0' => Ok(false),
					b'1' => Ok(true),
					_ => Err(ProcessStatusStatisticParseError::OutOfRange)
				}
			}
			else
			{
				Err(ProcessStatusStatisticParseError::InvalidLength)
			}
		}

		#[inline(always)]
		fn parse_seccomp_mode(value: &[u8]) -> Result<SeccompMode, ProcessStatusStatisticParseError>
		{
			if likely!(value.len() == 1)
			{
				use self::SeccompMode::*;

				match value[0]
				{
					b'0' => Ok(Off),
					b'1' => Ok(Strict),
					b'2' => Ok(Filter),
					_ => Err(ProcessStatusStatisticParseError::OutOfRange)
				}
			}
			else
			{
				Err(ProcessStatusStatisticParseError::InvalidLength)
			}
		}

		#[inline(always)]
		fn parse_speculation_store_bypass(value: &[u8]) -> Result<SpeculationStoreBypassStatus, ProcessStatusStatisticParseError>
		{
			use self::SpeculationStoreBypassStatus::*;

			let value = match value
			{
				b"unknown" => SpeculationStoreBypassStatus::Unknown,
				b"not vulnerable" => NotVulnerable,
				b"thread force mitigated" => ThreadForceMitigated,
				b"thread mitigated" => ThreadMitigated,
				b"thread vulnerable" => ThreadVulnerable,
				b"globally mitigated" => GloballyMitigated,
				b"vulnerable" => Vulnerable,
				_ => return Err(ProcessStatusStatisticParseError::OutOfRange),
			};
			Ok(value)
		}

		#[inline(always)]
		fn parse_cpus_or_numa_nodes_allowed_bitmasks(value: &[u8]) -> Result<Vec<u32>, ProcessStatusStatisticParseError>
		{
			let iterator = split(value, b',');
			let mut bitmasks = Vec::with_capacity(1);

			for raw_value in iterator
			{
				if likely!(raw_value.len() <= 8 && raw_value.len() != 0)
				{
					bitmasks.push(u32::from_str_radix(from_utf8(value)?, 16)?);
				}
				else
				{
					return Err(ProcessStatusStatisticParseError::InvalidLength)
				}
			}
			Ok(bitmasks)
		}

		#[inline(always)]
		fn parse_cpus_allowed_list(value: &[u8]) -> Result<BTreeSet<HyperThread>, ProcessStatusStatisticParseError>
		{
			Ok(ListParseError::parse_linux_list_string(value, HyperThread)?)
		}

		#[inline(always)]
		fn parse_numa_nodes_allowed_list(value: &[u8]) -> Result<BTreeSet<NumaNode>, ProcessStatusStatisticParseError>
		{
			Ok(ListParseError::parse_linux_list_string(value, NumaNode)?)
		}

		macro_rules! parse
		{
			($statistic_name: ident, $statistic_value: ident, $($proc_status_name: literal => $struct_field: ident @ $parse_expr: ident,)*) =>
			{
				match $statistic_name
				{
					$(
						$proc_status_name => if unlikely!(self.$struct_field.is_some())
						{
							Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
						}
						else
						{
							let result = $parse_expr($statistic_value);
							let parsed_value = result?;
							let some = Some(parsed_value);
							self.$struct_field = some;
							Ok(())
						},
					)*

					_ =>
					{
						let previous = self.unrecognised.insert(to_box($statistic_name), to_box($statistic_value));
						return if unlikely!(previous.is_some())
						{
							Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
						}
						else
						{
							Ok(())
						}
					}
				}
			}
		}

		parse!
		(
			statistic_name, statistic_value,

			b"Name" => process_name @ parse_token,
			b"Umask" => file_mode_creation_mask @ parse_mode,
			b"State" => state @ parse_process_state,
			b"Tgid" => thread_group_identifier @ parse_pid,
			b"Ngid" => numa_group_identifier @ parse_numa_node,
			b"Pid" => process_identifier @ parse_pid,
			b"PPid" => parent_process_identifier @ parse_pid,
			b"TracerPid" => tracer_process_identifier @ parse_pid,
			b"Uid" => user_identifiers @ parse_user_identifiers,
			b"Gid" => group_identifiers @ parse_group_identifiers,
			b"FDSize" => number_of_file_descriptor_slots_currently_allocated @ parse_u64,
			b"Groups" => groups @ parse_groups,
			b"NStgid" => descendant_namespace_thread_group_identifier @ parse_pids,
			b"NSpid" => descendant_namespace_process_identifier @ parse_pids,
			b"NSpgid" => descendant_namespace_process_group_identifier @ parse_pids,
			b"NSsid" => descendant_namespace_session_identifier @ parse_pids,
			b"VmPeak" => peak_virtual_memory_size @ parse_kb,
			b"VmSize" => total_program_size @ parse_kb,
			b"VmLck" => locked_memory_size @ parse_kb,
			b"VmPin" => pinned_memory_size @ parse_kb,
			b"VmHWM" => peak_resident_set_size @ parse_kb,
			b"VmRSS" => resident_set_memory_size @ parse_kb,
			b"RssAnon" => anonymous_resident_set_memory_size @ parse_kb,
			b"RssFile" => resident_set_file_mappings_memory_size @ parse_kb,
			b"RssShmem" => resident_set_shared_memory_size @ parse_kb,
			b"VmData" => private_data_segments_size @ parse_kb,
			b"VmStk" => stack_segments_size @ parse_kb,
			b"VmExe" => text_segment_size @ parse_kb,
			b"VmLi" => dynamically_loaded_shared_library_size @ parse_kb,
			b"VmPTE" => page_table_entries_size @ parse_kb,
			b"VmPMD" => vm_pmd @ parse_kb,
			b"VmSwap" => swap_memory_size @ parse_kb,
			b"HugetlbPages" => huge_tlb_pages_memory_size @ parse_kb,
			b"Threads" => threads @ parse_u64,
			b"SigQ" => signal_queue @ parse_signal_queue,
			b"SigPnd" => thread_pending_signals @ parse_bitmask,
			b"ShdPnd" => process_shared_pending_signals @ parse_bitmask,
			b"SigBlk" => blocked_signals @ parse_bitmask,
			b"SigIgn" => ignored_signals @ parse_bitmask,
			b"SigCgt" => caught_signals @ parse_bitmask,
			b"CapInh" => inheritable_capabilities @ parse_bitmask,
			b"CapPrm" => permitted_capabilities @ parse_bitmask,
			b"CapEff" => effective_capabilities @ parse_bitmask,
			b"CapBnd" => capabilities_bounding_set @ parse_bitmask,
			b"CapAm" => ambient_capabilities @ parse_bitmask,
			b"NoNewPrivs" => thread_no_new_privileges_bit @ parse_bool,
			b"Seccomp" => seccomp_mode @ parse_seccomp_mode,
			b"Speculation_Store_Bypass" => speculation_store_bypass @ parse_speculation_store_bypass,
			b"Cpus_allowed" => cpus_allowed_bitmasks @ parse_cpus_or_numa_nodes_allowed_bitmasks,
			b"Cpus_allowed_list" => cpus_allowed_list @ parse_cpus_allowed_list,
			b"Mems_allowed" => numa_nodes_allowed_bitmasks @ parse_cpus_or_numa_nodes_allowed_bitmasks,
			b"Mems_allowed_list" => numa_nodes_allowed_list @ parse_numa_nodes_allowed_list,
			b"voluntary_ctxt_switches" => voluntary_context_switches @ parse_u64,
			b"nonvoluntary_ctxt_switches" => involuntary_context_switches @ parse_u64,
		)
	}
}
