// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Trait to handle ethernet port link status events.
pub trait LinkStatusEventHandler
{
	/// Link has come up.
	///
	/// Unlikely to be thread safe or to be called consistently from the same thread or core; likely to be invoked on a service core.
	///
	/// Can be used to do ARP probing, Neighbour discovery, DHCP, LLDP or other 'link up' activities.
	///
	/// After having done probing, etc, could then unlock a thread barrier for general receive / transmit threads.
	#[inline(always)]
	fn link_has_come_up(&mut self, ethernet_port_identifier: EthernetPortIdentifier, is_full_duplex: bool, was_auto_negotiated: bool, speed_in_megabits_per_second: u32);
	
	/// Link has gone down.
	///
	/// Unlikely to be thread safe or to be called consistently from the same thread or core; likely to be invoked on a service core.
	///
	/// Can be used to pause or sleep; could be used on a thread barrier.
	#[inline(always)]
	fn link_has_gone_down(&mut self, ethernet_port_identifier: EthernetPortIdentifier);
}
