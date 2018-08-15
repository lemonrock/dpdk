// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Generic device information flags.
	#[derive(Deserialize, Serialize)]
	pub struct DeviceInformationFlags: u32
	{
		/// Supports link status interrupt ('lsc').
		const SupportsLinkStatusInterrupt = RTE_ETH_DEV_INTR_LSC;
		
		/// Is a bonded slave, ie should not be used directly.
		const IsABondedSlave = RTE_ETH_DEV_BONDED_SLAVE;
		
		/// Supports removal ('rmv').
		const SupportsLinkStatusInterrupt = RTE_ETH_DEV_INTR_RMV;
		
		/// Is a switch port representor.
		const IsASwitchPortRepresentor = RTE_ETH_DEV_REPRESENTOR;
	}
}
