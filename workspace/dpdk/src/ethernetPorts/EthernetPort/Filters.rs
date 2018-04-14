// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn supportsFilter(&self, filterType: FilterType) -> bool
	{
		match unsafe { rte_eth_dev_filter_supported(self.portIdentifier(), filterType.as_rte_filter_type()) }
		{
			0 => true,
			NegativeE::ENOTSUP => false,

			NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

			result @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_filter_supported()", result),
		}
	}

	#[inline(always)]
	fn privateFilterOperation(&self, filterType: rte_filter_type, filterOperation: rte_filter_op, value: *mut c_void) -> Result<(), UnsupportedByHardwareError>
	{
		match unsafe { rte_eth_dev_filter_ctrl(self.portIdentifier(), filterType, filterOperation, value) }
		{
			0 => Ok(()),
			NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

			NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

			negative if negative < 0 => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

			result @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_filter_ctrl(()", result),
		}
	}

	#[inline(always)]
	fn filterOperation(&self, filterType: FilterType, filterOperation: FilterOperation, value: *mut c_void) -> Result<(), UnsupportedByHardwareError>
	{
		self.privateFilterOperation(filterType.as_rte_filter_type(), filterOperation.as_rte_filter_op(), value)
	}

	#[inline(always)]
	fn addFilterOperation(&self, filterType: FilterType, value: *mut c_void) -> Result<(), UnsupportedByHardwareError>
	{
		self.filterOperation(filterType, FilterOperation::Add, value)
	}

	#[inline(always)]
	fn setFilterOperation(&self, filterType: FilterType, value: *mut c_void) -> Result<(), UnsupportedByHardwareError>
	{
		self.filterOperation(filterType, FilterOperation::Set, value)
	}

	#[inline(always)]
	fn flushFilterOperation(&self, filterType: FilterType, value: *mut c_void) -> Result<(), UnsupportedByHardwareError>
	{
		self.filterOperation(filterType, FilterOperation::Flush, value)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn addFilterGlobalConfigurationSetting(&self, filterGlobalConfiguration: &mut rte_eth_global_cfg) -> Result<(), UnsupportedByHardwareError>
	{
		self.privateFilterOperation(rte_filter_type::RTE_ETH_FILTER_NONE, rte_filter_op::RTE_ETH_FILTER_SET, filterGlobalConfiguration as *mut _ as *mut c_void)
	}

	#[inline(always)]
	pub fn configureFilterGreKeyLength(&self, greKeyLength: u8) -> Result<(), UnsupportedByHardwareError>
	{
		let mut data = rte_eth_global_cfg__bindgen_ty_1::default();
		unsafe
		{
			let pointer = data.gre_key_len();
			write(pointer, greKeyLength);
		}

		let mut filterGlobalConfiguration = rte_eth_global_cfg
		{
			cfg_type: rte_eth_global_cfg_type::RTE_ETH_GLOBAL_CFG_TYPE_GRE_KEY_LEN,
			cfg: data,
		};

		self.addFilterGlobalConfigurationSetting(&mut filterGlobalConfiguration)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addMediaAccessControlAddressVirtualLanFilter(&self, filter: &mut rte_eth_mac_filter) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::MediaAccessControlVirtualLan, filter as *mut _ as *mut c_void)
	}

	// Don't need to specify MAC address necessarily (can be zero for any)
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addEtherTypeFilter(&self, filter: &mut rte_eth_ethertype_filter) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::EtherType, filter as *mut _ as *mut c_void)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addFlexibleFilter(&self, filter: &mut rte_eth_flex_filter) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::Flexible, filter as *mut _ as *mut c_void)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addTcpSynFilter(&self, filter: &mut rte_eth_syn_filter) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::TcpSyn, filter as *mut _ as *mut c_void)
	}

	// Can be used for IP, TCP or UDP filtering
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addNTupleFilter(&self, filter: &mut rte_eth_ntuple_filter) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::NTuple, filter as *mut _ as *mut c_void)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addVirtualLanTunnelFilter(&self, filter: &mut rte_eth_tunnel_filter_conf) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::VirtualLanTunnel, filter as *mut _ as *mut c_void)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addFlowDirectorFilter(&self, filter: &mut rte_eth_fdir_filter) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::FlowDirector, filter as *mut _ as *mut c_void)
	}

	// Also supports Update, Information and Statistics
	// Some implementations support Set
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn flushFlowDirectorFilter(&self) -> Result<(), UnsupportedByHardwareError>
	{
		self.flushFilterOperation(FilterType::FlowDirector, null_mut())
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn setHashFilter(&self, filter: &HashFilter) -> Result<(), UnsupportedByHardwareError>
	{
		let mut value = filter.as_rte_eth_hash_filter_info();

		self.setFilterOperation(FilterType::Hash, &mut value as *mut _ as *mut c_void)
	}

	// See also rte_eth_dev_l2_tunnel_eth_type_conf() (Not implemented)
	// See also rte_eth_dev_l2_tunnel_offload_set() (Not implemented)
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn addLayer2TunnelFilter(&self, filter: &mut rte_eth_l2_tunnel_conf) -> Result<(), UnsupportedByHardwareError>
	{
		self.addFilterOperation(FilterType::Layer2Tunnel, filter as *mut _ as *mut c_void)
	}
}
