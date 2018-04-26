// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Outer Virtual LAN packet processing configuration by Virtual LAN.
#[derive(Debug)]
#[derive(Serialization, Deserialization)]
pub struct PacketProcessingConfigurationForQinQVirtualLan
{
	/// Outer QinQ Virtual LAN permitted classes of service.
	pub outer_permitted_classes_of_service: ClassOfService,
	
	/// Inner packet processing configuration.
	pub inner_packet_processing_configuration: PacketProcessingConfiguration,
}

impl PacketProcessingConfigurationForQinQVirtualLan
{
	#[inline(always)]
	pub(crate) fn drop_packets_of_class_of_service(&self, outer_class_of_service: ClassOfService, inner_class_of_service: ClassOfService) -> bool
	{
		self.outer_permitted_classes_of_service.is_denied(outer_class_of_service) || self.packet_processing_configuration.drop_packets_of_class_of_service(inner_class_of_service)
	}
}
