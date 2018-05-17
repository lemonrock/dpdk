// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


fn authors() -> String
{
	const Authors: &'static str = env!("CARGO_PKG_AUTHORS");
	
	let mut splitter = Authors.split(':').peekable();
	
	let mut authors = String::with_capacity(Authors.len() * 2);
	
	authors.push_str(splitter.next().unwrap());
	loop
	{
		let author = match splitter.next()
		{
			None => break,
			Some(author) => author,
		};
		
		let is_last = splitter.peek().is_none();
		let push = if is_last
		{
			" and "
		}
		else
		{
			", "
		};
		authors.push_str(push);
		authors.push_str(author);
	}
	
	authors
}
