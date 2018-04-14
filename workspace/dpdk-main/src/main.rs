// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_main;


use ::dpdk_main::configuration::ethernetPorts::EthernetPortConfigurations;
use ::dpdk_main::dpdk::logicalCores::discovery::NumaSockets;
use ::dpdk_main::dpdk::logicalCores::discovery::LogicalCoreUser;
use ::dpdk_main::MainLogic;
use ::dpdk_main::Networking;

//noinspection SpellCheckingInspection
fn main()
{
	MainLogic::main(|numa_sockets, ethernetPortConfigurations|
	{
		let networkings = startNetworking(&numa_sockets, ethernetPortConfigurations);
		mainLoop(networkings);
		0
	});
}

fn startNetworking(numa_sockets: &NumaSockets, ethernetPortConfigurations: &mut EthernetPortConfigurations) -> Vec<Networking>
{
	let mut onlyLogicalCoreUserCurrently = LogicalCoreUser::newForNonEthernetThreads(1);
	
	let mut nonEthernetLogicalCoreUsers = Vec::new();
	nonEthernetLogicalCoreUsers.push(&mut onlyLogicalCoreUserCurrently);
	
	Networking::startAllNetworking
	(
		&mut nonEthernetLogicalCoreUsers,
		numa_sockets,
		ethernetPortConfigurations
	)
}

//noinspection SpellCheckingInspection
fn mainLoop(networkings: Vec<Networking>)
{
	for networking in networkings
	{
		/*
			main loop()
				- use signalfd() to receive SIGUSR1, SIGUSR2, SIGINFO and perhaps SIGTERM / SIGHUP
				
				- sleep every 10ms in loop
				- set the atomicbool that kills all slaves (need to change logic to use a CountDownLatch or barrier - check out ?beam?)
				- gather statistics from all known ethernet ports
				- every 100ms - 1 min, rebalance RETA tables
				
		*/
		networking.makeStop();
	}
	
	/*
		Still to do:
		 	check all have actually stopped
			stop ethernet ports
			close ethernet ports
	*/
}
