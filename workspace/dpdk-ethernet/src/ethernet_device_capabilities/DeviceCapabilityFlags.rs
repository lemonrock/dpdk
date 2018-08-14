// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Generic device capability flags.
	#[derive(Deserialize, Serialize)]
	pub struct DeviceCapabilityFlags: u64
	{
		/// Receive queue setup is possible after the device has started (ie at runtime).
		const ReceiveQueueSetupPossibleAfterDeviceStarted = RTE_ETH_DEV_CAPA_RUNTIME_RX_QUEUE_SETUP as u64;
		
		/// Transmit queue setup is possible after the device has started (ie at runtime).
		const RuntimeTransmitQueueSetup = RTE_ETH_DEV_CAPA_RUNTIME_TX_QUEUE_SETUP as u64;
	}
}
