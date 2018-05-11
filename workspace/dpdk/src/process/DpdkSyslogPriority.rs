// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a DPDK syslog priority.
///
/// Defaults to `debug` for debug builds and `warning` for production builds.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum DpdkSyslogPriority
{
	emergency,
	alert,
	critical,
	error,
	warning,
	notice,
	info,
	debug,
}

impl Default for DpdkSyslogPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::DpdkSyslogPriority::*;
		
		if cfg!(debug_assertions)
		{
			debug
		}
		else
		{
			warning
		}
	}
}

impl DpdkSyslogPriority
{
	#[inline(always)]
	pub(crate) fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::DpdkSyslogPriority::*;
		
		match self
		{
			emergency => const_cstr!("emergency"),
			alert => const_cstr!("alert"),
			critical => const_cstr!("critical"),
			error => const_cstr!("error"),
			warning => const_cstr!("warning"),
			notice => const_cstr!("notice"),
			info => const_cstr!("info"),
			debug => const_cstr!("debug"),
		}
	}
}
