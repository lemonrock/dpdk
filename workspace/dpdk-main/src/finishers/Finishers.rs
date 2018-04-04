// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Finishers
{
	finishers: Vec<Box<Finisher>>
}

impl Default for Finishers
{
	#[inline(always)]
	fn default() -> Self
	{
		Finishers
		{
			finishers: Vec::with_capacity(4)
		}
	}
}

impl Finishers
{
	#[inline(always)]
	pub fn push(&mut self, finisher: Box<Finisher>)
	{
		self.finishers.push(finisher);
	}
	
	#[inline(always)]
	pub fn finish(&mut self, sysPath: PathBuf)
	{
		self.finishers.reverse();
		for finisher in self.finishers.drain(..)
		{
			finisher.finish(&sysPath);
		}
	}
}
