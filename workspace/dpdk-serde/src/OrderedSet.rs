// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct OrderedSet<V: Debug + Clone + Eq + Hash + Serialize + Deserialize>(OrderMap<V, ()>);

impl<V: Debug + Clone + Eq + Hash + Serialize + Deserialize> Serialize for OrderedSet<V>
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let ordered: Vec<V> = self.0.keys().map(|key| key.clone()).collect();
		ordered.serialize(serializer)
	}
}

impl<V: Debug + Clone + Eq + Hash + Serialize + Deserialize> Deserialize for OrderedSet<V>
{
	fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error>
	{
		let mut ordered = Vec::<V>::deserialize(deserializer)?;
		let mut map = OrderMap::with_capacity(ordered.len());
		for value in ordered.drain(..)
		{
			map.insert(value, ());
		}
		Ok(OrderedSet(map))
	}
}

impl<V: Debug + Clone + Eq + Hash + Serialize + Deserialize> Default for OrderedSet<V>
{
	#[inline(always)]
	fn default() -> Self
	{
		OrderedSet(Default::default())
	}
}
