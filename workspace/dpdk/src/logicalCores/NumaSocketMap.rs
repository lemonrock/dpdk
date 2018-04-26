// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NumaSocketMap<V: Sized>
{
	map: [Option<V>; NumaNode::MaximumNumaSockets],
	defaultKey: Option<NumaSocketId>,
}

impl<V: Sized> NumaSocketMap<V>
{
	pub fn isValidNumaSocket(&self, index: usize) -> bool
	{
		self.map[index].is_some()
	}

	#[inline(always)]
	pub fn new() -> Self
	{
		let map = [None, None, None, None, None, None, None, None,];

		NumaSocketMap
		{
			map,
			defaultKey: None,
		}
	}

	pub fn iterateSockets<F>(&self, mut callback: F)
	where F: FnMut(NumaSocketId) -> ()
	{
		self.iterate(|numa_socket_id, _| { callback(numa_socket_id) })
	}

	pub fn iterate<F>(&self, mut callback: F)
	where F: FnMut(NumaSocketId, &V) -> ()
	{
		for index in 0..NumaNode::MaximumNumaSockets
		{
			if let Some(ref value) = self.map[index]
			{
				callback(NumaSocketId::fromU32(index as u32).unwrap(), value)
			}
		}
	}

	#[inline(always)]
	pub fn get(&self, key: NumaSocketId) -> Option<&V>
	{
		let index = key.as_usize();
		if let Some(ref value) = self.map[index]
		{
			Some(value)
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	pub fn getOrPanic(&self, key: NumaSocketId) -> &V
	{
		self.get(key).unwrap()
	}

	#[inline(always)]
	pub fn lowestKey(&self) -> Option<NumaSocketId>
	{
		for index in 0..NumaNode::MaximumNumaSockets
		{
			let value = &self.map[index];
			if value.is_some()
			{
				return Some(NumaSocketId::fromU32(index as u32).unwrap());
			}
		}
		None
	}

	#[inline(always)]
	pub fn has(&self, key: NumaSocketId) -> bool
	{
		let index = key.as_usize();
		self.map[index].is_some()
	}

	#[inline(always)]
	pub fn doesNotHave(&self, key: NumaSocketId) -> bool
	{
		let index = key.as_usize();
		self.map[index].is_none()
	}

	#[inline(always)]
	pub fn putOnce(&mut self, key: NumaSocketId, value: V)
	{
		let index = key.as_usize();
		if self.map[index].is_some()
		{
			panic!("Already contains key '{:?}'", key);
		}
		self.map[index] = Some(value);
	}

	// Designed to cope with configuring memory pools / packet buffer pools for EthernetPorts, which may not have an associated NumaSocketId AND we want only one default memory pool which is associated with the first ethernet port configured as they're very expensive
	#[inline(always)]
	pub fn getWithPutOnceIfMissingUsingDefaultKeyIfEmpty<F>(&mut self, key: Option<NumaSocketId>, default: F) -> &V
	where F: Fn(NumaSocketId) -> V
	{
		if let Some(key) = key
		{
			let index = key.as_usize();
			if let Some(ref value) = self.map[index]
			{
				value
			}
			else
			{
				let index = key.as_usize();
				self.defaultMe(key, index, default)
			}
		}
		else
		{
			// In the event that the very first get encounters no data whatsoever, we need to make a choice
			if self.defaultKey.is_none()
			{
				self.defaultKey = Some(NumaSocketId::SocketZeroAlwaysExists);
			}
			let defaultKey = self.defaultKey.unwrap();

			let index = defaultKey.as_usize();
			if let Some(ref value) = self.map[index]
			{
				value
			}
			else
			{
				self.defaultMe(defaultKey, index, default)
			}
		}
	}

	#[inline(always)]
	fn defaultMe<F>(&mut self, key: NumaSocketId, index: usize, default: F) -> &V
	where F: Fn(NumaSocketId) -> V
	{
		let value = default(key);
		self.map[index] = Some(value);
		if let Some(ref value) = self.map[index]
		{
			value
		}
		else
		{
			unreachable!("Should not be possible");
		}
	}
}
