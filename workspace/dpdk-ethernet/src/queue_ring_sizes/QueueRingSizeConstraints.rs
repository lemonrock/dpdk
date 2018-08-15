// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents constraints on a queue ring size.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct QueueRingSizeConstraints<Q: QueueRingSize>
{
	/// Inclusive maximum.
	pub inclusive_maximum: Q,
	
	/// Inclusive minimum.
	pub inclusive_minimum: Q,
	
	/// Alignment.
	pub alignment: NonZeroU16,
}

impl From<rte_eth_desc_lim> for QueueRingSizeConstraints<ReceiveQueueRingSize>
{
	#[inline(always)]
	fn from(value: rte_eth_desc_lim) -> Self
	{
		Self
		{
			inclusive_maximum: ReceiveQueueRingSize::try_from(value.nb_max).unwrap(),
			inclusive_minimum: ReceiveQueueRingSize::try_from(value.nb_min).unwrap(),
			alignment: NonZeroU16::new(value.nb_align).unwrap(),
		}
	}
}

impl From<rte_eth_desc_lim> for QueueRingSizeConstraints<TransmitQueueRingSize>
{
	#[inline(always)]
	fn from(value: rte_eth_desc_lim) -> Self
	{
		Self
		{
			inclusive_maximum: TransmitQueueRingSize::try_from(value.nb_max).unwrap(),
			inclusive_minimum: TransmitQueueRingSize::try_from(value.nb_min).unwrap(),
			alignment: NonZeroU16::new(value.nb_align).unwrap(),
		}
	}
}

impl<Q: QueueRingSize> QueueRingSizeConstraints<Q>
{
	/// Constraints a queue ring size.
	#[inline(always)]
	pub fn constrain(&self, queue_ring_size: Q) -> Q
	{
		let in_range: u16 = max(self.inclusive_minimum, min(self.inclusive_maximum, queue_ring_size)).into();
		
		let alignment = self.alignment.get();
		let aligned = ((in_range + alignment - 1) / alignment) * alignment;
		Q::try_from(aligned).unwrap_or(self.inclusive_maximum)
	}
}
