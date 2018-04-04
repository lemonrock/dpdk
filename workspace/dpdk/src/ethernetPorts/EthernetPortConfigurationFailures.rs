// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EthernetPortConfigurationFailures(Vec<EthernetPortConfigurationFailureKind>);

impl EthernetPortConfigurationFailures
{
	#[inline(always)]
	pub fn new() -> Self
	{
		EthernetPortConfigurationFailures(Vec::with_capacity(40))
	}
	
	#[inline(always)]
	pub fn hasFailures(&self) -> bool
	{
		self.0.len() != 0
	}
	
	#[inline(always)]
	pub fn ifError<O, E, F>(&mut self, result: Result<O, E>, ethernetPortConfigurationFailureKindConstructor: F)
	where F: Fn(E) -> EthernetPortConfigurationFailureKind
	{
		if let Err(error) = result
		{
			self.push(ethernetPortConfigurationFailureKindConstructor(error));
		}
	}
	
	#[inline(always)]
	pub fn push(&mut self, failure: EthernetPortConfigurationFailureKind)
	{
		self.0.push(failure);
	}
}
