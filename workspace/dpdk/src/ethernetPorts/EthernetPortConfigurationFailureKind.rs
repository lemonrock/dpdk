// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EthernetPortConfigurationFailureKind
{
	CouldNotCreateReceiveQueueBecauseOutOfMemory(QueueIdentifier),
	CouldNotCreateTransmitQueueBecauseOutOfMemory(QueueIdentifier),
	ReceiveQueueChangeVirtualLanStripping(QueueIdentifier, UnsupportedByHardwareError),
	TransmitQueueSetRateLimit(QueueIdentifier, UnsupportedByHardwareError),
	ReceiveSideScalingHashFilter(UnsupportedByHardwareError),
	UpdateRetaIndirectionTable(UnsupportedByHardwareError),
	ClearMulticastMediaAccessControlAddressesToFilter(UnsupportedByHardwareError),
	SetMulticastMediaAccessControlAddressesToFilter(UnsupportedOrFullError),
	SetMaximumTransmissionUnit(CouldNotSetMaximumTransmissionUnitError),
	AddUdpTunnelOffload(UnsupportedByHardwareError, UdpTunnelConfiguration),
	EnableTimestamping(UnsupportedByHardwareError),
	DisableTimestamping(UnsupportedByHardwareError),
	LinkUp(i32),
	LinkDown(i32),
	LedOn(UnsupportedByHardwareError),
	LedOff(UnsupportedByHardwareError),
	SetDefaultMediaAccessControlAddress(UnsupportedByHardwareError),
	AddMediaAccessControlAddress(UnsupportedOrFullError, (MediaAccessControlAddress, Option<u6>)),
	SetFlowControl(UnsupportedByHardwareError),
	SetDataCentreBridgingPriorityFlowControl(UnsupportedByHardwareError),
	EnableTrafficMirroringRule(UnsupportedByHardwareError, TrafficMirroringRuleNumber),
	SetVirtualLanOffloadFeatures(UnsupportedByHardwareError),
	ReceiveQueueDeferredStart(QueueIdentifier),
	TransmitQueueDeferredStart(QueueIdentifier),
}
