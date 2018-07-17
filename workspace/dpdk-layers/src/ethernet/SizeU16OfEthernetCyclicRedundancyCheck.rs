// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Size of Ethernet cyclic redundancy check (CRC) field that occurs after an ethernet packet.
///
/// Nearly all known hardware strips this field.
///
/// Also known as Frame Check Sequence (FCS).
pub const SizeU16OfEthernetCyclicRedundancyCheck: u16 = 4;
