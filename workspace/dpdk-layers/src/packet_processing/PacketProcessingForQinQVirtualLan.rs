// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Outer Virtual LAN packet processing configuration by Virtual LAN.
#[derive(Debug)]
pub struct PacketProcessingForQinQVirtualLan<PPDO: PacketProcessingDropObserver>
{
	/// Outer QinQ Virtual LAN permitted classes of service.
	pub(crate)  outer_packet_processing: PacketProcessing<PPDO>,
	
	/// Inner packet processing configuration.
	pub(crate)  inner_packet_processing: PacketProcessing<PPDO>,
}

impl<PPDO: PacketProcessingDropObserver> PacketProcessingForQinQVirtualLan<PPDO>
{
	#[inline(always)]
	pub(crate) fn honour_outer_drop_eligible_indicator(&self, outer_drop_eligible_indicator: bool) -> bool
	{
		self.outer_packet_processing.honour_drop_eligible_indicator(outer_drop_eligible_indicator)
	}
	
	#[inline(always)]
	pub(crate) fn honour_inner_drop_eligible_indicator(&self, inner_drop_eligible_indicator: bool) -> bool
	{
		self.inner_packet_processing.honour_drop_eligible_indicator(inner_drop_eligible_indicator)
	}
	
	#[inline(always)]
	pub(crate) fn drop_packets_of_outer_class_of_service(&self, outer_class_of_service: ClassOfService) -> bool
	{
		self.outer_packet_processing.drop_packets_of_class_of_service(outer_class_of_service)
	}
	
	#[inline(always)]
	pub(crate) fn drop_packets_of_inner_class_of_service(&self, inner_class_of_service: ClassOfService) -> bool
	{
		self.inner_packet_processing.drop_packets_of_class_of_service(inner_class_of_service)
	}
}
