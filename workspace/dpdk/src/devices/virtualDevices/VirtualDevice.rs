// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait VirtualDevice: Debug + Sized
{
	type V: DeviceDriverName;
	
	const DriverName: Self::V;
	
	#[inline(always)]
	fn name(&self) -> VirtualDeviceName<Self::V>
	{
		VirtualDeviceName::new(Self::DriverName, self.index())
	}
	
	#[inline(always)]
	fn index(&self) -> u5;
	
	#[inline(always)]
	fn asInitialisationArgument(&self) -> CString
	{
		CString::new(format!("{}{}", self.name().to_string(), self.formattedVirtualDeviceArgumentsWithLeadingComma())).unwrap()
	}
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String;
	
	#[inline(always)]
	fn addToMap(self, map: &mut HashMap<u8, Self>) -> Option<Self>
	{
		map.insert(self.index(), self)
	}	
}
