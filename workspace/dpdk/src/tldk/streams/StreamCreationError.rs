// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// Cause of error when Stream new fails.
	#[derive(Debug)]
	pub enum StreamCreationError
	{
		/// There is outstanding data to send or receive on the existing stream before it can be used.
		///
		/// Try again later.
		BecauseThereIsOutstandingDataToSendOrReceiveOnTheStreamBeforeIsCanBeUsed
		{
			description("Try again later because there is outstanding data to send or receive on the existing stream before it can be used")
			display("Try again later because there is outstanding data to send or receive on the existing stream before it can be used")
		}
		
		/// The limit on the number of streams has been reached.
		///
		/// Similar to out-of-memory but recoverable.
		NoMoreStreamsAvailable
		{
			description("No more streams available")
			display("No more streams available")
		}
	}
}
