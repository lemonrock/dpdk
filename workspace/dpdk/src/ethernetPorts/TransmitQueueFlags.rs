// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk; including this file; may be copied; modified; propagated; or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub struct TransmitQueueFlags: u32
	{
		const NoMultiSegments = ETH_TXQ_FLAGS_NOMULTSEGS;
		const NoReferenceCounting = ETH_TXQ_FLAGS_NOREFCOUNT;
		const NoMultiMempool = ETH_TXQ_FLAGS_NOMULTMEMP;
		const NoVirtualLanOffload = ETH_TXQ_FLAGS_NOVLANOFFL;
		const NoTransmitSumSctp = ETH_TXQ_FLAGS_NOXSUMSCTP;
		const NoTransmitSumUdp = ETH_TXQ_FLAGS_NOXSUMUDP;
		const NoTransmitSumTcp = ETH_TXQ_FLAGS_NOXSUMTCP;

		const NoOffloads = ETH_TXQ_FLAGS_NOOFFLOADS;

		const NoTransmitSums = ETH_TXQ_FLAGS_NOXSUMS;
	}
}

impl Default for TransmitQueueFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
