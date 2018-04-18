// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Wraps the DPDK PCI bus logic.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkPciBus;

impl PrintAllInformation for DpdkPciBus
{
	#[inline(always)]
	fn print_information_to_stream(stream: *mut FILE)
	{
		unsafe { rte_pci_dump(stream) };
	}
}

impl DpdkPciBus
{
	/// Does this process have access to the PCI bus?
	#[inline(always)]
	pub fn configured_with_use_of_pci_bus() -> bool
	{
		(unsafe { rte_eal_has_pci() }) != 0
	}
}
