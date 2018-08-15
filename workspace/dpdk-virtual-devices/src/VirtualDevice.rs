// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a virtual device.
pub trait VirtualDevice: Debug + Sized
{
	/// Device driver name.
	type V: DeviceDriverName;

	/// Driver name.
	const DriverName: Self::V;

	/// Name.
	#[inline(always)]
	fn name(&self, index: VirtualDeviceIndex) -> VirtualDeviceName<Self::V>
	{
		VirtualDeviceName::new(Self::DriverName, index)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn as_initialization_argument(&self, index: VirtualDeviceIndex) -> String
	{
		format!("{}{}", self.name(index).to_string(), self.formatted_virtual_device_arguments_with_leading_comma())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String;
}
