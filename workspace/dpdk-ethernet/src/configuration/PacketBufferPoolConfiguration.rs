// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Packet buffer pool configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct PacketBufferPoolConfiguration
{
	/// Number of packets.
	///
	/// If this number of packets is used up, then no more receives or transmissions are possible.
	///
	/// Should be quite large...
	pub number_of_packets: NonZeroU32,
	
	/// Number of packets cached per CPU core.
	#[serde(default = "PacketBufferPoolConfiguration::packets_cached_per_core_default")]
	pub packets_cached_per_core: NonZeroU32,
	
	/// This, added to the Default Data Room size (`RTE_MBUF_DEFAULT_DATAROOM`, 2048 bytes) and the default head room (`RTE_PKTMBUF_HEADROOM`, 128 bytes) gives the `data_size` of each packet.
	///
	/// Defaults to a value that ensures 2^16 - 1 bytes are allowed.
	///
	/// Addition is saturating, ie overflow is not a concern.
	#[serde(default = "PacketBufferPoolConfiguration::additional_buffer_headroom_bytes_default")]
	pub additional_buffer_headroom_bytes: u16,
	
	/// NUMA node, if any, to create this packet buffer pool on.
	#[serde(default)]
	pub numa_node_choice: NumaNodeChoice,
	
	/// Application private size.
	///
	/// Normally zero.
	#[serde(default)]
	pub application_private_size: u16,
}

impl PacketBufferPoolConfiguration
{
	/// Create a new instance.
	#[inline(always)]
	pub fn configure(&self, packet_buffer_pool_reference: PacketBufferPoolReference) -> Result<NonNull<rte_mempool>, ()>
	{
		let memory_zone_name = packet_buffer_pool_reference.name();
		let data_room_size = Self::minimum_data_room_size().saturating_add(self.additional_buffer_headroom_bytes);
		let result = unsafe { rte_pktmbuf_pool_create(memory_zone_name.as_ptr(), self.number_of_packets.get(), self.packets_cached_per_core.get(), self.application_private_size, data_room_size, self.numa_node_choice.into()) };
		
		if unlikely!(result.is_null())
		{
			match LogicalCore::current_logical_core_error_number()
			{
				E::ENOSPC => Err(()),
				E::ENOMEM => Err(()),
				
				E_RTE::NO_CONFIG => panic!("No config"),
				E_RTE::SECONDARY => panic!("Secondary process"),
				E::EINVAL => panic!("cache size provided is too large, or priv_size is not aligned"),
				E::EEXIST => panic!("a memzone with the same name already exists"),
				
				illegal @ _ => panic!("Unexpected error code '{}' set by rte_pktmbuf_pool_create()", illegal),
			}
		}
		else
		{
			Ok(unsafe { NonNull::new_unchecked(result) })
		}
	}
	
	#[inline(always)]
	fn packets_cached_per_core_default() -> NonZeroU32
	{
		unsafe { NonZeroU32::new_unchecked(32) }
	}
	
	#[inline(always)]
	fn additional_buffer_headroom_bytes_default() -> u16
	{
		::std::u16::MAX - Self::minimum_data_room_size()
	}
	
	#[inline(always)]
	fn minimum_data_room_size() -> u16
	{
		(RTE_PKTMBUF_HEADROOM as u16).saturating_add(RTE_MBUF_DEFAULT_DATAROOM)
	}
}
