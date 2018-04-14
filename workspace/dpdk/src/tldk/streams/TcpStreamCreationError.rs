// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// Cause of error when TcpStream new fails.
	#[derive(Debug)]
	pub enum TcpStreamCreationError
	{
		/// Stream creation failed but can be tried again later.
		TryCreationAgain(cause: StreamCreationError)
		{
			description(cause.description())
			display("Stream creation failed but can be tried again later: {}", cause.description())
			cause(cause)
			from()
		}
	}
}
