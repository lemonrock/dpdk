// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlowControl
{
	enableTransmitFlowControl: bool,
	enableReceiveFlowControl: bool,
	highThresholdValueToTriggerXOFF: u32,
	lowThresholdValueToTriggerXON: u32,
	pauseQuota: u16,
	forwardMediaAccessControlFrames: bool,
	usePauseAutonegotiation: bool,
}

impl Default for FlowControl
{
	#[inline(always)]
	fn default() -> Self
	{
		// From http://www.dpdk.org/doc/api/quota_watermark_2qw_2init_8c-example.html#_a4
		FlowControl::new(true, true, 80 * 510 / 100, 60 * 510 / 100, 1337, false, false)
	}
}

impl FlowControl
{
	#[inline(always)]
	pub fn new(enableTransmitFlowControl: bool, enableReceiveFlowControl: bool, highThresholdValueToTriggerXOFF: u32, lowThresholdValueToTriggerXON: u32, pauseQuota: u16, forwardMediaAccessControlFrames: bool, usePauseAutonegotiation: bool) -> Self
	{
		debug_assert!(highThresholdValueToTriggerXOFF > lowThresholdValueToTriggerXON, "highThresholdValueToTriggerXOFF '{}' > lowThresholdValueToTriggerXON '{}'", highThresholdValueToTriggerXOFF, lowThresholdValueToTriggerXON);
		FlowControl
		{
			enableTransmitFlowControl,
			enableReceiveFlowControl,
			highThresholdValueToTriggerXOFF,
			lowThresholdValueToTriggerXON,
			pauseQuota,
			forwardMediaAccessControlFrames,
			usePauseAutonegotiation,
		}
	}

	#[inline(always)]
	pub fn as_rte_eth_fc_conf(&self) -> rte_eth_fc_conf
	{
		let mode = if self.enableTransmitFlowControl
		{
			if self.enableReceiveFlowControl
			{
				rte_eth_fc_mode::RTE_FC_FULL
			}
			else
			{
				// yes, this is the right value!
				rte_eth_fc_mode::RTE_FC_RX_PAUSE
			}
		}
		else if self.enableReceiveFlowControl
		{
			// yes, this is the right value!
			rte_eth_fc_mode::RTE_FC_TX_PAUSE
		}
		else
		{
			rte_eth_fc_mode::RTE_FC_NONE
		};

		rte_eth_fc_conf
		{
		 	high_water: self.highThresholdValueToTriggerXOFF,
			low_water: self.lowThresholdValueToTriggerXON,
			pause_time: self.pauseQuota,
			send_xon: 0,
			mode,
			mac_ctrl_frame_fwd: if self.forwardMediaAccessControlFrames
			{
				1
			}
			else
			{
				0
			},
			autoneg: if self.usePauseAutonegotiation
			{
				1
			}
			else
			{
				0
			},
		}
	}
}
