// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A redirection table strategy.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum RedirectionTableStategy
{
	/// Assigns queues to hash indices.
	///
	/// * If the number of queues is an exact multiple of the support table size, all queues are represented fairly;
	/// * If the number of queues is not, then the remainder will be over-represented slightly;
	/// * In an extreme case, where there are fewer hash indices than queues, some queues will not be represented at all.
	Striped
	{
		/// First queue identifier.
		///
		/// Recommended to not be 0, so that one can use this queue for special work.
		first_receive_side_scaling_queue_index: ReceiveQueueIdentifier,
	},
}

impl RedirectionTableStategy
{
	/// Creates a redirection table (RETA).
	///
	/// If `first_queue_index >= number_of_receive_queues`, returns an Error.
	///
	/// `device_specific_reta_size` must be one of the supported DPDK sizes:-
	///
	/// * `ETH_RSS_RETA_SIZE_64`
	/// * `ETH_RSS_RETA_SIZE_128`
	/// * `ETH_RSS_RETA_SIZE_256`
	/// * `ETH_RSS_RETA_SIZE_512`
	///
	/// If it is not, this code will panic.
	#[inline(always)]
	pub fn create(&self, device_specific_reta_size: u16, number_of_receive_queues: usize, first_queue_index: ReceiveQueueIdentifier) -> Result<RedirectionTable, ()>
	{
		use self::RedirectionTableStategy::*;
		use self::RedirectionTable::*;
		
		{
			let first_queue_index: usize = first_queue_index.into();
			if first_queue_index > number_of_receive_queues
			{
				return Err(())
			}
		}
		
		macro_rules! entry
		{
			($first_queue_index: ident, $last_queue_index: ident, $queue_index: ident) =>
			{
				rte_eth_rss_reta_entry64
				{
					mask:
					{
						const BitMaskAll: u64 = ::std::u64::MAX;
						BitMaskAll
					},
					reta:
					{
						let mut reta: [u16; 64] = unsafe { uninitialized() };
						for set_index in 0 .. RTE_RETA_GROUP_SIZE
						{
							*(unsafe { reta.get_unchecked_mut(set_index) }) = $queue_index.into();
							
							if $queue_index == $last_queue_index
							{
								$queue_index = $first_queue_index;
							}
							else
							{
								$queue_index += 1usize;
							}
						}
						reta
					}
				}
			}
		}
		
		match *self
		{
			Striped { first_receive_side_scaling_queue_index } =>
			{
				let last_queue_index = first_receive_side_scaling_queue_index + min(number_of_receive_queues, device_specific_reta_size as usize);
				let mut queue_index = first_queue_index;
				
				let redirection_table = match device_specific_reta_size
				{
					ETH_RSS_RETA_SIZE_64 => Entries64
					(
						[
							entry!(first_queue_index, last_queue_index, queue_index),
						]
					),
					
					ETH_RSS_RETA_SIZE_128 => Entries128
					(
						[
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
						]
					),
					
					ETH_RSS_RETA_SIZE_256 => Entries256
					(
						[
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
						]
					),
					
					ETH_RSS_RETA_SIZE_512 => Entries512
					(
						[
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
							entry!(first_queue_index, last_queue_index, queue_index),
						]
					),
					
					_ => panic!("Unsupported reta size '{}'", device_specific_reta_size)
				};
				
				Ok(redirection_table)
			}
		}
	}
}
