// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a DPDK syslog facility.
///
/// Defaults to `auth`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum DpdkSyslogFacility
{
	auth,
	cron,
	daemon,
	ftp,
	kern,
	lpr,
	mail,
	news,
	syslog,
	user,
	uucp,
	local0,
	local1,
	local2,
	local3,
	local4,
	local5,
	local6,
	local7,
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
	pub(crate) fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::DpdkSyslogFacility::*;
		
		match self
		{
			auth => const_cstr!("auth"),
			cron => const_cstr!("cron"),
			daemon => const_cstr!("daemon"),
			ftp => const_cstr!("ftp"),
			kern => const_cstr!("kern"),
			lpr => const_cstr!("lpr"),
			mail => const_cstr!("mail"),
			news => const_cstr!("news"),
			syslog => const_cstr!("syslog"),
			user => const_cstr!("user"),
			uucp => const_cstr!("uucp"),
			local0 => const_cstr!("local0"),
			local1 => const_cstr!("local1"),
			local2 => const_cstr!("local2"),
			local3 => const_cstr!("local3"),
			local4 => const_cstr!("local4"),
			local5 => const_cstr!("local5"),
			local6 => const_cstr!("local6"),
			local7 => const_cstr!("local7"),
		}
	}
}
