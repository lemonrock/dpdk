// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_mempool
{
	pub name: [c_char; 32usize],
	pub __bindgen_anon_1: rte_mempool__bindgen_ty_1,
	pub pool_config: *mut c_void,
	pub mz: *const rte_memzone,
	pub flags: c_uint,
	pub socket_id: c_int,
	pub size: u32,
	pub cache_size: u32,
	pub elt_size: u32,
	pub header_size: u32,
	pub trailer_size: u32,
	pub private_data_size: c_uint,
	pub ops_index: i32,
	pub local_cache: *mut rte_mempool_cache,
	pub populated_size: u32,
	pub elt_list: rte_mempool_objhdr_list,
	pub nb_mem_chunks: u32,
	pub mem_list: rte_mempool_memhdr_list,
	pub __bindgen_padding_0: [u64; 5usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_mempool
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mempool
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_mempool {{ name: [{}], __bindgen_anon_1: {:?}, pool_config: {:?}, mz: {:?}, local_cache: {:?}, elt_list: {:?}, mem_list: {:?} }}",
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.__bindgen_anon_1,
			self.pool_config,
			self.mz,
			self.local_cache,
			self.elt_list,
			self.mem_list
		)
	}
}
