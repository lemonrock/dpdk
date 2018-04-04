// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


struct CircularIterator<'a, D: 'a>
{
	length: usize,
	lastIndex: usize,
	nextIndex: usize,
	data: &'a mut [&'a mut D],
}

impl<'a, D> CircularIterator<'a, D>
{
	#[inline(always)]
	pub fn new(data: &'a mut [&'a mut D]) -> Self
	{
		let length = data.len();
		
		CircularIterator
		{
			length: length,
			lastIndex: if length == 0
			{
				0
			}
			else
			{
				length - 1
			},
			nextIndex: 0,
			data: data,
		}
	}
	
	#[inline(always)]
	pub fn iter_mut<F>(&mut self, mut callback: F)
	where F: FnMut(&mut D) -> bool
	{
		let length = self.length;
		let lastIndex = self.lastIndex;
		let mut nextIndex;
		
		let mut count = 0;
		let ref mut data = self.data;
		while count < length
		{
			nextIndex = self.nextIndex;
			
			if nextIndex == lastIndex
			{
				nextIndex = 0;
			}
			else
			{
				nextIndex += 1;
			}

			let datum = unsafe { data.get_unchecked_mut(nextIndex) };
			if callback(datum)
			{
				self.nextIndex = nextIndex;
				break;
			}
			
			count += 1;
		}
	}
}
