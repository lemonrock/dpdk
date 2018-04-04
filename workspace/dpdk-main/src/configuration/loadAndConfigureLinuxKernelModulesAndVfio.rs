// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))]
pub fn loadAndConfigureLinuxKernelModules(configuration: &Configuration, finishers: &mut Finishers)
{
	let usesVfioPciKernelModule = loadAnyLinuxKernelModulesAndReturnTrueIfVfioPciKernelModuleIsUsed(configuration, finishers);
	if usesVfioPciKernelModule
	{
		ensureCorrectSettingsForVfioPciKernelModule(configuration.devPath());
	}
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn loadAnyLinuxKernelModulesAndReturnTrueIfVfioPciKernelModuleIsUsed(configuration: &Configuration, finishers: &mut Finishers) -> bool
{
	let (loadModulesFromPath, dpdkModules, usesVfioPciKernelModule) = configuration.linuxModules();
	LinuxKernelModule::loadAbsentModules(configuration.procPath(), loadModulesFromPath, dpdkModules, finishers);
	usesVfioPciKernelModule
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn ensureCorrectSettingsForVfioPciKernelModule(devPath: &Path)
{
	verifyMemLockResourceLimitForVfio();
	ensureVfioDeviceAccessible(&devPath);
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn ensureVfioDeviceAccessible(devPath: &Path)
{
	let mut devVfioPath = PathBuf::from(devPath);
	devVfioPath.push("vfio");
	
	match makeFolderReadableAndExecutable(&devVfioPath)
	{
		Err(error) => warn!("Could not change permissions on '{:?}' because '{}'", devVfioPath, error),
		Ok(_) => (),
	};
	
	let mut devVfioVfioPath = PathBuf::from(devVfioPath);
	devVfioVfioPath.push("vfio");
	if !devVfioVfioPath.exists()
	{
		panic!("Path '{:?}' does not exist", devVfioVfioPath);
	}
	
	match read_dir(&devVfioVfioPath)
	{
		Err(_) => panic!("Could not read directory entries for {:?}!", devVfioVfioPath),
		Ok(readDirectory) =>
		{
			for entry in readDirectory
			{
				match entry
				{
					Ok(entry) =>
					{
						let path = &entry.path();
						match makeFileReadWriteAll(path)
						{
							Err(error) => warn!("Could not change permissions on '{:?}' because '{}'", path, error),
							Ok(_) => (),
						}
					},
					Err(error) => warn!("Could not access directory entry for '{:?}' because '{}'", &devVfioVfioPath, error)
				};
			}
		}
	}
	
	// Additionally, the BIOS/EFI and kernel must have IO virtualization enabled, eg Intel VT-d
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn verifyMemLockResourceLimitForVfio()
{
	const _64MegaBytesInKiloBytes: u64 = 65_536;

	let softAndHardLimit = ResourceName::MaximumNumberOfBytesThatProcessCanMemLock.get();
	if softAndHardLimit.hardLimit().value() < _64MegaBytesInKiloBytes
	{
		warn!("MemLock is limited to less than 64Mb; VFIO may not be able to initialize (check `ulimit -l`)");
	}
}
