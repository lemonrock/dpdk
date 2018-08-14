// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Serialize)]
pub struct ReceiveSideScaling
{
	pub hash_key: ToeplitzHashFunctionKeyDataStrategy,
	
	pub redirection_table_strategy: RedirectionTableStategy,
	
	#[serde(skip)]
	cached: rte_flow_action_rss,
	
	// We can't cache this, as the generated size depends on the number of queues / device hash key length.
	// We could force number of queues, and ignore the number the device has.
	#[serde(skip)]
	cached_receive_side_scaling_key: Either<Cow<ToeplitzHashFunctionKeyData40Bytes>, Cow<ToeplitzHashFunctionKeyData52Bytes>>,
	
	// We can't cache this, as the generated size depends on the number of queues / device RETA table size.
	// We could force number of queues, and ignore the number the device has.
	#[serde(skip)]
	cached_receive_redirection_table: RedirectionTable,
}

custom_deserialize!
{
	ReceiveSideScaling,
	0 => source_ethernet_address,
	1 => destination_ethernet_address,
	2 => source_internet_protocol_version_4_address,
	3 => destination_internet_protocol_version_4_address,
	4 => operation,
}

impl Clone for ReceiveSideScaling
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			source_ethernet_address: self.source_ethernet_address,
			destination_ethernet_address: self.destination_ethernet_address,
			source_internet_protocol_version_4_address: self.source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address: self.destination_internet_protocol_version_4_address,
			operation: self.operation,
			cached: bitwise_clone!(self, rte_flow_item_arp_eth_ipv4),
		}
	}
}

impl PartialOrd for ReceiveSideScaling
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for ReceiveSideScaling
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.source_ethernet_address.cmp(&rhs.source_ethernet_address).then_with(|| self.destination_ethernet_address.cmp(&rhs.destination_ethernet_address)).then_with(|| self.source_internet_protocol_version_4_address.cmp(&rhs.source_internet_protocol_version_4_address)).then_with(|| self.destination_internet_protocol_version_4_address.cmp(&rhs.destination_internet_protocol_version_4_address)).then_with(|| self.operation.cmp(&rhs.operation))
	}
}

impl PartialEq for ReceiveSideScaling
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.source_ethernet_address == rhs.source_ethernet_address && self.destination_ethernet_address == rhs.destination_ethernet_address && self.source_internet_protocol_version_4_address == rhs.source_internet_protocol_version_4_address && self.destination_internet_protocol_version_4_address == rhs.destination_internet_protocol_version_4_address && self.operation == rhs.operation
	}
}

impl Eq for ReceiveSideScaling
{
}

impl Hash for ReceiveSideScaling
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.source_ethernet_address.hash(hasher);
		self.destination_ethernet_address.hash(hasher);
		self.source_internet_protocol_version_4_address.hash(hasher);
		self.destination_internet_protocol_version_4_address.hash(hasher);
		self.operation.hash(hasher)
	}
}

impl ReceiveSideScaling
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(source_ethernet_address: MediaAccessControlAddressMask, destination_ethernet_address: MediaAccessControlAddressMask, source_internet_protocol_version_4_address: NetworkEndianU32, destination_internet_protocol_version_4_address: NetworkEndianU32, operation: NetworkEndianU16) -> Self
	{
		Self
		{
			source_ethernet_address,
			destination_ethernet_address,
			source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address,
			operation,
			cached: rte_flow_action_rss
			{
				level: 0,
				types: /**< Specific RSS hash types (see ETH_RSS_*). */
				key_len: self.hash_key.len() as u32,
				queue_num: self.hash_to_receive_queue_indices_mapping.len() as u32,
				key: self.hash_key.as_ptr(),
				queue: self.hash_to_receive_queue_indices_mapping.as_ptr() as u32,
			}
		}
	}
}

/*
 * The RSS offload types are defined based on flow types which are defined
 * in rte_eth_ctrl.h. Different NIC hardwares may support different RSS offload
 * types. The supported flow types or RSS offload types can be queried by
 * rte_eth_dev_info_get().
#define ETH_RSS_IPV4               (1ULL << RTE_ETH_FLOW_IPV4)
#define ETH_RSS_FRAG_IPV4          (1ULL << RTE_ETH_FLOW_FRAG_IPV4)
#define ETH_RSS_NONFRAG_IPV4_TCP   (1ULL << RTE_ETH_FLOW_NONFRAG_IPV4_TCP)
#define ETH_RSS_NONFRAG_IPV4_UDP   (1ULL << RTE_ETH_FLOW_NONFRAG_IPV4_UDP)
#define ETH_RSS_NONFRAG_IPV4_SCTP  (1ULL << RTE_ETH_FLOW_NONFRAG_IPV4_SCTP)
#define ETH_RSS_NONFRAG_IPV4_OTHER (1ULL << RTE_ETH_FLOW_NONFRAG_IPV4_OTHER)
#define ETH_RSS_IPV6               (1ULL << RTE_ETH_FLOW_IPV6)
#define ETH_RSS_FRAG_IPV6          (1ULL << RTE_ETH_FLOW_FRAG_IPV6)
#define ETH_RSS_NONFRAG_IPV6_TCP   (1ULL << RTE_ETH_FLOW_NONFRAG_IPV6_TCP)
#define ETH_RSS_NONFRAG_IPV6_UDP   (1ULL << RTE_ETH_FLOW_NONFRAG_IPV6_UDP)
#define ETH_RSS_NONFRAG_IPV6_SCTP  (1ULL << RTE_ETH_FLOW_NONFRAG_IPV6_SCTP)
#define ETH_RSS_NONFRAG_IPV6_OTHER (1ULL << RTE_ETH_FLOW_NONFRAG_IPV6_OTHER)
#define ETH_RSS_L2_PAYLOAD         (1ULL << RTE_ETH_FLOW_L2_PAYLOAD)
#define ETH_RSS_IPV6_EX            (1ULL << RTE_ETH_FLOW_IPV6_EX)
#define ETH_RSS_IPV6_TCP_EX        (1ULL << RTE_ETH_FLOW_IPV6_TCP_EX)
#define ETH_RSS_IPV6_UDP_EX        (1ULL << RTE_ETH_FLOW_IPV6_UDP_EX)
#define ETH_RSS_PORT               (1ULL << RTE_ETH_FLOW_PORT)
#define ETH_RSS_VXLAN              (1ULL << RTE_ETH_FLOW_VXLAN)
#define ETH_RSS_GENEVE             (1ULL << RTE_ETH_FLOW_GENEVE)
#define ETH_RSS_NVGRE              (1ULL << RTE_ETH_FLOW_NVGRE)

#define ETH_RSS_IP ( \
	ETH_RSS_IPV4 | \
	ETH_RSS_FRAG_IPV4 | \
	ETH_RSS_NONFRAG_IPV4_OTHER | \
	ETH_RSS_IPV6 | \
	ETH_RSS_FRAG_IPV6 | \
	ETH_RSS_NONFRAG_IPV6_OTHER | \
	ETH_RSS_IPV6_EX)

#define ETH_RSS_UDP ( \
	ETH_RSS_NONFRAG_IPV4_UDP | \
	ETH_RSS_NONFRAG_IPV6_UDP | \
	ETH_RSS_IPV6_UDP_EX)

#define ETH_RSS_TCP ( \
	ETH_RSS_NONFRAG_IPV4_TCP | \
	ETH_RSS_NONFRAG_IPV6_TCP | \
	ETH_RSS_IPV6_TCP_EX)

#define ETH_RSS_SCTP ( \
	ETH_RSS_NONFRAG_IPV4_SCTP | \
	ETH_RSS_NONFRAG_IPV6_SCTP)

#define ETH_RSS_TUNNEL ( \
	ETH_RSS_VXLAN  | \
	ETH_RSS_GENEVE | \
	ETH_RSS_NVGRE)

/**< Mask of valid RSS hash protocols */
#define ETH_RSS_PROTO_MASK ( \
	ETH_RSS_IPV4 | \
	ETH_RSS_FRAG_IPV4 | \
	ETH_RSS_NONFRAG_IPV4_TCP | \
	ETH_RSS_NONFRAG_IPV4_UDP | \
	ETH_RSS_NONFRAG_IPV4_SCTP | \
	ETH_RSS_NONFRAG_IPV4_OTHER | \
	ETH_RSS_IPV6 | \
	ETH_RSS_FRAG_IPV6 | \
	ETH_RSS_NONFRAG_IPV6_TCP | \
	ETH_RSS_NONFRAG_IPV6_UDP | \
	ETH_RSS_NONFRAG_IPV6_SCTP | \
	ETH_RSS_NONFRAG_IPV6_OTHER | \
	ETH_RSS_L2_PAYLOAD | \
	ETH_RSS_IPV6_EX | \
	ETH_RSS_IPV6_TCP_EX | \
	ETH_RSS_IPV6_UDP_EX | \
	ETH_RSS_PORT  | \
	ETH_RSS_VXLAN | \
	ETH_RSS_GENEVE | \
	ETH_RSS_NVGRE)

 * Definitions used for redirection table entry size.
 * Some RSS RETA sizes may not be supported by some drivers, check the
 * documentation or the description of relevant functions for more details.
#define ETH_RSS_RETA_SIZE_64  64
#define ETH_RSS_RETA_SIZE_128 128
#define ETH_RSS_RETA_SIZE_256 256
#define ETH_RSS_RETA_SIZE_512 512
#define RTE_RETA_GROUP_SIZE   64

*/
