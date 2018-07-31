// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// TODO: Consider handling `SIGINFO`.


/// An object that can be used with a configuration file (eg via Serde) to configure a daemon.
///
/// The following are done:-
///
/// * umask is set to just the current user
/// * Checks are made to check the program is not running with the set uid bit set ('setuid' or 'suid').
/// * A PID file is created
/// * standard in is redirected to `/dev/null`.
/// * standard out and error are redirected to `/dev/null`.
/// * `fprintf` and friends using the `FILE` API are redirected to syslog on Linux (this is probably also possible to implement for FreeBSD - see <https://mischasan.wordpress.com/2011/05/25/redirecting-stderr-to-syslog/>).
/// * Double forking and a new session are created.
/// * Real and effective user and group ids are changed.
/// * Additional groups from `/etc/group`, if any, are assigned.
/// * Environment variables are populated if missing (`IFS`, `PATH`)
/// * User environment variables are overwritten (`HOME`, `LOGNAME`, `USER`).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct Daemonize
{
	/// The folder path to use as the 'current working directory' (CWD).
	///
	/// Equivalent functionality to the shell command `chdir`.
	///
	/// Defaults to `/`.
	#[serde(default = "Daemonize::working_folder_path_default")] pub working_folder_path: PathBuf,
	
	/// A folder path in which to put a PID file.
	///
	/// This uses the processes' name for the actual file base name.
	///
	/// Defaults to `/var/run`.
	#[serde(default = "Daemonize::pid_folder_path_default")] pub pid_folder_path: PathBuf,
	
	/// An user name that must exist in `/etc/passwd` (or the local equivalent).
	///
	/// Use to discover runtime user and groups to change to and the home folder of the running user.
	#[serde(default = "Daemonize::user_name_default")] pub user_name: CString,
}

impl Default for Daemonize
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			working_folder_path: Self::working_folder_path_default(),
			pid_folder_path: Self::pid_folder_path_default(),
			user_name: Self::user_name_default(),
		}
	}
}

impl Daemonize
{
	/// Daemonizes the current process.
	///
	/// Returns an object that needs to have `clean_up()` called on it just before process exit.
	#[inline(always)]
	pub fn daemonize(self) -> DaemonizeCleanUpOnExit
	{
		Self::verify_not_running_with_set_uid_bit_set();
		
		Self::initial_umask();
		
		let pid_file_path = self.switch_user();
		
		self.change_current_working_directory();
		
		self.redirect_standard_in_out_and_error();
		
		Self::fork();
		
		Self::create_a_new_progress_group_and_session_detach_controlling_terminal();
		
		Self::fork();
		
		self.populate_pid_file_when_running(&pid_file_path);
		
		Self::ensure_environment_variable_is_set("IFS", "\t\n");
		Self::ensure_environment_variable_is_set("PATH", "/usr/local/bin:/usr/bin");
		
		DaemonizeCleanUpOnExit
		{
			pid_file_path
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn verify_not_running_with_set_uid_bit_set()
	{
		assert_eq!(unsafe { geteuid() }, unsafe { getuid() }, "Can not be run with set uid bit set ('setuid')");
	}
	
	#[inline(always)]
	fn initial_umask()
	{
		unsafe { umask(0o0000) };
	}
	
	#[inline(always)]
	fn get_user_entry(&self) -> NonNull<passwd>
	{
		let entry = unsafe { getpwnam(self.user_name.as_ptr()) };
		assert!(!entry.is_null(), "user name '{:?} does not exist in /etc/passwd", &self.user_name);
		unsafe { NonNull::new_unchecked(entry) }
	}
	
	#[inline(always)]
	fn switch_user(&self) -> PathBuf
	{
		Self::guard_we_are_root();
		
		let entry = self.get_user_entry();
		
		let (uid, gid, user_name, home_folder_path) =
		{
			let entry = unsafe { entry.as_ref() };
			(
				entry.pw_uid,
				entry.pw_gid,
				NonNull::new(entry.pw_name).expect("pw_name was null"),
				NonNull::new(entry.pw_dir).expect("pw_dir was null"),
			)
		};
		
		let pid_file_path = self.create_pid_file_before_switching_user(uid, gid);
		
		assert_eq!(unsafe { setgid(gid) }, 0, "Could not set group identifier to '{}' because '{}'", gid, Self::os_error());
		#[cfg(not(any(target_os = "ios", target_os = "macos")))] assert_eq!(unsafe { initgroups(user_name.as_ptr(), gid) }, 0, "Could not initialize additional groups for '{}' because '{}'", gid, Self::os_error());
		#[cfg(any(target_os = "ios", target_os = "macos"))] assert_eq!(unsafe { initgroups(user_name.as_ptr(), gid as i32) }, 0, "Could not initialize additional groups for '{}' because '{}'", gid, Self::os_error());
		
		Self::restrict_umask_to_current_user();
		
		assert_eq!(unsafe { setegid(gid) }, 0, "Could not set effective group id to '{}' because '{}", gid, Self::os_error());
		assert_eq!(unsafe { setuid(uid) }, 0, "Could not set user id to '{}' because '{}", uid, Self::os_error());
		assert_eq!(unsafe { seteuid(uid) }, 0, "Could not set effective user id to '{}' because '{}", uid, Self::os_error());
		
		Self::make_environment_variables_match_user(user_name, home_folder_path);
		
		pid_file_path
	}
	
	#[inline(always)]
	fn guard_we_are_root()
	{
		assert_effective_user_id_is_root("Changing user in daemonize()");
	}
	
	#[inline(always)]
	fn restrict_umask_to_current_user()
	{
		unsafe { umask(0o0077) };
	}
	
	#[inline(always)]
	fn create_pid_file_before_switching_user(&self, uid: uid_t, gid: gid_t) -> PathBuf
	{
		let pid_file_path = self.pid_file_path();
		let pid_file_path_string = pid_file_path.to_c_string();
		
		let file_descriptor = unsafe { open(pid_file_path_string.as_ptr(), O_CREAT | O_WRONLY, (S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) as u32) };
		assert!(file_descriptor >= 0, "Could not create PID file '{:?}' because '{}'", &pid_file_path_string, Self::os_error());
		assert_eq!(unsafe { fchown(file_descriptor, uid, gid) }, 0, "Could not change ownership of PID file '{:?}' because '{}'", &pid_file_path_string, Self::os_error());
		unsafe { close(file_descriptor) };
		
		pid_file_path
	}
	
	#[inline(always)]
	fn populate_pid_file_when_running(&self, pid_file_path: &Path)
	{
		pid_file_path.write_value(process::id()).unwrap();
	}
	
	#[inline(always)]
	fn make_environment_variables_match_user(user_name: NonNull<c_char>, home_folder_path: NonNull<c_char>)
	{
		const_cstr!
		{
			USER = "USER";
			LOGNAME = "LOGNAME";
			HOME = "HOME";
		}
		Self::set_environment_variable(USER, user_name);
		Self::set_environment_variable(LOGNAME, user_name);
		Self::set_environment_variable(HOME, home_folder_path);;
	}
	
	#[inline(always)]
	fn change_current_working_directory(&self)
	{
		let c_string = self.working_folder_path.to_c_string();
		assert_eq!(unsafe { chdir(c_string.as_ptr()) }, 0, "Could not change current working directory to '{:?}' because '{}'", c_string, Self::os_error());
	}
	
	#[inline(always)]
	fn create_a_new_progress_group_and_session_detach_controlling_terminal()
	{
		assert!(unsafe { setsid() } >= 0, "setsid failed because '{}'", Self::os_error());
	}
	
	#[inline(always)]
	fn fork()
	{
		const ForkedToChild: i32 = 0;
		
		match unsafe { fork() }
		{
			ForkedToChild => (),
			-1 => panic!("Fork failed with {}", Self::os_error()),
			_child_process_id_returned_to_parent @ _ => process::exit(0),
		}
	}
	
	#[inline(always)]
	fn redirect_standard_in_out_and_error(&self)
	{
		Self::redirect_to_dev_null(&io::stdin());
		Self::redirect_to_dev_null(&io::stdout());
		Self::redirect_to_dev_null(&io::stderr());
		
		#[cfg(target_os = "linux")]
		{
			#[inline(always)]
			fn write_to_syslog(priority: c_int, data: *const c_char, length: size_t) -> ssize_t
			{
				const_cstr!
				{
					SyslogFormat = "%s:%s";
				}
				
				unsafe { syslog(priority, SyslogFormat.as_ptr(), length, program_invocation_short_name, data) };
				length as ssize_t
			}
			
			unsafe extern "C" fn write_standard_out_to_syslog(_cookie: *mut c_void, data: *const c_char, length: size_t) -> ssize_t
			{
				write_to_syslog(LOG_NOTICE, data, length)
			}
			
			unsafe extern "C" fn write_standard_error_to_syslog(_cookie: *mut c_void, data: *const c_char, length: size_t) -> ssize_t
			{
				write_to_syslog(LOG_ERR, data, length)
			}
			
			#[inline(always)]
			fn redirect_to_syslog(original: &mut *const FILE, callback: cookie_write_function_t)
			{
				let mut functions = cookie_io_functions_t::default();
				functions.write = callback;
				
				const_cstr!
				{
					w = "w";
				}
				
				let file = unsafe { fopencookie(null_mut(), w.as_ptr(), functions) };
				assert!(!file.is_null(), "file is null from fopencookie");
				*original = file;
				unsafe { setvbuf(*original as *mut _, null_mut(), _IOLBF, 0) };
			}
			
			redirect_to_syslog(unsafe { &mut stdout }, write_standard_out_to_syslog);
			redirect_to_syslog(unsafe { &mut stderr }, write_standard_error_to_syslog);
		}
	}
	
	#[inline(always)]
	fn redirect_to_dev_null<A: AsRawFd>(a: &A)
	{
		const_cstr!
		{
			DevNull = "/dev/null";
		}
		
		let file_descriptor = a.as_raw_fd();
		let null_file_descriptor = unsafe { open(DevNull.as_ptr(), O_WRONLY) };
		assert!(null_file_descriptor >= 0, "Could not open /dev/null because '{}'", Self::os_error());
		assert_eq!(unsafe { dup2(null_file_descriptor, file_descriptor)}, 0, "Could not dup2 because '{}'", Self::os_error());
		assert_eq!(unsafe { close(null_file_descriptor) }, 0, "Could not close null file descriptor because '{}'", Self::os_error());
	}
	
	#[inline(always)]
	fn pid_file_path(&self) -> PathBuf
	{
		self.pid_folder_path.join(PathBuf::from(format!("{}.pid", get_program_name())))
	}
	
	#[inline(always)]
	fn ensure_environment_variable_is_set(name: &str, value: &str)
	{
		if var_os(name).is_none()
		{
			set_var(name, value)
		}
	}
	
	// NOTE: This does not use the Rust functions, as we do not want to convert from a libc-supplied string to an OsString.
	#[inline(always)]
	fn set_environment_variable(name: ConstCStr, value: NonNull<c_char>)
	{
		const Overwrite: i32 = 1;
		
		assert_eq!(unsafe { setenv(name.as_ptr(), value.as_ptr(), Overwrite) }, 0, "Could not set environment variable '{:?}' because '{}", name, Self::os_error());
	}
	
	#[inline(always)]
	fn os_error() -> io::Error
	{
		io::Error::last_os_error()
	}
	
	#[inline(always)]
	fn pid_folder_path_default() -> PathBuf
	{
		PathBuf::from("/var/run")
	}
	
	#[inline(always)]
	fn working_folder_path_default() -> PathBuf
	{
		PathBuf::from("/")
	}
	
	#[inline(always)]
	fn user_name_default() -> CString
	{
		CString::new("root").unwrap()
	}
}
