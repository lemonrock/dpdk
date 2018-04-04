// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportedDevice
{
	pub vendor: VendorId,
	pub driver: DriverIdentifier,
	pub device: DeviceId,
}

impl SupportedDevice
{
	/// This test does not return true is Any (0xFFFF) is set for a vendor or device id
	#[inline(always)]
	pub fn is_exactly_rte_pci_id(&self, other: &rte_pci_id) -> bool
	{
		self.vendor.is(other.vendor_id) && self.device.is(other.device_id)
	}
		
	#[inline(always)]
	pub fn underlyingEthernetPort(&self) -> Vec<(EthernetPort, rte_eth_dev)>
	{
		let mut matches = EthernetPort::newVecWithCapacityForAllAttachedPorts();
		
		let mut portIdentifier = 0;
		while portIdentifier < EthernetPort::MaximumEthernetPorts
		{
			if let Some(ethernetPort) = EthernetPort::new(portIdentifier as u8)
			{
				let underlying_rte_eth_dev = ethernetPort.underlyingEthernetDevice();
				if !underlying_rte_eth_dev.device.is_null()
				{
					unsafe
					{
						let pciDevice = *(rust_RTE_DEV_TO_PCI(underlying_rte_eth_dev.device));
						if self.is_exactly_rte_pci_id(&pciDevice.id)
						{
							matches.push((ethernetPort, underlying_rte_eth_dev));
						}
					}
				}
			}
			portIdentifier += 1;
		}
		
		matches
	}
}

impl SupportedDevice
{
	pub const AmazonEna_VF: SupportedDevice = Self::AmazonEna(0xEC20);
	pub const AmazonEna_LLQ_VF: SupportedDevice = Self::AmazonEna(0xEC21);
	
	pub const BroadcomBnx2x_ChipNumber_57711: SupportedDevice = Self::BroadcomBnx2x(0x164f);
	pub const BroadcomBnx2x_ChipNumber_57711E: SupportedDevice = Self::BroadcomBnx2x(0x1650);
	pub const BroadcomBnx2x_ChipNumber_57712: SupportedDevice = Self::BroadcomBnx2x(0x1662);
	pub const BroadcomBnx2x_ChipNumber_57712_MF: SupportedDevice = Self::BroadcomBnx2x(0x1663);
	pub const BroadcomBnx2x_ChipNumber_57712_VF: SupportedDevice = Self::BroadcomBnx2x(0x166f);
	pub const BroadcomBnx2x_ChipNumber_57800: SupportedDevice = Self::BroadcomBnx2x(0x168a);
	pub const BroadcomBnx2x_ChipNumber_57800_MF: SupportedDevice = Self::BroadcomBnx2x(0x16a5);
	pub const BroadcomBnx2x_ChipNumber_57800_VF: SupportedDevice = Self::BroadcomBnx2x(0x16a9);
	pub const BroadcomBnx2x_ChipNumber_57810: SupportedDevice = Self::BroadcomBnx2x(0x168e);
	pub const BroadcomBnx2x_ChipNumber_57810_MF: SupportedDevice = Self::BroadcomBnx2x(0x16ae);
	pub const BroadcomBnx2x_ChipNumber_57810_VF: SupportedDevice = Self::BroadcomBnx2x(0x16af);
	pub const BroadcomBnx2x_ChipNumber_57811: SupportedDevice = Self::BroadcomBnx2x(0x163d);
	pub const BroadcomBnx2x_ChipNumber_57811_MF: SupportedDevice = Self::BroadcomBnx2x(0x163e);
	pub const BroadcomBnx2x_ChipNumber_57811_VF: SupportedDevice = Self::BroadcomBnx2x(0x163f);
	pub const BroadcomBnx2x_ChipNumber_57840_2_20: SupportedDevice = Self::BroadcomBnx2x(0x16a2);
	pub const BroadcomBnx2x_ChipNumber_57840_4_10: SupportedDevice = Self::BroadcomBnx2x(0x16a1);
	pub const BroadcomBnx2x_ChipNumber_57840_MF: SupportedDevice = Self::BroadcomBnx2x(0x16a4);
	pub const BroadcomBnx2x_ChipNumber_57840_OBS: SupportedDevice = Self::BroadcomBnx2x(0x168d);
	pub const BroadcomBnx2x_ChipNumber_57840_OBS_MF: SupportedDevice = Self::BroadcomBnx2x(0x16ab);
	pub const BroadcomBnx2x_ChipNumber_57840_VF: SupportedDevice = Self::BroadcomBnx2x(0x16ad);
	
	pub const BroadcomBnxt_ChipNumber_57301: SupportedDevice = Self::BroadcomBnxt(0x16c8);
	pub const BroadcomBnxt_ChipNumber_57302: SupportedDevice = Self::BroadcomBnxt(0x16c9);
	pub const BroadcomBnxt_ChipNumber_57304_PF: SupportedDevice = Self::BroadcomBnxt(0x16ca);
	pub const BroadcomBnxt_ChipNumber_57304_VF: SupportedDevice = Self::BroadcomBnxt(0x16cb);
	pub const BroadcomBnxt_ChipNumber_57402: SupportedDevice = Self::BroadcomBnxt(0x16d0);
	pub const BroadcomBnxt_ChipNumber_57404: SupportedDevice = Self::BroadcomBnxt(0x16d1);
	pub const BroadcomBnxt_ChipNumber_57406_PF: SupportedDevice = Self::BroadcomBnxt(0x16d2);
	pub const BroadcomBnxt_ChipNumber_57406_VF: SupportedDevice = Self::BroadcomBnxt(0x16d3);
	pub const BroadcomBnxt_ChipNumber_57406_MF: SupportedDevice = Self::BroadcomBnxt(0x16d4);
	pub const BroadcomBnxt_ChipNumber_57314: SupportedDevice = Self::BroadcomBnxt(0x16df);
	
	pub const ChelsioT5_T580_dbg: SupportedDevice = Self::ChelsioT5(0x5000);
	pub const ChelsioT5_T520_cr: SupportedDevice = Self::ChelsioT5(0x5001);
	pub const ChelsioT5_T522_cr: SupportedDevice = Self::ChelsioT5(0x5002);
	pub const ChelsioT5_T540_cr: SupportedDevice = Self::ChelsioT5(0x5003);
	pub const ChelsioT5_T520_bch: SupportedDevice = Self::ChelsioT5(0x5004);
	pub const ChelsioT5_T540_bch: SupportedDevice = Self::ChelsioT5(0x5005);
	pub const ChelsioT5_T540_ch: SupportedDevice = Self::ChelsioT5(0x5006);
	pub const ChelsioT5_T520_so: SupportedDevice = Self::ChelsioT5(0x5007);
	pub const ChelsioT5_T520_cx: SupportedDevice = Self::ChelsioT5(0x5008);
	pub const ChelsioT5_T520_bt: SupportedDevice = Self::ChelsioT5(0x5009);
	pub const ChelsioT5_T504_bt: SupportedDevice = Self::ChelsioT5(0x500a);
	pub const ChelsioT5_B520_sr: SupportedDevice = Self::ChelsioT5(0x500b);
	pub const ChelsioT5_B504_bt: SupportedDevice = Self::ChelsioT5(0x500c);
	pub const ChelsioT5_T580_cr: SupportedDevice = Self::ChelsioT5(0x500d);
	pub const ChelsioT5_T540_LP_cr: SupportedDevice = Self::ChelsioT5(0x500e);
	pub const ChelsioT5_T580_LP_cr: SupportedDevice = Self::ChelsioT5(0x5010);
	pub const ChelsioT5_T520_LL_cr: SupportedDevice = Self::ChelsioT5(0x5011);
	pub const ChelsioT5_T560_cr: SupportedDevice = Self::ChelsioT5(0x5012);
	pub const ChelsioT5_T580_chr: SupportedDevice = Self::ChelsioT5(0x5013);
	pub const ChelsioT5_T580_so: SupportedDevice = Self::ChelsioT5(0x5014);
	pub const ChelsioT5_T502_bt: SupportedDevice = Self::ChelsioT5(0x5015);
	pub const ChelsioT5_Custom_T540_cr: SupportedDevice = Self::ChelsioT5(0x5080);
	pub const ChelsioT5_Custom_T540_LL_cr: SupportedDevice = Self::ChelsioT5(0x5081);
	pub const ChelsioT5_Custom_T504_cr: SupportedDevice = Self::ChelsioT5(0x5082);
	pub const ChelsioT5_Custom_T540_LP_CR: SupportedDevice = Self::ChelsioT5(0x5083);
	pub const ChelsioT5_Custom_T580_cr: SupportedDevice = Self::ChelsioT5(0x5084);
	pub const ChelsioT5_Custom_3x_T580_CR: SupportedDevice = Self::ChelsioT5(0x5085);
	pub const ChelsioT5_Custom_2x_T580_CR: SupportedDevice = Self::ChelsioT5(0x5086);
	pub const ChelsioT5_Custom_T580_CR: SupportedDevice = Self::ChelsioT5(0x5087);
	pub const ChelsioT5_Custom_T570_CR: SupportedDevice = Self::ChelsioT5(0x5088);
	pub const ChelsioT5_Custom_T520_CR: SupportedDevice = Self::ChelsioT5(0x5089);
	pub const ChelsioT5_Custom_T540_CR: SupportedDevice = Self::ChelsioT5(0x5090);
	pub const ChelsioT5_Custom_T522_CR: SupportedDevice = Self::ChelsioT5(0x5091);
	pub const ChelsioT5_Custom_T520_CR_alt: SupportedDevice = Self::ChelsioT5(0x5092);
	
	pub const CiscoVic_Enet: SupportedDevice = Self::CiscoVic(0x0043);
	pub const CiscoVic_Enet_VF: SupportedDevice = Self::CiscoVic(0x0071);
	
	pub const IntelE1000Em_82540EM: SupportedDevice = Self::IntelE1000Em(0x100E);
	pub const IntelE1000Em_82545EM_COPPER: SupportedDevice = Self::IntelE1000Em(0x100F);
	pub const IntelE1000Em_82545EM_FIBER: SupportedDevice = Self::IntelE1000Em(0x1011);
	pub const IntelE1000Em_82546EB_COPPER: SupportedDevice = Self::IntelE1000Em(0x1010);
	pub const IntelE1000Em_82546EB_FIBER: SupportedDevice = Self::IntelE1000Em(0x1012);
	pub const IntelE1000Em_82546EB_QUAD_COPPER: SupportedDevice = Self::IntelE1000Em(0x101D);
	pub const IntelE1000Em_82571EB_COPPER: SupportedDevice = Self::IntelE1000Em(0x105E);
	pub const IntelE1000Em_82571EB_FIBER: SupportedDevice = Self::IntelE1000Em(0x105F);
	pub const IntelE1000Em_82571EB_SERDES: SupportedDevice = Self::IntelE1000Em(0x1060);
	pub const IntelE1000Em_82571EB_SERDES_DUAL: SupportedDevice = Self::IntelE1000Em(0x10D9);
	pub const IntelE1000Em_82571EB_SERDES_QUAD: SupportedDevice = Self::IntelE1000Em(0x10DA);
	pub const IntelE1000Em_82571EB_QUAD_COPPER: SupportedDevice = Self::IntelE1000Em(0x10A4);
	pub const IntelE1000Em_82571PT_QUAD_COPPER: SupportedDevice = Self::IntelE1000Em(0x10D5);
	pub const IntelE1000Em_82571EB_QUAD_FIBER: SupportedDevice = Self::IntelE1000Em(0x10A5);
	pub const IntelE1000Em_82571EB_QUAD_COPPER_LP: SupportedDevice = Self::IntelE1000Em(0x10BC);
	pub const IntelE1000Em_82572EI_COPPER: SupportedDevice = Self::IntelE1000Em(0x107D);
	pub const IntelE1000Em_82572EI_FIBER: SupportedDevice = Self::IntelE1000Em(0x107E);
	pub const IntelE1000Em_82572EI_SERDES: SupportedDevice = Self::IntelE1000Em(0x107F);
	pub const IntelE1000Em_82572EI: SupportedDevice = Self::IntelE1000Em(0x10B9);
	pub const IntelE1000Em_82573L: SupportedDevice = Self::IntelE1000Em(0x109A);
	pub const IntelE1000Em_82574L: SupportedDevice = Self::IntelE1000Em(0x10D3);
	pub const IntelE1000Em_82574LA: SupportedDevice = Self::IntelE1000Em(0x10F6);
	pub const IntelE1000Em_82583V: SupportedDevice = Self::IntelE1000Em(0x150C);
	pub const IntelE1000Em_PCH_LPT_I217_LM: SupportedDevice = Self::IntelE1000Em(0x153A);
	pub const IntelE1000Em_PCH_LPT_I217_V: SupportedDevice = Self::IntelE1000Em(0x153B);
	pub const IntelE1000Em_PCH_LPTLP_I218_LM: SupportedDevice = Self::IntelE1000Em(0x155A);
	pub const IntelE1000Em_PCH_LPTLP_I218_V: SupportedDevice = Self::IntelE1000Em(0x1559);
	pub const IntelE1000Em_PCH_I218_LM2: SupportedDevice = Self::IntelE1000Em(0x15A0);
	pub const IntelE1000Em_PCH_I218_V2: SupportedDevice = Self::IntelE1000Em(0x15A1);
	pub const IntelE1000Em_PCH_I218_LM3: SupportedDevice = Self::IntelE1000Em(0x15A2);
	pub const IntelE1000Em_PCH_I218_V3: SupportedDevice = Self::IntelE1000Em(0x15A3);
	
	pub const IntelE1000Igb_82576: SupportedDevice = Self::IntelE1000Igb(0x10C9);
	pub const IntelE1000Igb_82576_FIBER: SupportedDevice = Self::IntelE1000Igb(0x10E6);
	pub const IntelE1000Igb_82576_SERDES: SupportedDevice = Self::IntelE1000Igb(0x10E7);
	pub const IntelE1000Igb_82576_QUAD_COPPER: SupportedDevice = Self::IntelE1000Igb(0x10E8);
	pub const IntelE1000Igb_82576_QUAD_COPPER_ET2: SupportedDevice = Self::IntelE1000Igb(0x1526);
	pub const IntelE1000Igb_82576_NS: SupportedDevice = Self::IntelE1000Igb(0x150A);
	pub const IntelE1000Igb_82576_NS_SERDES: SupportedDevice = Self::IntelE1000Igb(0x1518);
	pub const IntelE1000Igb_82576_SERDES_QUAD: SupportedDevice = Self::IntelE1000Igb(0x150D);
	pub const IntelE1000Igb_82575EB_COPPER: SupportedDevice = Self::IntelE1000Igb(0x10A7);
	pub const IntelE1000Igb_82575EB_FIBER_SERDES: SupportedDevice = Self::IntelE1000Igb(0x10A9);
	pub const IntelE1000Igb_82575GB_QUAD_COPPER: SupportedDevice = Self::IntelE1000Igb(0x10D6);
	pub const IntelE1000Igb_82580_COPPER: SupportedDevice = Self::IntelE1000Igb(0x150E);
	pub const IntelE1000Igb_82580_FIBER: SupportedDevice = Self::IntelE1000Igb(0x150F);
	pub const IntelE1000Igb_82580_SERDES: SupportedDevice = Self::IntelE1000Igb(0x1510);
	pub const IntelE1000Igb_82580_SGMII: SupportedDevice = Self::IntelE1000Igb(0x1511);
	pub const IntelE1000Igb_82580_COPPER_DUAL: SupportedDevice = Self::IntelE1000Igb(0x1516);
	pub const IntelE1000Igb_82580_QUAD_FIBER: SupportedDevice = Self::IntelE1000Igb(0x1527);
	pub const IntelE1000Igb_I350_COPPER: SupportedDevice = Self::IntelE1000Igb(0x1521);
	pub const IntelE1000Igb_I350_FIBER: SupportedDevice = Self::IntelE1000Igb(0x1522);
	pub const IntelE1000Igb_I350_SERDES: SupportedDevice = Self::IntelE1000Igb(0x1523);
	pub const IntelE1000Igb_I350_SGMII: SupportedDevice = Self::IntelE1000Igb(0x1524);
	pub const IntelE1000Igb_I350_DA4: SupportedDevice = Self::IntelE1000Igb(0x1546);
	pub const IntelE1000Igb_I210_COPPER: SupportedDevice = Self::IntelE1000Igb(0x1533);
	pub const IntelE1000Igb_I210_COPPER_OEM1: SupportedDevice = Self::IntelE1000Igb(0x1534);
	pub const IntelE1000Igb_I210_COPPER_IT: SupportedDevice = Self::IntelE1000Igb(0x1535);
	pub const IntelE1000Igb_I210_FIBER: SupportedDevice = Self::IntelE1000Igb(0x1536);
	pub const IntelE1000Igb_I210_SERDES: SupportedDevice = Self::IntelE1000Igb(0x1537);
	pub const IntelE1000Igb_I210_SGMII: SupportedDevice = Self::IntelE1000Igb(0x1538);
	pub const IntelE1000Igb_I211_COPPER: SupportedDevice = Self::IntelE1000Igb(0x1539);
	pub const IntelE1000Igb_I354_BACKPLANE_1GBPS: SupportedDevice = Self::IntelE1000Igb(0x1F40);
	pub const IntelE1000Igb_I354_SGMII: SupportedDevice = Self::IntelE1000Igb(0x1F41);
	pub const IntelE1000Igb_I354_BACKPLANE_2_5GBPS: SupportedDevice = Self::IntelE1000Igb(0x1F45);
	pub const IntelE1000Igb_DH89XXCC_SGMII: SupportedDevice = Self::IntelE1000Igb(0x0438);
	pub const IntelE1000Igb_DH89XXCC_SERDES: SupportedDevice = Self::IntelE1000Igb(0x043A);
	pub const IntelE1000Igb_DH89XXCC_BACKPLANE: SupportedDevice = Self::IntelE1000Igb(0x043C);
	pub const IntelE1000Igb_DH89XXCC_SFP: SupportedDevice = Self::IntelE1000Igb(0x0440);
	
	pub const IntelFM10K_PF: SupportedDevice = Self::IntelFM10K(0x15A4);
	pub const IntelFM10K_VF: SupportedDevice = Self::IntelFM10K(0x15A5);
	pub const IntelFM10K_SDI_FM10420_QDA2: SupportedDevice = Self::IntelFM10K(0x15D0);
	pub const IntelFM10K_SDI_FM10420_DA2: SupportedDevice = Self::IntelFM10K(0x15D5);
	
	pub const Inteli40ePhysical_SFP_XL710: SupportedDevice = Self::Inteli40ePhysical(0x1572);
	pub const Inteli40ePhysical_QEMU: SupportedDevice = Self::Inteli40ePhysical(0x1574);
	pub const Inteli40ePhysical_KX_B: SupportedDevice = Self::Inteli40ePhysical(0x1580);
	pub const Inteli40ePhysical_KX_C: SupportedDevice = Self::Inteli40ePhysical(0x1581);
	pub const Inteli40ePhysical_QSFP_A: SupportedDevice = Self::Inteli40ePhysical(0x1583);
	pub const Inteli40ePhysical_QSFP_B: SupportedDevice = Self::Inteli40ePhysical(0x1584);
	pub const Inteli40ePhysical_QSFP_C: SupportedDevice = Self::Inteli40ePhysical(0x1585);
	pub const Inteli40ePhysical_10G_BASE_T: SupportedDevice = Self::Inteli40ePhysical(0x1586);
	pub const Inteli40ePhysical_20G_KR2: SupportedDevice = Self::Inteli40ePhysical(0x1587);
	pub const Inteli40ePhysical_20G_KR2_A: SupportedDevice = Self::Inteli40ePhysical(0x1588);
	pub const Inteli40ePhysical_10G_BASE_T4: SupportedDevice = Self::Inteli40ePhysical(0x1589);
	pub const Inteli40ePhysical_25G_B: SupportedDevice = Self::Inteli40ePhysical(0x158A);
	pub const Inteli40ePhysical_25G_SFP28: SupportedDevice = Self::Inteli40ePhysical(0x158B);
	pub const Inteli40ePhysical_X722_A0: SupportedDevice = Self::Inteli40ePhysical(0x374C);
	pub const Inteli40ePhysical_KX_X722: SupportedDevice = Self::Inteli40ePhysical(0x37CE);
	pub const Inteli40ePhysical_QSFP_X722: SupportedDevice = Self::Inteli40ePhysical(0x37CF);
	pub const Inteli40ePhysical_SFP_X722: SupportedDevice = Self::Inteli40ePhysical(0x37D0);
	pub const Inteli40ePhysical_1G_BASE_T_X722: SupportedDevice = Self::Inteli40ePhysical(0x37D1);
	pub const Inteli40ePhysical_10G_BASE_T_X722: SupportedDevice = Self::Inteli40ePhysical(0x37D2);
	pub const Inteli40ePhysical_SFP_I_X722: SupportedDevice = Self::Inteli40ePhysical(0x37D3);
	pub const Inteli40ePhysical_QSFP_I_X722: SupportedDevice = Self::Inteli40ePhysical(0x37D4);
	
	pub const Inteli40eVirtual_VF: SupportedDevice = Self::Inteli40eVirtual(0x154C);
	pub const Inteli40eVirtual_VF_HV: SupportedDevice = Self::Inteli40eVirtual(0x1571);
	pub const Inteli40eVirtual_X722_A0_VF: SupportedDevice = Self::Inteli40eVirtual(0x374D);
	pub const Inteli40eVirtual_X722_VF: SupportedDevice = Self::Inteli40eVirtual(0x37CD);
	pub const Inteli40eVirtual_X722_VF_HV: SupportedDevice = Self::Inteli40eVirtual(0x37D9);
	
	pub const IntelIxgbePhysical_82598: SupportedDevice = Self::IntelIxgbePhysical(0x10B6);
	pub const IntelIxgbePhysical_82598_BX: SupportedDevice = Self::IntelIxgbePhysical(0x1508);
	pub const IntelIxgbePhysical_82598AF_DUAL_PORT: SupportedDevice = Self::IntelIxgbePhysical(0x10C6);
	pub const IntelIxgbePhysical_82598AF_SINGLE_PORT: SupportedDevice = Self::IntelIxgbePhysical(0x10C7);
	pub const IntelIxgbePhysical_82598AT: SupportedDevice = Self::IntelIxgbePhysical(0x10C8);
	pub const IntelIxgbePhysical_82598AT2: SupportedDevice = Self::IntelIxgbePhysical(0x150B);
	pub const IntelIxgbePhysical_82598EB_SFP_LOM: SupportedDevice = Self::IntelIxgbePhysical(0x10DB);
	pub const IntelIxgbePhysical_82598EB_CX4: SupportedDevice = Self::IntelIxgbePhysical(0x10DD);
	pub const IntelIxgbePhysical_82598_CX4_DUAL_PORT: SupportedDevice = Self::IntelIxgbePhysical(0x10EC);
	pub const IntelIxgbePhysical_82598_DA_DUAL_PORT: SupportedDevice = Self::IntelIxgbePhysical(0x10F1);
	pub const IntelIxgbePhysical_82598_SR_DUAL_PORT_EM: SupportedDevice = Self::IntelIxgbePhysical(0x10E1);
	pub const IntelIxgbePhysical_82598EB_XF_LR: SupportedDevice = Self::IntelIxgbePhysical(0x10F4);
	pub const IntelIxgbePhysical_82599_KX4: SupportedDevice = Self::IntelIxgbePhysical(0x10F7);
	pub const IntelIxgbePhysical_82599_KX4_MEZZ: SupportedDevice = Self::IntelIxgbePhysical(0x1514);
	pub const IntelIxgbePhysical_82599_KR: SupportedDevice = Self::IntelIxgbePhysical(0x1517);
	pub const IntelIxgbePhysical_82599_COMBO_BACKPLANE: SupportedDevice = Self::IntelIxgbePhysical(0x10F8);
	pub const IntelIxgbePhysicalSubDevice_82599_KX4_KR_MEZZ: SupportedDevice = Self::IntelIxgbePhysicalSubDevice(0x000C);
	pub const IntelIxgbePhysical_82599_CX4: SupportedDevice = Self::IntelIxgbePhysical(0x10F9);
	pub const IntelIxgbePhysical_82599_SFP: SupportedDevice = Self::IntelIxgbePhysical(0x10FB);
	pub const IntelIxgbePhysicalSubDevice_82599_SFP: SupportedDevice = Self::IntelIxgbePhysicalSubDevice(0x11A9);
	pub const IntelIxgbePhysicalSubDevice_82599_RNDC: SupportedDevice = Self::IntelIxgbePhysicalSubDevice(0x1F72);
	pub const IntelIxgbePhysicalSubDevice_82599_560FLR: SupportedDevice = Self::IntelIxgbePhysicalSubDevice(0x17D0);
	pub const IntelIxgbePhysicalSubDevice_82599_ECNA_DP: SupportedDevice = Self::IntelIxgbePhysicalSubDevice(0x0470);
	pub const IntelIxgbePhysical_82599_BACKPLANE_FCOE: SupportedDevice = Self::IntelIxgbePhysical(0x152A);
	pub const IntelIxgbePhysical_82599_SFP_FCOE: SupportedDevice = Self::IntelIxgbePhysical(0x1529);
	pub const IntelIxgbePhysical_82599_SFP_EM: SupportedDevice = Self::IntelIxgbePhysical(0x1507);
	pub const IntelIxgbePhysical_82599_SFP_SF2: SupportedDevice = Self::IntelIxgbePhysical(0x154D);
	pub const IntelIxgbePhysical_82599_SFP_SF_QP: SupportedDevice = Self::IntelIxgbePhysical(0x154A);
	pub const IntelIxgbePhysical_82599_QSFP_SF_QP: SupportedDevice = Self::IntelIxgbePhysical(0x1558);
	pub const IntelIxgbePhysical_82599EN_SFP: SupportedDevice = Self::IntelIxgbePhysical(0x1557);
	pub const IntelIxgbePhysical_82599_XAUI_LOM: SupportedDevice = Self::IntelIxgbePhysical(0x10FC);
	pub const IntelIxgbePhysical_82599_T3_LOM: SupportedDevice = Self::IntelIxgbePhysical(0x151C);
	pub const IntelIxgbePhysical_82599_LS: SupportedDevice = Self::IntelIxgbePhysical(0x154F);
	pub const IntelIxgbePhysical_X540T: SupportedDevice = Self::IntelIxgbePhysical(0x1528);
	pub const IntelIxgbePhysical_X540T1: SupportedDevice = Self::IntelIxgbePhysical(0x1560);
	pub const IntelIxgbePhysical_X550EM_X_SFP: SupportedDevice = Self::IntelIxgbePhysical(0x15AC);
	pub const IntelIxgbePhysical_X550EM_X_10G_T: SupportedDevice = Self::IntelIxgbePhysical(0x15AD);
	pub const IntelIxgbePhysical_X550EM_X_1G_T: SupportedDevice = Self::IntelIxgbePhysical(0x15AE);
	pub const IntelIxgbePhysical_X550T: SupportedDevice = Self::IntelIxgbePhysical(0x1563);
	pub const IntelIxgbePhysical_X550T1: SupportedDevice = Self::IntelIxgbePhysical(0x15D1);
	pub const IntelIxgbePhysical_X550EM_A_KR: SupportedDevice = Self::IntelIxgbePhysical(0x15C2);
	pub const IntelIxgbePhysical_X550EM_A_KR_L: SupportedDevice = Self::IntelIxgbePhysical(0x15C3);
	pub const IntelIxgbePhysical_X550EM_A_SFP_N: SupportedDevice = Self::IntelIxgbePhysical(0x15C4);
	pub const IntelIxgbePhysical_X550EM_A_SGMII: SupportedDevice = Self::IntelIxgbePhysical(0x15C6);
	pub const IntelIxgbePhysical_X550EM_A_SGMII_L: SupportedDevice = Self::IntelIxgbePhysical(0x15C7);
	pub const IntelIxgbePhysical_X550EM_A_10G_T: SupportedDevice = Self::IntelIxgbePhysical(0x15C8);
	pub const IntelIxgbePhysical_X550EM_A_QSFP: SupportedDevice = Self::IntelIxgbePhysical(0x15CA);
	pub const IntelIxgbePhysical_X550EM_A_QSFP_N: SupportedDevice = Self::IntelIxgbePhysical(0x15CC);
	pub const IntelIxgbePhysical_X550EM_A_SFP: SupportedDevice = Self::IntelIxgbePhysical(0x15CE);
	pub const IntelIxgbePhysical_X550EM_A_1G_T: SupportedDevice = Self::IntelIxgbePhysical(0x15E4);
	pub const IntelIxgbePhysical_X550EM_A_1G_T_L: SupportedDevice = Self::IntelIxgbePhysical(0x15E5);
	pub const IntelIxgbePhysical_X550EM_X_KX4: SupportedDevice = Self::IntelIxgbePhysical(0x15AA);
	pub const IntelIxgbePhysical_X550EM_X_KR: SupportedDevice = Self::IntelIxgbePhysical(0x15AB);
	pub const IntelIxgbePhysical_82599_BYPASS: SupportedDevice = Self::IntelIxgbePhysical(0x155D);
	
	pub const IntelIxgbeVirtual_82599_VF: SupportedDevice = Self::IntelIxgbeVirtual(0x10ED);
	pub const IntelIxgbeVirtual_82599_VF_HV: SupportedDevice = Self::IntelIxgbeVirtual(0x152E);
	pub const IntelIxgbeVirtual_X540_VF: SupportedDevice = Self::IntelIxgbeVirtual(0x1515);
	pub const IntelIxgbeVirtual_X540_VF_HV: SupportedDevice = Self::IntelIxgbeVirtual(0x1530);
	pub const IntelIxgbeVirtual_X550_VF_HV: SupportedDevice = Self::IntelIxgbeVirtual(0x1564);
	pub const IntelIxgbeVirtual_X550_VF: SupportedDevice = Self::IntelIxgbeVirtual(0x1565);
	pub const IntelIxgbeVirtual_X550EM_A_VF: SupportedDevice = Self::IntelIxgbeVirtual(0x15C5);
	pub const IntelIxgbeVirtual_X550EM_A_VF_HV: SupportedDevice = Self::IntelIxgbeVirtual(0x15B4);
	pub const IntelIxgbeVirtual_X550EM_X_VF: SupportedDevice = Self::IntelIxgbeVirtual(0x15A8);
	pub const IntelIxgbeVirtual_X550EM_X_VF_HV: SupportedDevice = Self::IntelIxgbeVirtual(0x15A9);
	
	pub const IntelQatSymmetricCrypto: SupportedDevice = Self::IntelQat(0x0443);
	
	pub const MellanoxMlx4_ConnectX3: SupportedDevice = Self::MellanoxMlx4(0x1003);
	pub const MellanoxMlx4_ConnectX3_VF: SupportedDevice = Self::MellanoxMlx4(0x1004);
	pub const MellanoxMlx4_ConnectX3_PRO: SupportedDevice = Self::MellanoxMlx4(0x1007);

	pub const MellanoxMlx5_ConnectX4: SupportedDevice = Self::MellanoxMlx5(0x1013);
	pub const MellanoxMlx5_ConnectX4_VF: SupportedDevice = Self::MellanoxMlx5(0x1014);
	pub const MellanoxMlx5_ConnectX4_LX: SupportedDevice = Self::MellanoxMlx5(0x1015);
	pub const MellanoxMlx5_ConnectX4_LX_VF: SupportedDevice = Self::MellanoxMlx5(0x1016);

	pub const NetCopeSzedata2_Combo80G: SupportedDevice = Self::NetCopeSzedata2(0xcb80);
	pub const NetCopeSzedata2_Combo100G: SupportedDevice = Self::NetCopeSzedata2(0xc1c1);
	pub const NetCopeSzedata2_Combo100G2: SupportedDevice = Self::NetCopeSzedata2(0xc2c1);

	pub const NetronomeNfp6000_PF: SupportedDevice = Self::NetronomeNfp6000(0x6000);
	pub const NetronomeNfp6000_VF: SupportedDevice = Self::NetronomeNfp6000(0x6003);

	pub const QLogicQedePhysical_NX2_57980E: SupportedDevice = Self::QLogicQedePhysical(0x1634);
	pub const QLogicQedePhysical_NX2_57980S: SupportedDevice = Self::QLogicQedePhysical(0x1629);
	pub const QLogicQedePhysical_57980S_40: SupportedDevice = Self::QLogicQedePhysical(0x1634);
	pub const QLogicQedePhysical_57980S_25: SupportedDevice = Self::QLogicQedePhysical(0x1656);
	pub const QLogicQedePhysical_57980S_100: SupportedDevice = Self::QLogicQedePhysical(0x1644);

	pub const QLogicQedeVirtual_NX2_VF: SupportedDevice = Self::QLogicQedeVirtual(0x1630);
	pub const QLogicQedeVirtual_57980S_IOV: SupportedDevice = Self::QLogicQedeVirtual(0x1664);

	pub const VirtIoNetwork_: SupportedDevice = Self::VirtIoNetwork(0x1000);
	
	pub const VmWareVmxNet3_: SupportedDevice = Self::VmWareVmxNet3(0x07B0);
	
	const fn AmazonEna(deviceId: u16) -> Self
	{
		Self::Amazon(DriverIdentifier::AmazonEna, deviceId)
	}
	
	const fn BroadcomBnx2x(deviceId: u16) -> Self
	{
		Self::Broadcom(DriverIdentifier::BroadcomBnx2x, deviceId)
	}
	
	const fn BroadcomBnxt(deviceId: u16) -> Self
	{
		Self::Broadcom(DriverIdentifier::BroadcomBnxt, deviceId)
	}
	
	const fn ChelsioT5(deviceId: u16) -> Self
	{
		Self::Chelsio(DriverIdentifier::ChelsioT5, deviceId)
	}
	
	const fn CiscoVic(deviceId: u16) -> Self
	{
		Self::Cisco(DriverIdentifier::CiscoVic, deviceId)
	}
	
	const fn IntelE1000Em(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::IntelE1000Em, deviceId)
	}
	
	const fn IntelE1000Igb(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::IntelE1000Igb, deviceId)
	}
	
	const fn IntelFM10K(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::IntelFM10K, deviceId)
	}
	
	const fn Inteli40ePhysical(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::Inteli40ePhysical, deviceId)
	}
	
	const fn Inteli40eVirtual(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::Inteli40eVirtual, deviceId)
	}
	
	const fn IntelIxgbePhysical(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::IntelIxgbePhysical, deviceId)
	}
	
	const fn IntelIxgbePhysicalSubDevice(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::IntelIxgbePhysicalSubDevice, deviceId)
	}
	
	const fn IntelIxgbeVirtual(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::IntelIxgbeVirtual, deviceId)
	}
	
	const fn IntelQat(deviceId: u16) -> Self
	{
		Self::Intel(DriverIdentifier::IntelQat, deviceId)
	}
	
	const fn MellanoxMlx4(deviceId: u16) -> Self
	{
		Self::Mellanox(DriverIdentifier::MellanoxMlx4, deviceId)
	}
	
	const fn MellanoxMlx5(deviceId: u16) -> Self
	{
		Self::Mellanox(DriverIdentifier::MellanoxMlx5, deviceId)
	}
	
	const fn NetCopeSzedata2(deviceId: u16) -> Self
	{
		Self::NetCope(DriverIdentifier::NetCopeSzedata2, deviceId)
	}
	
	const fn NetronomeNfp6000(deviceId: u16) -> Self
	{
		Self::Netronome(DriverIdentifier::NetronomeNfp6000, deviceId)
	}
	
	const fn QLogicQedePhysical(deviceId: u16) -> Self
	{
		Self::QLogic(DriverIdentifier::QLogicQedePhysical, deviceId)
	}
	
	const fn QLogicQedeVirtual(deviceId: u16) -> Self
	{
		Self::QLogic(DriverIdentifier::QLogicQedeVirtual, deviceId)
	}
	
	const fn VirtIoNetwork(deviceId: u16) -> Self
	{
		Self::VirtIO(DriverIdentifier::VirtIoNetwork, deviceId)
	}
	
	const fn VmWareVmxNet3(deviceId: u16) -> Self
	{
		Self::VmWare(DriverIdentifier::VmWareVmxNet3, deviceId)
	}
	
	const fn Amazon(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::Amazon, driver, deviceId)
	}

	const fn Broadcom(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::Broadcom, driver, deviceId)
	}
	
	const fn Chelsio(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::Chelsio, driver, deviceId)
	}
	
	const fn Cisco(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::Cisco, driver, deviceId)
	}
	
	const fn Intel(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::Intel, driver, deviceId)
	}
	
	const fn Mellanox(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::Mellanox, driver, deviceId)
	}
	
	const fn NetCope(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::NetCope, driver, deviceId)
	}
	
	const fn Netronome(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::Netronome, driver, deviceId)
	}
	
	const fn QLogic(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::QLogic, driver, deviceId)
	}
	
	const fn VirtIO(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::VirtIO, driver, deviceId)
	}
	
	const fn VmWare(driver: DriverIdentifier, deviceId: u16) -> Self
	{
		Self::supportedDevice(VendorId::VmWare, driver, deviceId)
	}
	
	const fn supportedDevice(vendor: VendorId, driver: DriverIdentifier, deviceId: u16) -> Self
	{
		SupportedDevice
		{
			vendor: vendor,
			driver: driver,
			device: DeviceId(deviceId),
		}
	}
}
