// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Permitted classes of service.
#[derive(Debug)]
#[derive(Serialization, Deserialization)]
#[serde(default)]
pub struct PermittedClassesOfService([bool; 8]);

impl Default for PermittedClassesOfService
{
	#[inline(always)]
	fn default() -> Self
	{
		PermittedClassesOfService([true; 8])
	}
}

impl PermittedClassesOfService
{
	/// Permit this `class_of_service`.
	#[inline(always)]
	pub fn permit(&mut self, class_of_service: ClassOfService)
	{
		unsafe { *self.0.get_unchecked_mut(class_of_service as u8 as usize) = true }
	}
	
	/// Deny this `class_of_service`.
	#[inline(always)]
	pub fn deny(&mut self, class_of_service: ClassOfService)
	{
		unsafe { *self.0.get_unchecked_mut(class_of_service as u8 as usize) = false }
	}
	
	/// Is this `class_of_service` permitted?
	#[inline(always)]
	pub fn is_permitted(&self, class_of_service: ClassOfService) -> bool
	{
		unsafe { *self.0.get_unchecked(class_of_service as u8 as usize) }
	}
	
	/// Is this `class_of_service` denied?
	#[inline(always)]
	pub fn is_denied(&self, class_of_service: ClassOfService) -> bool
	{
		!self.is_permitted()
	}
}
