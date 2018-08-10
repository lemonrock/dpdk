// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/*
	METER is interesting. It applied a 'color' to an (IP) packet (optionally taking into account the DSCP / DiffServ field - we pass a table of 64 colors, one for each diffserv value). It can then (with the policer) drop the packet or adjust the color.
	
	Finally, color is put in 'rte_mbuf.sched.color' according to the docs. This doesn't exist.
	There is rte_mbuf.hash.sched.as_ref().lo or .hi
	
	The initial table can be updated over the life of the meter, allowing dynamic policy changes
	
	There is also traffic management: rte_tm_level_capabilities_get et al.
	
	The referenced meter identifier needs to have already been created...
*/
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct Meter
{
	pub identifier: u32,
}
