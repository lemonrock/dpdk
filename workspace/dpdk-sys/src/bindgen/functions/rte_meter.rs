// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_meter_srtcm_config(m: *mut rte_meter_srtcm, p: *mut rte_meter_srtcm_profile) -> c_int;
	pub fn rte_meter_srtcm_profile_config(p: *mut rte_meter_srtcm_profile, params: *mut rte_meter_srtcm_params) -> c_int;
	pub fn rte_meter_trtcm_config(m: *mut rte_meter_trtcm, p: *mut rte_meter_trtcm_profile) -> c_int;
	pub fn rte_meter_trtcm_profile_config(p: *mut rte_meter_trtcm_profile, params: *mut rte_meter_trtcm_params) -> c_int;
}
