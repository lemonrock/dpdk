// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub enum HashFilter
{
	SymmetricHashPerPort
	{
		enable: bool,
	},

	GlobalConfigurationForAllPortsOfSameNic
	{
		hashFilterFunction: HashFilterFunction,

		// Only used by the Intel i40e driver
		appliesToFlowTypes: ReceiveSideScalingOffloadFlowTypeSet,
		flowTypesValidForEthernetDeviceDriver: ReceiveSideScalingOffloadFlowTypeSet,
	},

	InputSetSelect
	{
		flowType: ReceiveSideScalingOffloadFlowType,
		inputSet: HashSet<FilterInputSetField>,
		filterInputSetOperation: FilterInputSetOperation,
	},
}

impl HashFilter
{
	pub const GenericToeplitzHashFilter: HashFilter = HashFilter::GlobalConfigurationForAllPortsOfSameNic
	{
		hashFilterFunction: HashFilterFunction::Toeplitz,
		appliesToFlowTypes: ReceiveSideScalingOffloadFlowTypeSet::empty(),
		flowTypesValidForEthernetDeviceDriver: ReceiveSideScalingOffloadFlowTypeSet::empty(),
	};

	#[inline(always)]
	pub fn as_rte_eth_hash_filter_info(&self) -> rte_eth_hash_filter_info
	{
		let mut info = rte_eth_hash_filter_info__bindgen_ty_1::default();

		let info_type = match *self
		{
			HashFilter::SymmetricHashPerPort { enable } =>
			{
				let enable = if enable
				{
					1
				}
				else
				{
					0
				};
				unsafe { write(info.enable(), enable) };

				rte_eth_hash_filter_info_type::RTE_ETH_HASH_FILTER_SYM_HASH_ENA_PER_PORT
			},

			HashFilter::GlobalConfigurationForAllPortsOfSameNic { hashFilterFunction, appliesToFlowTypes, flowTypesValidForEthernetDeviceDriver } =>
			{
				let globalConfiguration = rte_eth_hash_global_conf
				{
					hash_func: hashFilterFunction.as_rte_eth_hash_function(),
					sym_hash_enable_mask: appliesToFlowTypes.asHashFilterSet(),
					valid_bit_mask: flowTypesValidForEthernetDeviceDriver.asHashFilterSet(),
				};

				unsafe { write(info.global_conf(), globalConfiguration) };

				rte_eth_hash_filter_info_type::RTE_ETH_HASH_FILTER_GLOBAL_CONFIG
			},

			HashFilter::InputSetSelect { flowType, ref inputSet, filterInputSetOperation } =>
			{
				let length = inputSet.len();
				assert!(length < RTE_ETH_INSET_SIZE_MAX, "inputSet.len() '{}' exceeds RTE_ETH_INSET_SIZE_MAX '{}'", length, RTE_ETH_INSET_SIZE_MAX);

				let mut field: [rte_eth_input_set_field; RTE_ETH_INSET_SIZE_MAX] = unsafe { zeroed() };

				let mut index = 0;
				for value in inputSet.iter()
				{
					field[index] = value.as_rte_eth_input_set_field();
					index += 1;
				}

				let inputSetConfiguration = rte_eth_input_set_conf
				{
					flow_type: flowType as u16,
					inset_size: length as u16,
					field,
					op: filterInputSetOperation.as_rte_filter_input_set_op()
				};

				unsafe { write(info.input_set_conf(), inputSetConfiguration) };

				rte_eth_hash_filter_info_type::RTE_ETH_HASH_FILTER_INPUT_SET_SELECT
			},
		};

		rte_eth_hash_filter_info
		{
			info_type,
			info,
		}
	}
}
