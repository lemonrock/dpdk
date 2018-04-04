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
	
		let (sysPath, numaSockets, mut ethernetPortConfigurations) = initialise::<SampleConfigurationAndProgramArguments>(&mut finishers);
		
		let result = catch_unwind(AssertUnwindSafe(||
		{
			usefulMain(&numaSockets, &mut ethernetPortConfigurations)
		}));

		finishers.finish(sysPath);
		
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
	setCurrentThreadNameTo("Master");
	blockNearlyAllSignals();
	let programArguments = P::parseThenDisplayHelpOrVersionAndExitIfSoRequestedThenConfigureLogging();
	checkWeAreRoot();
	startANewProcessGroup();
	forkAndStartANewSession();
	blockAllSignalsBarChild();
	restrictUmaskToCurrentUser();
	
	let configuration = programArguments.configurationAsModifiedByCommandLine();
	configuration.changeResourceLimits();
	configuration.loadAndConfigureLinuxKernelModules(&mut finishers);
	let hugePageFilePathInformation = configuration.setUpHugePagesAndNumaMemory(&mut finishers);
	
	let configurations =
	{
		let (dpdkRteInitData, configurations) = configuration.dpdkRteInitData(&mut finishers);
		dpdkRteInitData.initialiseDpdk(configuration.borrowNumaSockets(), hugePageFilePathInformation);
		configurations
	};
	let sysPathBuf = configuration.sysPath().to_path_buf();
	let numaSockets = configuration.destroyAsNumaSockets();
	
	lockDownCapabilitiesOnLinux();
	
	(sysPathBuf, numaSockets, configurations)
}

fn checkWeAreRoot()
{
	assertEffectiveUserIsRoot("Initialisation");
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
	disableDumpable();
	noNewPrivileges();
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
	
	Capability::ensureDropped(&CapabilitiesToDrop);
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn removeNearlyAllCapabilitiesOnLinux()
{
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn maximallyRestrictCapabilities()
{
	Capability::ensureDropped(&[Capability::KernelModule]);
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn setCurrentThreadNameTo(value: &str)
{
	setCurrentThreadName(value).expect(&format!("Could not set thread name to '{}'", value.to_owned()));
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn setCurrentThreadNameTo(value: &str)
{
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn lockDownCapabilitiesOnLinux()
{
	maximallyRestrictCapabilities();
	Capability::clearAllAmbientCapabilities();
	lockSecureBitsAndRemoveAmbientCapabilityRaiseAndKeepCaps();
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn lockDownCapabilitiesOnLinux()
{
}
