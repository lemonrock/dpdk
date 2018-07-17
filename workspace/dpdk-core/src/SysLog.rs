// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Logging to syslog support.
pub struct SysLog;

impl SysLog
{
	/// Caught an unwind. Log it to to syslog.
	#[inline(always)]
	pub fn caught_unwind(panic_payload: &(Any + 'static + Send))
	{
		use self::LogicalCoreChoice::*;
		
		let logical_core_choice = match LogicalCoreChoice::current_logical_core()
		{
			Any => Self::to_c_string_robustly("Any"),
			Specific(logical_core) => Self::to_c_string_robustly(format!("{}", logical_core)),
		};
		
		let cause = Self::to_c_string_robustly(Self::panic_payload_to_cause(panic_payload));
		
		unsafe { syslog(LOG_ERR, const_cstr!("LogicalCore:%s:Cause:%s").as_ptr(), logical_core_choice, cause) }
	}
	
	#[inline(always)]
	pub(crate) fn exit_signalled(signal_number: Option<SignalNumber>)
	{
		match signal_number
		{
			None => unsafe { syslog(LOG_NOTICE, const_cstr!("ExitSignalled:Other").as_ptr()) },
			Some(signal_number) => unsafe { syslog(LOG_NOTICE, const_cstr!("ExitSignalled:%s").as_ptr(), strsignal(signal_number)) },
		}
	}
	
	/// What caused the panic?
	#[inline(always)]
	pub fn panic_payload_to_cause(panic_payload: &(Any + 'static + Send)) -> String
	{
		if panic_payload.is::<String>()
		{
			panic_payload.downcast_ref::<String>().unwrap().to_string()
		}
		else if panic_payload.is::<&str>()
		{
			panic_payload.downcast_ref::<&str>().unwrap().to_string()
		}
		else
		{
			"(unknown cause)".to_string()
		}
	}
	
	/// To C String robustly.
	#[inline(always)]
	pub fn to_c_string_robustly<T: Into<Vec<u8>>>(string: T) -> CString
	{
		#[inline(always)]
		fn substitute_for_bad_c_string() -> CString
		{
			CString::new("?").unwrap()
		}
		
		CString::new(string).unwrap_or_else(|_| substitute_for_bad_c_string())
	}
	
}
