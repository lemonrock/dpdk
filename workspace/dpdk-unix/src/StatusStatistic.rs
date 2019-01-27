// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Status statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct StatusStatistics
{
	known: HashMap<Discriminant<StatusStatistic>, StatusStatistic>,
	unknown: HashMap<Box<[u8]>, Box<[u8]>>,
}

impl StatusStatistics
{
	fn parse(mut reader: BufReader<File>) -> Self
	{
		let mut known = HashMap::with_capacity(64);
		let mut unknown = HashMap::with_capacity(64);

		fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize>


		let mut line = Vec::with_capacity(64);
		while reader.read_until('\n', &mut line)? > 0
		{
			let line = if line[line.len() - 1] == '\n'
			{
				&mut line[0 .. line.len() - 1]
			}
			else
			{
				line
			};

			{
				let line_without_ = &line[0 .. line.len() - 1];

				use self::ErrorKind::InvalidData;

				let mut split = line.splitn(2, ":\t");

				let statistic_name = StatusStatisticName::parse(split.next().unwrap());

				let statistic_value = match split.next()
				{
					None => return Err(io::Error::new(InvalidData, format!("Zero based line '{}' does not have a value second column", zero_based_line_number))),
					Some(value) =>
					{
						match value.parse::<u64>()
						{
							Err(parse_error) => return Err(io::Error::new(InvalidData, parse_error)),
							Ok(value) => value,
						}
					}
				};

				if let Some(previous) = statistics.insert(statistic_name, statistic_value)
				{
					return Err(io::Error::new(InvalidData, format!("Zero based line '{}' has a duplicate statistic (was '{}')", zero_based_line_number, previous)))
				}
			}

			line.clear();
			zero_based_line_number += 1;
		}


		known.shrink_to_fit();
		unknown.shrink_to_fit();

		Self
		{
			known,
			unknown,
		}
	}
}


/// A parse error.
#[derive(Debug)]
pub enum StatusStatisticParseError
{
	/// Length was invalid.
	InvalidLength,

	/// Ending was invalid.
	InvalidEnding,

	/// Value was not a valid UTF-8 string.
	NotAUtf8String(Utf8Error),

	/// Value was not a valid integer.
	NotAValidInteger(ParseIntError),

	/// Value was out-of-range, eg `2` for a `bool`.
	OutOfRange,
}

impl From<Utf8Error> for StatusStatisticParseError
{
	#[inline(always)]
	fn from(error: Utf8Error) -> Self
	{
		StatusStatisticParseError::NotAUtf8String(error)
	}
}

impl From<ParseIntError> for StatusStatisticParseError
{
	#[inline(always)]
	fn from(error: ParseIntError) -> Self
	{
		StatusStatisticParseError::NotAValidInteger(error)
	}
}

/// A kilobyte.
pub type StatusStatisticKilobyte = u64;

/// A bitmask.
pub type StatusStatisticBitmask = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum StatusStatisticSeccompMode
{
	/// Off.
	Off = 0,

	/// Strict.
	Strict = 1,

	/// Filter.
	Filter = 2,
}


/// A list of known virtual memory statistics related to NUMA nodes.
///
/// There are far more statistics than those listed here.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusStatistic
{
	/// Process name.
	ProcessName(Box<[u8]>),

	/// File creation mode mask (`umask`).
	FileModeCreationMask(mode_t),

	/// State, eg `R (running)`.
	State(Box<[u8]>),

	/// Thread group identifier.
	ThreadGroupIdentifier(pid_t),

	/// NUMA group identifier.
	///
	/// Zero if no NUMA is not supported.
	NumaGroupIdentifier(pid_t),

	/// Process identifier.
	ProcessIdentifier(pid_t),

	/// Parent process identifier.
	ParentProcessIdentifier(pid_t),

	/// Usually zero, implying no tracer process.
	TracerProcessIdentifier(pid_t),

	/// eg `64`.
	FileDescriptorSize(u8),

	/// Descendant namespace thread group identifier.
	DescendantNamespaceThreadGroupIdentifier(pid_t),

	/// Descendant namespace process identifier.
	DescendantNamespaceProcessIdentifier(pid_t),

	/// Descendant namespace process group identifier.
	DescendantNamespaceProcessGroupIdentifier(pid_t),

	/// Descendant namespace session identifier.
	DescendantNamespaceSessionIdentifier(pid_t),

	/// Peak virtual memory size.
	PeakVirtualMemorySize(StatusStatisticKilobyte),

	/// Total program size.
	TotalProgramSize(StatusStatisticKilobyte),

	/// Locked memory size.
	LockedMemorySize(StatusStatisticKilobyte),

	/// Pinned memory size.
	PinnedMemorySize(StatusStatisticKilobyte),

	/// Peak resident set size ("High Water Mark").
	PeakResidentSetSize(StatusStatisticKilobyte),

	/// The sum of `AnonymousResidentSetMemorySize`, `ResidentSetFileMappingsMemorySize` and `ResidentSetSharedMemorySize`.
	ResidentSetMemorySize(StatusStatisticKilobyte),

	/// Size of resident set anonymous memory.
	AnonymousResidentSetMemorySize(StatusStatisticKilobyte),

	/// Size of resident set file mappings.
	ResidentSetFileMappingsMemorySize(StatusStatisticKilobyte),

	/// Size of resident set shared memory (`shmem`) (includes SysV `shm`, any mappings from `tmpfs` and shared anonymous mappings).
	ResidentSetSharedMemorySize(StatusStatisticKilobyte),

	PrivateDataSegmentsSize(StatusStatisticKilobyte),

	StackSegmentsSize(StatusStatisticKilobyte),

	TextSegmentSize(StatusStatisticKilobyte),

	DynamicallyLoadedSharedLibrarySize(StatusStatisticKilobyte),

	PageTableEntriesSize(StatusStatisticKilobyte),

	VmPMD(StatusStatisticKilobyte),

	/// The amount of swap used by anonymous private data (shared memory `shmem` swap usage is not included).
	SwapMemorySize(StatusStatisticKilobyte),

	/// Size of `hugetlb` memory portions.
	HugeTlbPagesMemorySize(StatusStatisticKilobyte),

	/// Number of threads.
	Threads(u64),

	/// Pending signals for the thread.
	ThreadPendingSignals(StatusStatisticBitmask),

	/// Shared pending signals for the process.
	ProcessSharedPendingSignals(StatusStatisticBitmask),

	/// Blocked signals.
	BlockedSignals(StatusStatisticBitmask),

	/// Ignored signals.
	IgnoredSignals(StatusStatisticBitmask),

	/// Caught signals.
	CaughtSignals(StatusStatisticBitmask),

	/// Inheritable capabilities.
	InheritableCapabilities(StatusStatisticBitmask),

	/// Permitted capabilities.
	PermittedCapabilities(StatusStatisticBitmask),

	/// Effective capabilities.
	EffectiveCapabilities(StatusStatisticBitmask),

	/// Capabilities bounding set.
	CapabilitiesBoundingSet(StatusStatisticBitmask),

	/// Ambient capabilities.
	AmbientCapabilities(StatusStatisticBitmask),

	/// Thread's `no_new_privs` bit (see `man 2 prctl` description for `PR_GET_NO_NEW_PRIVS`).
	ThreadNoNewPrivilegesBit(StatusStatisticBitmask),

	/// Seccomp mode.
	SeccompMode(StatusStatisticSeccompMode),



	/// Voluntary context switches.
	VoluntaryContextSwitches(u64),

	/// Involuntary context switches.
	InvoluntaryContextSwitches(u64),

	/// Unknown value.
	Unknown
	{
		name: Box<[u8]>,
		value: Box<[u8]>,
	}
}

impl StatusStatistic
{
	#[inline]
	pub(crate) fn parse_line(name: &[u8], value: &[u8]) -> Result<StatusStatistic, StatusStatisticParseError>
	{
		use self::StatusStatistic::*;

		#[inline(always)]
		fn parse_token(value: &[u8]) -> Box<[u8]>
		{
			value.to_vec().into_boxed_slice()
		}

		#[inline(always)]
		fn parse_pid(value: &[u8]) -> Result<pid_t, StatusStatisticParseError>
		{
			pid_t::from_str_radix(from_str(value)?, 10)?
		}

		#[inline(always)]
		fn parse_mode(value: &[u8]) -> Result<mode_t, StatusStatisticParseError>
		{
			if likely!(value.len() == 4)
			{
				mode_t::from_str_radix(from_str(value)?, 8)?
			}
			else
			{
				Err(InvalidLength)
			}
		}

		#[inline(always)]
		fn parse_u64(value: &[u8]) -> Result<u64, StatusStatisticParseError>
		{
			if likely!(value.ends_with(" kB"))
			{
				u64::from_str_radix(from_str(value)?, 10)?
			}
			else
			{
				Err(InvalidEnding)
			}
		}

		#[inline(always)]
		fn parse_kb(value: &[u8]) -> Result<StatusStatisticKilobyte, StatusStatisticParseError>
		{
			parse_u64(value)
		}

		#[inline(always)]
		fn parse_bitmask(value: &[u8]) -> Result<StatusStatisticBitmask, StatusStatisticParseError>
		{
			if likely!(value.len() == 16)
			{
				u64::from_str_radix(from_str(value)?, 16)?
			}
			else
			{
				Err(InvalidLength)
			}
		}

		#[inline(always)]
		fn parse_bool(value: &[u8]) -> Result<bool, StatusStatisticParseError>
		{
			if likely!(value.len() == 1)
			{
				match value[0]
				{
					b'0' => Ok(false),
					b'1' => Ok(true),
					_ => Err(StatusStatisticParseError::OutOfRange)
				}
			}
			else
			{
				Err(InvalidLength)
			}
		}

		#[inline(always)]
		fn parse_seccomp_mode(value: &[u8]) -> Result<SeccompMode, StatusStatisticParseError>
		{
			if likely!(value.len() == 1)
			{
				use self::StatusStatisticSeccompMode::*;

				match value[0]
				{
					b'0' => Ok(Off),
					b'1' => Ok(Strict),
					b'2' => Ok(Filter),
					_ => Err(StatusStatisticParseError::OutOfRange)
				}
			}
			else
			{
				Err(InvalidLength)
			}
		}

		let parsed = match name
		{
			b"Name" => ProcessName(parse_token(value)),

			b"Umask" => FileModeCreationMask(parse_mode(value)?),

			// TODO: convert to an enumerated value from eg "R (running)"
			b"State" => State(parse_token(value)),

			b"Tgid" => ThreadGroupIdentifier(parse_pid(value)?),

			b"Ngid" => NumaGroupIdentifier(parse_pid(value)?),

			b"Pid" => ProcessIdentifier(parse_pid(value)?),

			b"PPid" => ParentProcessIdentifier(parse_pid(value)?),

			b"TracerPid" => TracerProcessIdentifier(parse_pid(value)?),

			// TODO
			x
			/*
		Uid:	1000	1000	1000	1000
		Gid:	1000	1000	1000	1000
			*/

			b"FDSize" => FileDescriptorSize(u8::from_str_radix(from_str(value)?)?),

			//TODO 			Groups:	1000   (this is a list, space separated)
			x

			b"DescendantNamespacetgid" => DescendantNamespaceThreadGroupIdentifier(parse_pid(value)?),

			b"DescendantNamespacepid" => DescendantNamespaceProcessIdentifier(parse_pid(value)?),

			b"DescendantNamespacepgid" => DescendantNamespaceProcessGroupIdentifier(parse_pid(value)?),

			b"DescendantNamespacesid" => DescendantNamespaceSessionIdentifier(parse_pid(value)?),

			b"VmPeak" => PeakVirtualMemorySize(parse_kb(value)?),
			
			b"VmSize" => TotalProgramSize(parse_kb(value)?),
			
			b"VmLck" => LockedMemorySize(parse_kb(value)?),
			
			b"VmPin" => PinnedMemorySize(parse_kb(value)?),
			
			b"VmHWM" => PeakResidentSetSize(parse_kb(value)?),
			
			b"VmRSS" => ResidentSetMemorySize(parse_kb(value)?),
			
			b"RssAnon" => AnonymousResidentSetMemorySize(parse_kb(value)?),
			
			b"RssFile" => ResidentSetFileMappingsMemorySize(parse_kb(value)?),
			
			b"RssShmem" => ResidentSetSharedMemorySize(parse_kb(value)?),
			
			b"VmData" => PrivateDataSegmentsSize(parse_kb(value)?),
			
			b"VmStk" => StackSegmentsSize(parse_kb(value)?),
			
			b"VmExe" => TextSegmentSize(parse_kb(value)?),
			
			b"VmLib" => DynamicallyLoadedSharedLibrarySize(parse_kb(value)?),
			
			b"VmPTE" => PageTableEntriesSize(parse_kb(value)?),
			
			b"VmPMD" => VmPMD(parse_kb(value)?),
			
			b"VmSwap" => SwapMemorySize(parse_kb(value)?),
			
			b"HugetlbPages" => HugeTlbPagesMemorySize(parse_kb(value)?),

			b"Threads" => Threads(parse_u64(value)?),

			// TODO
			x
			// SigQ:	0/3913

			b"SigPnd" => ThreadPendingSignals(parse_bitmask(value)?),
		
			b"ShdPnd" => ProcessSharedPendingSignals(parse_bitmask(value)?),
			
			b"SigBlk" => BlockedSignals(parse_bitmask(value)?),
			
			b"SigIgn" => IgnoredSignals(parse_bitmask(value)?),
			
			b"SigCgt" => CaughtSignals(parse_bitmask(value)?),
			
			b"CapInh" => InheritableCapabilities(parse_bitmask(value)?),
			
			b"CapPrm" => PermittedCapabilities(parse_bitmask(value)?),
			
			b"CapEff" => EffectiveCapabilities(parse_bitmask(value)?),
			
			b"CapBnd" => CapabilitiesBoundingSet(parse_bitmask(value)?),
			
			b"CapAmb" => AmbientCapabilities(parse_bitmask(value)?),

			b"NoNewPrivs" => ThreadNoNewPrivilegesBit(parse_bool(value)?),

			b"Seccomp" => SeccompMode(parse_seccomp_mode(value)?),

			// TODO: Convert to named token.
			b"Speculation_Store_Bypass" => SpeculationStoreBypass(parse_token(value)),


			// TODO:
			x
			/*
			Cpus_allowed:	ffffffff
			Cpus_allowed_list:	0-31


			*/


			// This is NUMA.
			/*

			Mems_allowed:	1
			Mems_allowed_list:	0
			*/



			b"voluntary_ctxt_switches" => VoluntaryContextSwitches(parse_u64(value)?),

			b"nonvoluntary_ctxt_switches" => InvoluntaryContextSwitches(parse_u64(value)?),

			_ => Unknown
			{
				name: parse_token(name),
				value: parse_token(value),
			}
		};
		Ok(parsed)
	}
	}
}

/*
    PID: Process Id
    PPID: Parent Process Id (the one which launched this PID)
    TGID: Thread Group Id

*/

pub enum StatusStatisticValue
{
	/// A string, such as a process name; not necessarily UTF-8 encoded.
	String(Box<[u8]>),

	/// eg `R (running)` for `State` or `vulnerable` for `Speculation_Store_Bypass`.
	Token(Box<[u8]>),

	FileModeCreationMask(mode_t),

	/// A PID, TID, SID or the like.
	ProcessIdentifierLike(pid_t),

	UserIdentifiers(uid_t, uid_t, uid_t, uid_t),

	GroupIdentifiers(gid_t, gid_t, gid_t, gid_t),

	/// Will always contain at least one entry.
	Groups(HashSet<gid_t>),

	/// A memory size in kilobytes.
	StatusStatisticKilobytes(usize),

	UnsignedInteger(usize),

	SignalQueue(usize, usize),

	/// eg `0000000000000000` for `SigPnd`.
	SignalBitMask(u64),

	/// eg `0000003fffffffff` for `CapBnd`.
	CapabilityBitMask(u64),

	ContextSwitches(u64),

	// CPU mask
	// CPU list
	// Mem mask
	// Mem list
}
