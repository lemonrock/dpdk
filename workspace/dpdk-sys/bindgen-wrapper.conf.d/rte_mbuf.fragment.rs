#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct rte_mbuf
{
	pub cacheline0: MARKER,
	pub buf_addr: *mut c_void,
	pub buf_physaddr: phys_addr_t,
	pub buf_len: uint16_t,
	pub rearm_data: MARKER8,
	pub data_off: uint16_t,
	pub refcnt_atomic: rte_atomic16_t,
	pub nb_segs: uint8_t,
	pub port: uint8_t,
	pub ol_flags: uint64_t,
	pub rx_descriptor_fields1: MARKER,
	pub packet_type: uint32_t,
	pub pkt_len: uint32_t,
	pub data_len: uint16_t,
	pub vlan_tci: uint16_t,
	pub hash: rte_mbuf_AnonymousUnion_hash,
	pub seqn: uint32_t,
	pub vlan_tci_outer: uint16_t,
	pub cacheline1: MARKER,
	pub _bindgen_data_3_: [u64; 1usize],
	pub pool: *mut rte_mempool,
	pub next: *mut rte_mbuf,
	pub tx_offload_OR_header_lengths_bitfield: uint64_t,
	pub priv_size: uint16_t,
	pub timesync: uint16_t,
	_bindgen_padding_0_: [u8; 28usize],
}

impl rte_mbuf
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn userdata(&mut self) -> *mut *mut c_void
	{
		let raw = &mut self._bindgen_data_3_ as *mut _ as *mut u8;
		raw as *mut _
	}
	
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn udata64(&mut self) -> *mut uint64_t
	{
		let raw = &mut self._bindgen_data_3_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Default for rte_mbuf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
