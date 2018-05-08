// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


trait VecArguments<'a>
{
	fn initialise() -> Self;
	
	fn argument(&mut self, key: ConstCStr);
	
	fn optionalArgument(&mut self, key: ConstCStr, onlyAddIfTrue: bool)
	{
		if onlyAddIfTrue
		{
			self.argument(key)
		}
	}
	
	fn keyPointerValue(&mut self, key: ConstCStr, value: *const c_char);
	
	fn keyConstantValue(&mut self, key: ConstCStr, value: ConstCStr)
	{
		self.keyPointerValue(key, value.as_ptr());
	}
	
	fn keyCStrValue(&mut self, key: ConstCStr, value: &'a CStr)
	{
		self.keyPointerValue(key, value.as_ptr());
	}
	
	fn keyBytesValue(&mut self, key: ConstCStr, value: &'a [u8])
	{
		self.keyPointerValue(key, value.as_ptr() as *const c_char);
	}
}

impl<'a> VecArguments<'a> for Vec<*const c_char>
{
	fn initialise() -> Self
	{
		Vec::with_capacity(128)
	}
	
	fn argument(&mut self, key: ConstCStr)
	{
		self.push(key.as_ptr())
	}
	
	fn keyPointerValue(&mut self, key: ConstCStr, value: *const c_char)
	{
		self.argument(key);
		self.push(value);
	}
}
