// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


fn print_version(name: &str, version: String, copyright_string: &str, licence_text: &str)
{
	println!("{} {}", name, version);
	println!("{} ({})", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_HOMEPAGE"));
	println!("{}", copyright_string);
	println!("{}", licence_text);
	println!();
	println!("Written by {}.", authors())
}
