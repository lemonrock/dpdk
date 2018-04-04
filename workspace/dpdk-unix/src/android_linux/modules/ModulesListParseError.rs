// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum ModulesListParseError
	{
		CouldNotOpenFile(cause: Error)
		{
			cause(cause)
			from()
		}
		
		CouldNotParseEmptyModuleName(lineCount: usize)
		{
			display("Zero-based line '{}' could not be parsed as a module name because it is empty", lineCount)
		}
		
		DuplicateModuleName(lineCount: usize, moduleName: String)
		{
			display("Zero-based line '{}' module name '{:?}' was duplicated", lineCount, moduleName)
		}
	}
}
