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
	let programArguments = P::parseThenDisplayHelpOrVersionAndExitIfSoRequestedThenConfigureLogging();
	
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
}
