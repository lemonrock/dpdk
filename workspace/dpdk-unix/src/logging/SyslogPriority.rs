// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.
//


/// Represents a DPDK syslog priority.
///
/// Defaults to `debug` for debug builds and `warning` for production builds.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
#[repr(i32)]
pub enum SyslogPriority
{
	emergency = LOG_EMERG,
	alert = LOG_ALERT,
	critical = LOG_CRIT,
	error = LOG_ERR,
	warning = LOG_WARNING,
	notice = LOG_NOTICE,
	info = LOG_INFO,
	debug = LOG_DEBUG,
}

impl Default for SyslogPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::SyslogPriority::*;
		
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

impl SyslogPriority
{
	/// As an initialization argument for DPDK.
	#[inline(always)]
	pub fn as_initialization_argument(self) -> ConstCStr
	{
		use self::SyslogPriority::*;
		
		match self
		{
			emergency => ConstCStr(b"emergency\0"),
			alert => ConstCStr(b"alert\0"),
			critical => ConstCStr(b"critical\0"),
			error => ConstCStr(b"error\0"),
			warning => ConstCStr(b"warning\0"),
			notice => ConstCStr(b"notice\0"),
			info => ConstCStr(b"info\0"),
			debug => ConstCStr(b"debug\0"),
		}
	}
	
	#[inline(always)]
	pub(crate) fn log_upto(self) -> i32
	{
		(1 << ((self as i32) + 1)) - 1
	}
}
