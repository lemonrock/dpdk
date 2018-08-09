// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Calculates a Cyclic Redundancy Check (CRC) for an Ethernet's trailing Frame Check Sequence (FCS).
///
///Also known as the CRC-32 algorithm, and used in standards including ISO 3309 (HDLC), ANSI X3.66 (ADCCP), FIPS PUB 71, FED-STD-1003, ITU-T V.42, ISO/IEC/IEEE 802-3 (Ethernet), SATA, MPEG-2, PKZIP, Gzip, Bzip2, POSIX cksum, PNG and ZMODEM.
///
/// `data` must have a length which does not exeed 2^32 - 1.
#[inline(always)]
pub fn cyclic_redundancy_check_32(data: &[u8]) -> u32
{
	debug_assert!(data.len() <= ::std::u32::MAX as usize, "data.len() '{}' exceeds ::std::u32::MAX '{}'", data.len(), ::std::u32::MAX);
	
	unsafe { rte_net_crc_calc(data.as_ptr() as *const _, data.len() as u32, rte_net_crc_type::RTE_NET_CRC32_ETH) }
}
