// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// What caused the panic?
#[inline(always)]
pub fn panic_payload_to_cause(panic_payload: &(Any + 'static + Send)) -> String
{
	if panic_payload.is::<String>()
	{
		panic_payload.downcast_ref::<String>().unwrap().to_string()
	}
	else if panic_payload.is::<&str>()
	{
		panic_payload.downcast_ref::<&str>().unwrap().to_string()
	}
	else
	{
		"(unknown cause)".to_string()
	}
}
