// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a DPDK syslog facility.
///
/// Defaults to `auth`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
#[repr(i32)]
pub enum DpdkSyslogFacility
{
	auth = LOG_AUTH,
	cron = LOG_CRON,
	daemon = LOG_DAEMON,
	ftp = LOG_FTP,
	kern = LOG_KERN,
	lpr = LOG_LPR,
	mail = LOG_MAIL,
	news = LOG_NEWS,
	syslog = LOG_SYSLOG,
	user = LOG_USER,
	uucp = LOG_UUCP,
	local0 = LOG_LOCAL0,
	local1 = LOG_LOCAL1,
	local2 = LOG_LOCAL2,
	local3 = LOG_LOCAL3,
	local4 = LOG_LOCAL4,
	local5 = LOG_LOCAL5,
	local6 = LOG_LOCAL6,
	local7 = LOG_LOCAL7,
}

impl Default for DpdkSyslogFacility
{
	#[inline(always)]
	fn default() -> Self
	{
		DpdkSyslogFacility::auth
	}
}

impl DpdkSyslogFacility
{
	#[inline(always)]
	pub(crate) fn as_initialization_argument(self) -> ConstCStr
	{
		use self::DpdkSyslogFacility::*;
		
		match self
		{
			auth => ConstCStr(b"auth\0"),
			cron => ConstCStr(b"cron\0"),
			daemon => ConstCStr(b"daemon\0"),
			ftp => ConstCStr(b"ftp\0"),
			kern => ConstCStr(b"kern\0"),
			lpr => ConstCStr(b"lpr\0"),
			mail => ConstCStr(b"mail\0"),
			news => ConstCStr(b"news\0"),
			syslog => ConstCStr(b"syslog\0"),
			user => ConstCStr(b"user\0"),
			uucp => ConstCStr(b"uucp\0"),
			local0 => ConstCStr(b"local0\0"),
			local1 => ConstCStr(b"local1\0"),
			local2 => ConstCStr(b"local2\0"),
			local3 => ConstCStr(b"local3\0"),
			local4 => ConstCStr(b"local4\0"),
			local5 => ConstCStr(b"local5\0"),
			local6 => ConstCStr(b"local6\0"),
			local7 => ConstCStr(b"local7\0"),
		}
	}
}
