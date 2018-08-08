// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Calculates a Cyclic Redundancy Check (CRC) using the CRC-16-CCITT (Consultative Committee for International Telephony and Telegraphy) algorithm.
///
/// Also known as the CRC-CITT algorithm, and used in standards including X.25, V.41, HDLC FCS, XMODEM, Bluetooth, PACTOR, SD and DigRF.
///
/// `data` must have a length which does not exeeds 2^32 - 1.
#[inline(always)]
pub fn cyclic_redundancy_check_16_CCITT(data: &[u8]) -> u16
{
	debug_assert!(data.len() <= ::std::u32::MAX as usize, "data.len() '{}' exceeds ::std::u32::MAX '{}'", data.len(), ::std::u32::MAX);
	
	let result = unsafe { rte_net_crc_calc(data.as_ptr() as *const _, data.len() as u32, rte_net_crc_type::RTE_NET_CRC16_CCITT) };
	result as u16
}
