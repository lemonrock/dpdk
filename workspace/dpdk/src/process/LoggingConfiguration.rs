// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Logging configuration.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct LoggingConfiguration
{
	/// Defaults to `auth`.
	pub syslog_facility: DpdkSyslogFacility,
	
	/// Defaults to `debug` for debug builds and `warning` for production builds.
	///
	/// DPDK also supports specifying either a regex or a pattern; this is not supported by `LoggingConfiguration` at this time.
	pub syslog_priority: DpdkSyslogPriority,

	/// Up to 31 bytes (more are ignored) identifying the source of log messages.
	///
	/// Defaults to program name.
	pub identity: String,

	/// When a panic occurs that isn't caught (or if the error-chain crate is in use), capture a full stack back trace.
	pub enable_full_rust_stack_back_traces: bool,
}

impl Default for LoggingConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			syslog_facility: Default::default(),
			syslog_priority: Default::default(),
			identity: get_program_name(),
			enable_rust_backtraces: true,
		}
	}
}

impl LoggingConfiguration
{
	/// Issues a warning (currently to syslog).
	#[inline(always)]
	pub fn warn(name: &str, message: String)
	{
		let name = Self::to_c_string_robustly(name);
		let message = Self::to_c_string_robustly(message);
		unsafe { syslog(LOG_WARNING, const_cstr!("%s:%s").as_ptr(), name.as_ptr(), message.as_ptr()) };
	}
	
	#[inline(always)]
	fn caught_panic(source_file: &str, line_number: u32, column_number: u32, cause: &str)
	{
		let source_file = Self::to_c_string_robustly(source_file);
		let cause = Self::to_c_string_robustly(cause);
		unsafe { syslog(LOG_CRIT, const_cstr!("File:%s:Line:%u:Column:%u:Cause:%s").as_ptr(), source_file, line_number, column_number, cause) }
	}
	
	#[inline(always)]
	pub(crate) fn caught_unwind(panic_payload: &(Any + 'static + Send))
	{
		use self::LogicalCoreChoice::*;
		
		let logical_core_choice = match LogicalCoreChoice::current_logical_core()
		{
			Any => Self::to_c_string_robustly("Any"),
			Specific(logical_core) => Self::to_c_string_robustly(format!("{}", logical_core.into::<u16>())),
		};
		
		let cause = Self::to_c_string_robustly(Self::panic_payload_to_cause(panic_payload));
		
		unsafe { syslog(LOG_ERR, const_cstr!("LogicalCore:%s:Cause:%s").as_ptr(), logical_core_choice, cause) }
	}
	
	#[inline(always)]
	pub(crate) fn exit_signalled(&self, signal_number: Option<SignalNumber>)
	{
		match signal_number
		{
			None => unsafe { syslog(LOG_NOTICE, const_cstr!("ExitSignalled:Other").as_ptr()) },
			Some(signal_number) => unsafe { syslog(LOG_NOTICE, const_cstr!("ExitSignalled:%s").as_ptr(), unsafe { strsignal(signal_number) }) },
		}
		
	}
	
	#[inline(always)]
	pub(crate) fn configure_rust_stack_back_traces(&self)
	{
		let setting = if self.enable_full_rust_stack_back_traces
		{
			"1"
		}
		else
		{
			"0"
		};
		set_var("RUST_BACKTRACE", setting);
	}
	
	#[inline(always)]
	pub(crate) fn configure_syslog(&self, running_interactively_so_also_log_to_standard_error: bool)
	{
		unsafe { setlogmask(self.syslog_priority.log_upto()) };
		
		let mut log_options = LOG_PID | LOG_NDELAY;
		
		if running_interactively_so_also_log_to_standard_error
		{
			log_options |= LOG_PERROR;
		}
		
		let identity = CString::from(&self.identity).unwrap();
		unsafe { openlog(identity.as_ptr(), log_options, self.syslog_facility as i32) }
	}
	
	#[inline(always)]
	pub(crate) fn configure_panic_hook(&self)
	{
		set_panic_hook(Box::new(|panic_info|
		{
			let (source_file, line_number, column_number) = match panic_info.location()
			{
				None => ("(unknown source file)", 0, 0),
				Some(location) => (location.file(), location.line(), location.column())
			};
			
			let cause = Self::panic_payload_to_cause(panic_info.payload());
			
			Self::caught_panic(source_file, line_number, column_number, cause)
		}));
	}
	
	#[inline(always)]
	pub(crate) fn stop_panic_hook(&self)
	{
		drop(take_hook());
	}
	
	#[inline(always)]
	pub(crate) fn stop_logging(&self)
	{
		unsafe { closelog() }
	}
	
	#[inline(always)]
	fn panic_payload_to_cause(panic_payload: &(Any + 'static + Send)) -> &str
	{
		if payload.is::<String>()
		{
			payload.downcast_ref::<String>().unwrap().as_str()
		}
		else if payload.is::<&str>()
		{
			*payload.downcast_ref::<&str>().unwrap()
		}
		else
		{
			"(unknown cause)"
		}
	}
	
	#[inline(always)]
	fn to_c_string_robustly<T: Into<Vec<u8>>>(string: T) -> CString
	{
		CString::new(string).unwrap_or_else(|_| Self::substitute_for_bad_c_string())
	}
	
	#[inline(always)]
	fn substitute_for_bad_c_string() -> CString
	{
		CString::new("?").unwrap()
	}
}
