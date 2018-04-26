// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EthernetPortIdentifier(u16);

impl EthernetPortIdentifier
{
	//noinspection SpellCheckingInspection
	/// Returns an `Err(())` if the `ethernet_port_identifier` is greater than or equal to `RTE_MAX_ETHPORTS`, currently `32`.
	#[inline(always)]
	pub fn new(ethernet_port_identifier: u16) -> Result<Self, ()>
	{
		if (ethernet_port_identifier as usize) >= RTE_MAX_ETHPORTS
		{
			Err(())
		}
		else
		{
			Ok(EthernetPortIdentifier(ethernet_port_identifier))
		}
	}
	
	#[inline(always)]
	pub(crate) fn ethernet_device(self) -> &'static mut rte_eth_dev
	{
		unsafe { &mut rte_eth_devices[self.0 as usize] }
	}
}
