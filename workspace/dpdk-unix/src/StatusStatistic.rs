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



/// A list of known virtual memory statistics related to NUMA nodes.
///
/// There are far more statistics than those listed here.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusStatistic
{
	Name(Box<[u8]>),
	FileModeCreationMask(mode_t),
}


impl StatusStatistic
{
	#[inline]
	pub(crate) fn parse(name: &[u8], value: &[u8])
	{
		use self::StatusStatistic::*;
		
		match name
		{
			b"Name" => Name,
			b"Umask" => FileModeCreationMask,
			b"State" => State,
			b"Tgid" => ThreadGroupIdentifier,

			other @ _ =>
			{
				let name = other.to_vec().into_boxed_slice();
				let value = value.to_vec().into_boxed_slice();
			}
		}

		/*
		Name:	cat
		Umask:	0022
		State:	R (running)
		Tgid:	28392
		Ngid:	0
		Pid:	28392
		PPid:	28391
		TracerPid:	0
		Uid:	1000	1000	1000	1000
		Gid:	1000	1000	1000	1000
		FDSize:	64
		Groups:	1000
		NStgid:	28392
		NSpid:	28392
		NSpgid:	28392
		NSsid:	28391
		VmPeak:	    3544 kB
		VmSize:	    1516 kB
		VmLck:	       0 kB
		VmPin:	       0 kB
		VmHWM:	       4 kB
		VmRSS:	       4 kB
		RssAnon:	       4 kB
		RssFile:	       0 kB
		RssShmem:	       0 kB
		VmData:	      20 kB
		VmStk:	     132 kB
		VmExe:	     760 kB
		VmLib:	     572 kB
		VmPTE:	      24 kB
		VmPMD:	      12 kB
		VmSwap:	       0 kB
		HugetlbPages:	       0 kB
		Threads:	1
		SigQ:	0/3913
		SigPnd:	0000000000000000
		ShdPnd:	0000000000000000
		SigBlk:	0000000000000000
		SigIgn:	0000000000000000
		SigCgt:	0000000000000000
		CapInh:	0000000000000000
		CapPrm:	0000000000000000
		CapEff:	0000000000000000
		CapBnd:	0000003fffffffff
		CapAmb:	0000000000000000
		NoNewPrivs:	0
		Seccomp:	0
		Speculation_Store_Bypass:	vulnerable
		Cpus_allowed:	ffffffff
		Cpus_allowed_list:	0-31
		Mems_allowed:	1
		Mems_allowed_list:	0
		voluntary_ctxt_switches:	0
		nonvoluntary_ctxt_switches:	0

		*/
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
	Kilobytes(usize),

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
