// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MainLogic
{
}

impl MainLogic
{
	#[inline(always)]
	pub fn main<F>(mut usefulMain: F) -> !
	where F: FnMut(&NumaSockets, &mut EthernetPortConfigurations) -> i32
	{
		let mut finishers = Default::default();
	
		let (sys_path, numa_sockets, mut ethernetPortConfigurations) = initialise::<SampleConfigurationAndProgramArguments>(&mut finishers);
		
		let result = catch_unwind(AssertUnwindSafe(||
		{
			usefulMain(&numa_sockets, &mut ethernetPortConfigurations)
		}));

		finishers.finish(sys_path);
		
		match result
		{
			Ok(exitCode) => exit(exitCode),
			Err(payload) => resume_unwind(payload),
		}
	}
}

fn initialise<P: ConfigurationAndProgramArguments>(mut finishers: &mut Finishers) -> (PathBuf, NumaSockets, EthernetPortConfigurations)
{
	initialUmask();
	removeNearlyAllCapabilitiesOnLinux();
	additionalEarlyProcessSecurityTighteningOnLinux();
	set_current_thread_nameTo("Master");
	block_all_signals_on_current_thread_bar_hang_up_and_terminate_and_child();
	let programArguments = P::parseThenDisplayHelpOrVersionAndExitIfSoRequestedThenConfigureLogging();
	checkWeAreRoot();
	startANewProcessGroup();
	forkAndStartANewSession();
	block_all_signals_on_current_thread_bar_child();
	restrictUmaskToCurrentUser();
	
	let timer_progress_engine = TimerProgressEngine::initialize(Cycles::AroundTenMillisecondsAt2GigaHertzSuitableForATimerProgressEngine);
	
	let configuration = programArguments.configurationAsModifiedByCommandLine();
	configuration.changeResourceLimits();
	configuration.loadAndConfigureLinuxKernelModules(&mut finishers);
	let hugePageFilePathInformation = configuration.setUpHugePagesAndNumaMemory(&mut finishers);
	
	let configurations =
	{
		let (dpdkRteInitData, configurations) = configuration.dpdkRteInitData(&mut finishers);
		dpdkRteInitData.initialize_dpdk(configuration.borrowNumaSockets(), hugePageFilePathInformation);
		configurations
	};
	let sys_pathBuf = configuration.sys_path().to_path_buf();
	let numa_sockets = configuration.destroyAsNumaSockets();
	
	lockDownCapabilitiesOnLinux();
	
	(sys_pathBuf, numa_sockets, configurations)
}

fn checkWeAreRoot()
{
	assert_effective_user_id_is_root("Initialisation");
}

fn initialUmask()
{
	unsafe { umask(0o0000) };
}

fn restrictUmaskToCurrentUser()
{
	unsafe { umask(0o0077) };
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn additionalEarlyProcessSecurityTighteningOnLinux()
{
	disable_dumpable();
	no_new_privileges();
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn additionalEarlyProcessSecurityTighteningOnLinux()
{
}

fn startANewProcessGroup()
{
	let result = unsafe { setpgid(0, 0) };
	if likely(result == 0)
	{
		return;
	}
	match result
	{
		-1 => panic!("Could not setpgid"),
		_ => panic!("Positive value from setpgid"),
	}
}

fn forkAndStartANewSession()
{
	warn!("Implement forkAndStartANewSession via fork() and setsid()");
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn removeNearlyAllCapabilitiesOnLinux()
{
	const CapabilitiesToDrop: [Capability; 34] =
	[
		Capability::AuditControl,
		Capability::AuditRead,
		Capability::AuditWrite,
		Capability::BlockSuspend,
		Capability::Chown,
		Capability::DiscretionaryAccessControlBypass,
		Capability::DiscretionaryAccessControlFileReadBypass,
		Capability::FileOwnerBypass,
		Capability::FileSetId,
		//Capability::LockMemory,
		Capability::IpcOwner,
		Capability::Kill,
		Capability::Lease,
		Capability::Immutable,
		Capability::MandatoryAccessControlBypass,
		Capability::MandatoryAccessControlOverride,
		Capability::MakeNodes,
		Capability::SystemAdministration,
		Capability::NetworkAdministration,
		Capability::BindPortsBelow1024,
		//Capability::NetRaw,
		Capability::SetUid,
		Capability::SetGid,
		Capability::SetFileCapabilities,
		Capability::SetProcessCapabilities,
		Capability::RebootAndKexecLoad,
		Capability::Chroot,
		//Capability::KernelModule,
		Capability::Nice,
		Capability::ProcessAccounting,
		Capability::PTrace,
		Capability::RawIO,
		Capability::Resource,
		Capability::Time,
		Capability::TtyConfig,
		Capability::Syslog,
		Capability::WakeAlarm,
	];
	
	Capability::ensure_capabilities_dropped(&CapabilitiesToDrop);
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn removeNearlyAllCapabilitiesOnLinux()
{
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn maximallyRestrictCapabilities()
{
	Capability::ensure_capabilities_dropped(&[Capability::KernelModule]);
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn set_current_thread_nameTo(value: &str)
{
	set_current_thread_name(value).expect(&format!("Could not set thread name to '{}'", value.to_owned()));
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn set_current_thread_nameTo(value: &str)
{
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn lockDownCapabilitiesOnLinux()
{
	maximallyRestrictCapabilities();
	Capability::clear_all_ambient_capabilities();
	lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn lockDownCapabilitiesOnLinux()
{
}
