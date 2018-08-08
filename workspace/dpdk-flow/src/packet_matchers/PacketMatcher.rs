// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An abstraction of a DPDK flow item.
pub trait PacketMatcher
{
	/// DPDK related struct.
	type DpdkType: Sized;
	
	/// DPDK type.
	const Type: rte_flow_item_type;
	
	/// Is this a DPDK 'META' flow item type?
	const IsMeta: bool;
	
	/// DPDK static mask.
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType;
}
