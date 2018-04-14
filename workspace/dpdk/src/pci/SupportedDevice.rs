// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A PCI device known to be supported by DPDK.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SupportedDevice
{
	pci_vendor_identifier: PciVendorIdentifier,
	supported_driver_identifier: SupportedDriverIdentifier,
	pci_device_identifier: PciDeviceIdentifier,
}

impl SupportedDevice
{
	/// Does this support device match a `rte_pci_id`?
	///
	/// This test does not return true is Any (0xFFFF) is set for a vendor or device id.
	#[inline(always)]
	pub fn is_exactly_rte_pci_id(&self, other: &rte_pci_id) -> bool
	{
		self.pci_vendor_identifier.is(other.vendor_id) && self.device.is(other.device_id)
	}
	
	/// Underlying ethernet ports, if any.
	#[inline(always)]
	pub fn underlying_ethernet_port(&self) -> Vec<(EthernetPort, rte_eth_dev)>
	{
		let mut matches = EthernetPort::new_vec_with_capacity_for_all_attached_ports();

		let mut port_identifier = 0;
		while port_identifier < EthernetPort::MaximumEthernetPorts
		{
			if let Some(ethernet_port) = EthernetPort::new(port_identifier as u8)
			{
				let underlying_rte_eth_dev = ethernet_port.underlying_ethernet_device();
				if !underlying_rte_eth_dev.device.is_null()
				{
					unsafe
					{
						let pci_device = *(rust_RTE_DEV_TO_PCI(underlying_rte_eth_dev.device));
						if self.is_exactly_rte_pci_id(&pci_device.id)
						{
							matches.push((ethernet_port, underlying_rte_eth_dev));
						}
					}
				}
			}
			port_identifier += 1;
		}

		matches
	}
}

#[allow(missing_docs)]
impl SupportedDevice
{
	pub const AmazonEna_VF: Self = Self::AmazonEna(0xEC20);
	pub const AmazonEna_LLQ_VF: Self = Self::AmazonEna(0xEC21);

	pub const BroadcomBnx2x_ChipNumber_57711: Self = Self::BroadcomBnx2x(0x164f);
	pub const BroadcomBnx2x_ChipNumber_57711E: Self = Self::BroadcomBnx2x(0x1650);
	pub const BroadcomBnx2x_ChipNumber_57712: Self = Self::BroadcomBnx2x(0x1662);
	pub const BroadcomBnx2x_ChipNumber_57712_MF: Self = Self::BroadcomBnx2x(0x1663);
	pub const BroadcomBnx2x_ChipNumber_57712_VF: Self = Self::BroadcomBnx2x(0x166f);
	pub const BroadcomBnx2x_ChipNumber_57800: Self = Self::BroadcomBnx2x(0x168a);
	pub const BroadcomBnx2x_ChipNumber_57800_MF: Self = Self::BroadcomBnx2x(0x16a5);
	pub const BroadcomBnx2x_ChipNumber_57800_VF: Self = Self::BroadcomBnx2x(0x16a9);
	pub const BroadcomBnx2x_ChipNumber_57810: Self = Self::BroadcomBnx2x(0x168e);
	pub const BroadcomBnx2x_ChipNumber_57810_MF: Self = Self::BroadcomBnx2x(0x16ae);
	pub const BroadcomBnx2x_ChipNumber_57810_VF: Self = Self::BroadcomBnx2x(0x16af);
	pub const BroadcomBnx2x_ChipNumber_57811: Self = Self::BroadcomBnx2x(0x163d);
	pub const BroadcomBnx2x_ChipNumber_57811_MF: Self = Self::BroadcomBnx2x(0x163e);
	pub const BroadcomBnx2x_ChipNumber_57811_VF: Self = Self::BroadcomBnx2x(0x163f);
	pub const BroadcomBnx2x_ChipNumber_57840_2_20: Self = Self::BroadcomBnx2x(0x16a2);
	pub const BroadcomBnx2x_ChipNumber_57840_4_10: Self = Self::BroadcomBnx2x(0x16a1);
	pub const BroadcomBnx2x_ChipNumber_57840_MF: Self = Self::BroadcomBnx2x(0x16a4);
	pub const BroadcomBnx2x_ChipNumber_57840_OBS: Self = Self::BroadcomBnx2x(0x168d);
	pub const BroadcomBnx2x_ChipNumber_57840_OBS_MF: Self = Self::BroadcomBnx2x(0x16ab);
	pub const BroadcomBnx2x_ChipNumber_57840_VF: Self = Self::BroadcomBnx2x(0x16ad);

	pub const BroadcomBnxt_ChipNumber_57301: Self = Self::BroadcomBnxt(0x16c8);
	pub const BroadcomBnxt_ChipNumber_57302: Self = Self::BroadcomBnxt(0x16c9);
	pub const BroadcomBnxt_ChipNumber_57304_PF: Self = Self::BroadcomBnxt(0x16ca);
	pub const BroadcomBnxt_ChipNumber_57304_VF: Self = Self::BroadcomBnxt(0x16cb);
	pub const BroadcomBnxt_ChipNumber_57402: Self = Self::BroadcomBnxt(0x16d0);
	pub const BroadcomBnxt_ChipNumber_57404: Self = Self::BroadcomBnxt(0x16d1);
	pub const BroadcomBnxt_ChipNumber_57406_PF: Self = Self::BroadcomBnxt(0x16d2);
	pub const BroadcomBnxt_ChipNumber_57406_VF: Self = Self::BroadcomBnxt(0x16d3);
	pub const BroadcomBnxt_ChipNumber_57406_MF: Self = Self::BroadcomBnxt(0x16d4);
	pub const BroadcomBnxt_ChipNumber_57314: Self = Self::BroadcomBnxt(0x16df);

	pub const ChelsioT5_T580_dbg: Self = Self::ChelsioT5(0x5000);
	pub const ChelsioT5_T520_cr: Self = Self::ChelsioT5(0x5001);
	pub const ChelsioT5_T522_cr: Self = Self::ChelsioT5(0x5002);
	pub const ChelsioT5_T540_cr: Self = Self::ChelsioT5(0x5003);
	pub const ChelsioT5_T520_bch: Self = Self::ChelsioT5(0x5004);
	pub const ChelsioT5_T540_bch: Self = Self::ChelsioT5(0x5005);
	pub const ChelsioT5_T540_ch: Self = Self::ChelsioT5(0x5006);
	pub const ChelsioT5_T520_so: Self = Self::ChelsioT5(0x5007);
	pub const ChelsioT5_T520_cx: Self = Self::ChelsioT5(0x5008);
	pub const ChelsioT5_T520_bt: Self = Self::ChelsioT5(0x5009);
	pub const ChelsioT5_T504_bt: Self = Self::ChelsioT5(0x500a);
	pub const ChelsioT5_B520_sr: Self = Self::ChelsioT5(0x500b);
	pub const ChelsioT5_B504_bt: Self = Self::ChelsioT5(0x500c);
	pub const ChelsioT5_T580_cr: Self = Self::ChelsioT5(0x500d);
	pub const ChelsioT5_T540_LP_cr: Self = Self::ChelsioT5(0x500e);
	pub const ChelsioT5_T580_LP_cr: Self = Self::ChelsioT5(0x5010);
	pub const ChelsioT5_T520_LL_cr: Self = Self::ChelsioT5(0x5011);
	pub const ChelsioT5_T560_cr: Self = Self::ChelsioT5(0x5012);
	pub const ChelsioT5_T580_chr: Self = Self::ChelsioT5(0x5013);
	pub const ChelsioT5_T580_so: Self = Self::ChelsioT5(0x5014);
	pub const ChelsioT5_T502_bt: Self = Self::ChelsioT5(0x5015);
	pub const ChelsioT5_Custom_T540_cr: Self = Self::ChelsioT5(0x5080);
	pub const ChelsioT5_Custom_T540_LL_cr: Self = Self::ChelsioT5(0x5081);
	pub const ChelsioT5_Custom_T504_cr: Self = Self::ChelsioT5(0x5082);
	pub const ChelsioT5_Custom_T540_LP_CR: Self = Self::ChelsioT5(0x5083);
	pub const ChelsioT5_Custom_T580_cr: Self = Self::ChelsioT5(0x5084);
	pub const ChelsioT5_Custom_3x_T580_CR: Self = Self::ChelsioT5(0x5085);
	pub const ChelsioT5_Custom_2x_T580_CR: Self = Self::ChelsioT5(0x5086);
	pub const ChelsioT5_Custom_T580_CR: Self = Self::ChelsioT5(0x5087);
	pub const ChelsioT5_Custom_T570_CR: Self = Self::ChelsioT5(0x5088);
	pub const ChelsioT5_Custom_T520_CR: Self = Self::ChelsioT5(0x5089);
	pub const ChelsioT5_Custom_T540_CR: Self = Self::ChelsioT5(0x5090);
	pub const ChelsioT5_Custom_T522_CR: Self = Self::ChelsioT5(0x5091);
	pub const ChelsioT5_Custom_T520_CR_alt: Self = Self::ChelsioT5(0x5092);

	pub const CiscoVic_Enet: Self = Self::CiscoVic(0x0043);
	pub const CiscoVic_Enet_VF: Self = Self::CiscoVic(0x0071);

	pub const IntelE1000Em_82540EM: Self = Self::IntelE1000Em(0x100E);
	pub const IntelE1000Em_82545EM_COPPER: Self = Self::IntelE1000Em(0x100F);
	pub const IntelE1000Em_82545EM_FIBER: Self = Self::IntelE1000Em(0x1011);
	pub const IntelE1000Em_82546EB_COPPER: Self = Self::IntelE1000Em(0x1010);
	pub const IntelE1000Em_82546EB_FIBER: Self = Self::IntelE1000Em(0x1012);
	pub const IntelE1000Em_82546EB_QUAD_COPPER: Self = Self::IntelE1000Em(0x101D);
	pub const IntelE1000Em_82571EB_COPPER: Self = Self::IntelE1000Em(0x105E);
	pub const IntelE1000Em_82571EB_FIBER: Self = Self::IntelE1000Em(0x105F);
	pub const IntelE1000Em_82571EB_SERDES: Self = Self::IntelE1000Em(0x1060);
	pub const IntelE1000Em_82571EB_SERDES_DUAL: Self = Self::IntelE1000Em(0x10D9);
	pub const IntelE1000Em_82571EB_SERDES_QUAD: Self = Self::IntelE1000Em(0x10DA);
	pub const IntelE1000Em_82571EB_QUAD_COPPER: Self = Self::IntelE1000Em(0x10A4);
	pub const IntelE1000Em_82571PT_QUAD_COPPER: Self = Self::IntelE1000Em(0x10D5);
	pub const IntelE1000Em_82571EB_QUAD_FIBER: Self = Self::IntelE1000Em(0x10A5);
	pub const IntelE1000Em_82571EB_QUAD_COPPER_LP: Self = Self::IntelE1000Em(0x10BC);
	pub const IntelE1000Em_82572EI_COPPER: Self = Self::IntelE1000Em(0x107D);
	pub const IntelE1000Em_82572EI_FIBER: Self = Self::IntelE1000Em(0x107E);
	pub const IntelE1000Em_82572EI_SERDES: Self = Self::IntelE1000Em(0x107F);
	pub const IntelE1000Em_82572EI: Self = Self::IntelE1000Em(0x10B9);
	pub const IntelE1000Em_82573L: Self = Self::IntelE1000Em(0x109A);
	pub const IntelE1000Em_82574L: Self = Self::IntelE1000Em(0x10D3);
	pub const IntelE1000Em_82574LA: Self = Self::IntelE1000Em(0x10F6);
	pub const IntelE1000Em_82583V: Self = Self::IntelE1000Em(0x150C);
	pub const IntelE1000Em_PCH_LPT_I217_LM: Self = Self::IntelE1000Em(0x153A);
	pub const IntelE1000Em_PCH_LPT_I217_V: Self = Self::IntelE1000Em(0x153B);
	pub const IntelE1000Em_PCH_LPTLP_I218_LM: Self = Self::IntelE1000Em(0x155A);
	pub const IntelE1000Em_PCH_LPTLP_I218_V: Self = Self::IntelE1000Em(0x1559);
	pub const IntelE1000Em_PCH_I218_LM2: Self = Self::IntelE1000Em(0x15A0);
	pub const IntelE1000Em_PCH_I218_V2: Self = Self::IntelE1000Em(0x15A1);
	pub const IntelE1000Em_PCH_I218_LM3: Self = Self::IntelE1000Em(0x15A2);
	pub const IntelE1000Em_PCH_I218_V3: Self = Self::IntelE1000Em(0x15A3);

	pub const IntelE1000Igb_82576: Self = Self::IntelE1000Igb(0x10C9);
	pub const IntelE1000Igb_82576_FIBER: Self = Self::IntelE1000Igb(0x10E6);
	pub const IntelE1000Igb_82576_SERDES: Self = Self::IntelE1000Igb(0x10E7);
	pub const IntelE1000Igb_82576_QUAD_COPPER: Self = Self::IntelE1000Igb(0x10E8);
	pub const IntelE1000Igb_82576_QUAD_COPPER_ET2: Self = Self::IntelE1000Igb(0x1526);
	pub const IntelE1000Igb_82576_NS: Self = Self::IntelE1000Igb(0x150A);
	pub const IntelE1000Igb_82576_NS_SERDES: Self = Self::IntelE1000Igb(0x1518);
	pub const IntelE1000Igb_82576_SERDES_QUAD: Self = Self::IntelE1000Igb(0x150D);
	pub const IntelE1000Igb_82575EB_COPPER: Self = Self::IntelE1000Igb(0x10A7);
	pub const IntelE1000Igb_82575EB_FIBER_SERDES: Self = Self::IntelE1000Igb(0x10A9);
	pub const IntelE1000Igb_82575GB_QUAD_COPPER: Self = Self::IntelE1000Igb(0x10D6);
	pub const IntelE1000Igb_82580_COPPER: Self = Self::IntelE1000Igb(0x150E);
	pub const IntelE1000Igb_82580_FIBER: Self = Self::IntelE1000Igb(0x150F);
	pub const IntelE1000Igb_82580_SERDES: Self = Self::IntelE1000Igb(0x1510);
	pub const IntelE1000Igb_82580_SGMII: Self = Self::IntelE1000Igb(0x1511);
	pub const IntelE1000Igb_82580_COPPER_DUAL: Self = Self::IntelE1000Igb(0x1516);
	pub const IntelE1000Igb_82580_QUAD_FIBER: Self = Self::IntelE1000Igb(0x1527);
	pub const IntelE1000Igb_I350_COPPER: Self = Self::IntelE1000Igb(0x1521);
	pub const IntelE1000Igb_I350_FIBER: Self = Self::IntelE1000Igb(0x1522);
	pub const IntelE1000Igb_I350_SERDES: Self = Self::IntelE1000Igb(0x1523);
	pub const IntelE1000Igb_I350_SGMII: Self = Self::IntelE1000Igb(0x1524);
	pub const IntelE1000Igb_I350_DA4: Self = Self::IntelE1000Igb(0x1546);
	pub const IntelE1000Igb_I210_COPPER: Self = Self::IntelE1000Igb(0x1533);
	pub const IntelE1000Igb_I210_COPPER_OEM1: Self = Self::IntelE1000Igb(0x1534);
	pub const IntelE1000Igb_I210_COPPER_IT: Self = Self::IntelE1000Igb(0x1535);
	pub const IntelE1000Igb_I210_FIBER: Self = Self::IntelE1000Igb(0x1536);
	pub const IntelE1000Igb_I210_SERDES: Self = Self::IntelE1000Igb(0x1537);
	pub const IntelE1000Igb_I210_SGMII: Self = Self::IntelE1000Igb(0x1538);
	pub const IntelE1000Igb_I211_COPPER: Self = Self::IntelE1000Igb(0x1539);
	pub const IntelE1000Igb_I354_BACKPLANE_1GBPS: Self = Self::IntelE1000Igb(0x1F40);
	pub const IntelE1000Igb_I354_SGMII: Self = Self::IntelE1000Igb(0x1F41);
	pub const IntelE1000Igb_I354_BACKPLANE_2_5GBPS: Self = Self::IntelE1000Igb(0x1F45);
	pub const IntelE1000Igb_DH89XXCC_SGMII: Self = Self::IntelE1000Igb(0x0438);
	pub const IntelE1000Igb_DH89XXCC_SERDES: Self = Self::IntelE1000Igb(0x043A);
	pub const IntelE1000Igb_DH89XXCC_BACKPLANE: Self = Self::IntelE1000Igb(0x043C);
	pub const IntelE1000Igb_DH89XXCC_SFP: Self = Self::IntelE1000Igb(0x0440);

	pub const IntelFM10K_PF: Self = Self::IntelFM10K(0x15A4);
	pub const IntelFM10K_VF: Self = Self::IntelFM10K(0x15A5);
	pub const IntelFM10K_SDI_FM10420_QDA2: Self = Self::IntelFM10K(0x15D0);
	pub const IntelFM10K_SDI_FM10420_DA2: Self = Self::IntelFM10K(0x15D5);

	pub const Inteli40ePhysical_SFP_XL710: Self = Self::Inteli40ePhysical(0x1572);
	pub const Inteli40ePhysical_QEMU: Self = Self::Inteli40ePhysical(0x1574);
	pub const Inteli40ePhysical_KX_B: Self = Self::Inteli40ePhysical(0x1580);
	pub const Inteli40ePhysical_KX_C: Self = Self::Inteli40ePhysical(0x1581);
	pub const Inteli40ePhysical_QSFP_A: Self = Self::Inteli40ePhysical(0x1583);
	pub const Inteli40ePhysical_QSFP_B: Self = Self::Inteli40ePhysical(0x1584);
	pub const Inteli40ePhysical_QSFP_C: Self = Self::Inteli40ePhysical(0x1585);
	pub const Inteli40ePhysical_10G_BASE_T: Self = Self::Inteli40ePhysical(0x1586);
	pub const Inteli40ePhysical_20G_KR2: Self = Self::Inteli40ePhysical(0x1587);
	pub const Inteli40ePhysical_20G_KR2_A: Self = Self::Inteli40ePhysical(0x1588);
	pub const Inteli40ePhysical_10G_BASE_T4: Self = Self::Inteli40ePhysical(0x1589);
	pub const Inteli40ePhysical_25G_B: Self = Self::Inteli40ePhysical(0x158A);
	pub const Inteli40ePhysical_25G_SFP28: Self = Self::Inteli40ePhysical(0x158B);
	pub const Inteli40ePhysical_X722_A0: Self = Self::Inteli40ePhysical(0x374C);
	pub const Inteli40ePhysical_KX_X722: Self = Self::Inteli40ePhysical(0x37CE);
	pub const Inteli40ePhysical_QSFP_X722: Self = Self::Inteli40ePhysical(0x37CF);
	pub const Inteli40ePhysical_SFP_X722: Self = Self::Inteli40ePhysical(0x37D0);
	pub const Inteli40ePhysical_1G_BASE_T_X722: Self = Self::Inteli40ePhysical(0x37D1);
	pub const Inteli40ePhysical_10G_BASE_T_X722: Self = Self::Inteli40ePhysical(0x37D2);
	pub const Inteli40ePhysical_SFP_I_X722: Self = Self::Inteli40ePhysical(0x37D3);
	pub const Inteli40ePhysical_QSFP_I_X722: Self = Self::Inteli40ePhysical(0x37D4);

	pub const Inteli40eVirtual_VF: Self = Self::Inteli40eVirtual(0x154C);
	pub const Inteli40eVirtual_VF_HV: Self = Self::Inteli40eVirtual(0x1571);
	pub const Inteli40eVirtual_X722_A0_VF: Self = Self::Inteli40eVirtual(0x374D);
	pub const Inteli40eVirtual_X722_VF: Self = Self::Inteli40eVirtual(0x37CD);
	pub const Inteli40eVirtual_X722_VF_HV: Self = Self::Inteli40eVirtual(0x37D9);

	pub const IntelIxgbePhysical_82598: Self = Self::IntelIxgbePhysical(0x10B6);
	pub const IntelIxgbePhysical_82598_BX: Self = Self::IntelIxgbePhysical(0x1508);
	pub const IntelIxgbePhysical_82598AF_DUAL_PORT: Self = Self::IntelIxgbePhysical(0x10C6);
	pub const IntelIxgbePhysical_82598AF_SINGLE_PORT: Self = Self::IntelIxgbePhysical(0x10C7);
	pub const IntelIxgbePhysical_82598AT: Self = Self::IntelIxgbePhysical(0x10C8);
	pub const IntelIxgbePhysical_82598AT2: Self = Self::IntelIxgbePhysical(0x150B);
	pub const IntelIxgbePhysical_82598EB_SFP_LOM: Self = Self::IntelIxgbePhysical(0x10DB);
	pub const IntelIxgbePhysical_82598EB_CX4: Self = Self::IntelIxgbePhysical(0x10DD);
	pub const IntelIxgbePhysical_82598_CX4_DUAL_PORT: Self = Self::IntelIxgbePhysical(0x10EC);
	pub const IntelIxgbePhysical_82598_DA_DUAL_PORT: Self = Self::IntelIxgbePhysical(0x10F1);
	pub const IntelIxgbePhysical_82598_SR_DUAL_PORT_EM: Self = Self::IntelIxgbePhysical(0x10E1);
	pub const IntelIxgbePhysical_82598EB_XF_LR: Self = Self::IntelIxgbePhysical(0x10F4);
	pub const IntelIxgbePhysical_82599_KX4: Self = Self::IntelIxgbePhysical(0x10F7);
	pub const IntelIxgbePhysical_82599_KX4_MEZZ: Self = Self::IntelIxgbePhysical(0x1514);
	pub const IntelIxgbePhysical_82599_KR: Self = Self::IntelIxgbePhysical(0x1517);
	pub const IntelIxgbePhysical_82599_COMBO_BACKPLANE: Self = Self::IntelIxgbePhysical(0x10F8);
	pub const IntelIxgbePhysicalSubDevice_82599_KX4_KR_MEZZ: Self = Self::IntelIxgbePhysicalSubDevice(0x000C);
	pub const IntelIxgbePhysical_82599_CX4: Self = Self::IntelIxgbePhysical(0x10F9);
	pub const IntelIxgbePhysical_82599_SFP: Self = Self::IntelIxgbePhysical(0x10FB);
	pub const IntelIxgbePhysicalSubDevice_82599_SFP: Self = Self::IntelIxgbePhysicalSubDevice(0x11A9);
	pub const IntelIxgbePhysicalSubDevice_82599_RNDC: Self = Self::IntelIxgbePhysicalSubDevice(0x1F72);
	pub const IntelIxgbePhysicalSubDevice_82599_560FLR: Self = Self::IntelIxgbePhysicalSubDevice(0x17D0);
	pub const IntelIxgbePhysicalSubDevice_82599_ECNA_DP: Self = Self::IntelIxgbePhysicalSubDevice(0x0470);
	pub const IntelIxgbePhysical_82599_BACKPLANE_FCOE: Self = Self::IntelIxgbePhysical(0x152A);
	pub const IntelIxgbePhysical_82599_SFP_FCOE: Self = Self::IntelIxgbePhysical(0x1529);
	pub const IntelIxgbePhysical_82599_SFP_EM: Self = Self::IntelIxgbePhysical(0x1507);
	pub const IntelIxgbePhysical_82599_SFP_SF2: Self = Self::IntelIxgbePhysical(0x154D);
	pub const IntelIxgbePhysical_82599_SFP_SF_QP: Self = Self::IntelIxgbePhysical(0x154A);
	pub const IntelIxgbePhysical_82599_QSFP_SF_QP: Self = Self::IntelIxgbePhysical(0x1558);
	pub const IntelIxgbePhysical_82599EN_SFP: Self = Self::IntelIxgbePhysical(0x1557);
	pub const IntelIxgbePhysical_82599_XAUI_LOM: Self = Self::IntelIxgbePhysical(0x10FC);
	pub const IntelIxgbePhysical_82599_T3_LOM: Self = Self::IntelIxgbePhysical(0x151C);
	pub const IntelIxgbePhysical_82599_LS: Self = Self::IntelIxgbePhysical(0x154F);
	pub const IntelIxgbePhysical_X540T: Self = Self::IntelIxgbePhysical(0x1528);
	pub const IntelIxgbePhysical_X540T1: Self = Self::IntelIxgbePhysical(0x1560);
	pub const IntelIxgbePhysical_X550EM_X_SFP: Self = Self::IntelIxgbePhysical(0x15AC);
	pub const IntelIxgbePhysical_X550EM_X_10G_T: Self = Self::IntelIxgbePhysical(0x15AD);
	pub const IntelIxgbePhysical_X550EM_X_1G_T: Self = Self::IntelIxgbePhysical(0x15AE);
	pub const IntelIxgbePhysical_X550T: Self = Self::IntelIxgbePhysical(0x1563);
	pub const IntelIxgbePhysical_X550T1: Self = Self::IntelIxgbePhysical(0x15D1);
	pub const IntelIxgbePhysical_X550EM_A_KR: Self = Self::IntelIxgbePhysical(0x15C2);
	pub const IntelIxgbePhysical_X550EM_A_KR_L: Self = Self::IntelIxgbePhysical(0x15C3);
	pub const IntelIxgbePhysical_X550EM_A_SFP_N: Self = Self::IntelIxgbePhysical(0x15C4);
	pub const IntelIxgbePhysical_X550EM_A_SGMII: Self = Self::IntelIxgbePhysical(0x15C6);
	pub const IntelIxgbePhysical_X550EM_A_SGMII_L: Self = Self::IntelIxgbePhysical(0x15C7);
	pub const IntelIxgbePhysical_X550EM_A_10G_T: Self = Self::IntelIxgbePhysical(0x15C8);
	pub const IntelIxgbePhysical_X550EM_A_QSFP: Self = Self::IntelIxgbePhysical(0x15CA);
	pub const IntelIxgbePhysical_X550EM_A_QSFP_N: Self = Self::IntelIxgbePhysical(0x15CC);
	pub const IntelIxgbePhysical_X550EM_A_SFP: Self = Self::IntelIxgbePhysical(0x15CE);
	pub const IntelIxgbePhysical_X550EM_A_1G_T: Self = Self::IntelIxgbePhysical(0x15E4);
	pub const IntelIxgbePhysical_X550EM_A_1G_T_L: Self = Self::IntelIxgbePhysical(0x15E5);
	pub const IntelIxgbePhysical_X550EM_X_KX4: Self = Self::IntelIxgbePhysical(0x15AA);
	pub const IntelIxgbePhysical_X550EM_X_KR: Self = Self::IntelIxgbePhysical(0x15AB);
	pub const IntelIxgbePhysical_82599_BYPASS: Self = Self::IntelIxgbePhysical(0x155D);

	pub const IntelIxgbeVirtual_82599_VF: Self = Self::IntelIxgbeVirtual(0x10ED);
	pub const IntelIxgbeVirtual_82599_VF_HV: Self = Self::IntelIxgbeVirtual(0x152E);
	pub const IntelIxgbeVirtual_X540_VF: Self = Self::IntelIxgbeVirtual(0x1515);
	pub const IntelIxgbeVirtual_X540_VF_HV: Self = Self::IntelIxgbeVirtual(0x1530);
	pub const IntelIxgbeVirtual_X550_VF_HV: Self = Self::IntelIxgbeVirtual(0x1564);
	pub const IntelIxgbeVirtual_X550_VF: Self = Self::IntelIxgbeVirtual(0x1565);
	pub const IntelIxgbeVirtual_X550EM_A_VF: Self = Self::IntelIxgbeVirtual(0x15C5);
	pub const IntelIxgbeVirtual_X550EM_A_VF_HV: Self = Self::IntelIxgbeVirtual(0x15B4);
	pub const IntelIxgbeVirtual_X550EM_X_VF: Self = Self::IntelIxgbeVirtual(0x15A8);
	pub const IntelIxgbeVirtual_X550EM_X_VF_HV: Self = Self::IntelIxgbeVirtual(0x15A9);

	pub const IntelQatSymmetricCrypto: Self = Self::IntelQat(0x0443);

	pub const MellanoxMlx4_ConnectX3: Self = Self::MellanoxMlx4(0x1003);
	pub const MellanoxMlx4_ConnectX3_VF: Self = Self::MellanoxMlx4(0x1004);
	pub const MellanoxMlx4_ConnectX3_PRO: Self = Self::MellanoxMlx4(0x1007);

	pub const MellanoxMlx5_ConnectX4: Self = Self::MellanoxMlx5(0x1013);
	pub const MellanoxMlx5_ConnectX4_VF: Self = Self::MellanoxMlx5(0x1014);
	pub const MellanoxMlx5_ConnectX4_LX: Self = Self::MellanoxMlx5(0x1015);
	pub const MellanoxMlx5_ConnectX4_LX_VF: Self = Self::MellanoxMlx5(0x1016);

	pub const NetCopeSzedata2_Combo80G: Self = Self::NetCopeSzedata2(0xcb80);
	pub const NetCopeSzedata2_Combo100G: Self = Self::NetCopeSzedata2(0xc1c1);
	pub const NetCopeSzedata2_Combo100G2: Self = Self::NetCopeSzedata2(0xc2c1);

	pub const NetronomeNfp6000_PF: Self = Self::NetronomeNfp6000(0x6000);
	pub const NetronomeNfp6000_VF: Self = Self::NetronomeNfp6000(0x6003);

	pub const QLogicQedePhysical_NX2_57980E: Self = Self::QLogicQedePhysical(0x1634);
	pub const QLogicQedePhysical_NX2_57980S: Self = Self::QLogicQedePhysical(0x1629);
	pub const QLogicQedePhysical_57980S_40: Self = Self::QLogicQedePhysical(0x1634);
	pub const QLogicQedePhysical_57980S_25: Self = Self::QLogicQedePhysical(0x1656);
	pub const QLogicQedePhysical_57980S_100: Self = Self::QLogicQedePhysical(0x1644);

	pub const QLogicQedeVirtual_NX2_VF: Self = Self::QLogicQedeVirtual(0x1630);
	pub const QLogicQedeVirtual_57980S_IOV: Self = Self::QLogicQedeVirtual(0x1664);

	pub const VirtIoNetwork_: Self = Self::VirtIoNetwork(0x1000);

	pub const VmWareVmxNet3_: Self = Self::VmWareVmxNet3(0x07B0);

	const fn AmazonEna(pci_device_identifier: u16) -> Self
	{
		Self::Amazon(SupportedDriverIdentifier::AmazonEna, pci_device_identifier)
	}

	const fn BroadcomBnx2x(pci_device_identifier: u16) -> Self
	{
		Self::Broadcom(SupportedDriverIdentifier::BroadcomBnx2x, pci_device_identifier)
	}

	const fn BroadcomBnxt(pci_device_identifier: u16) -> Self
	{
		Self::Broadcom(SupportedDriverIdentifier::BroadcomBnxt, pci_device_identifier)
	}

	const fn ChelsioT5(pci_device_identifier: u16) -> Self
	{
		Self::Chelsio(SupportedDriverIdentifier::ChelsioT5, pci_device_identifier)
	}

	const fn CiscoVic(pci_device_identifier: u16) -> Self
	{
		Self::Cisco(SupportedDriverIdentifier::CiscoVic, pci_device_identifier)
	}

	const fn IntelE1000Em(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::IntelE1000Em, pci_device_identifier)
	}

	const fn IntelE1000Igb(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::IntelE1000Igb, pci_device_identifier)
	}

	const fn IntelFM10K(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::IntelFM10K, pci_device_identifier)
	}

	const fn Inteli40ePhysical(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::Inteli40ePhysical, pci_device_identifier)
	}

	const fn Inteli40eVirtual(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::Inteli40eVirtual, pci_device_identifier)
	}

	const fn IntelIxgbePhysical(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::IntelIxgbePhysical, pci_device_identifier)
	}

	const fn IntelIxgbePhysicalSubDevice(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::IntelIxgbePhysicalSubDevice, pci_device_identifier)
	}

	const fn IntelIxgbeVirtual(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::IntelIxgbeVirtual, pci_device_identifier)
	}

	const fn IntelQat(pci_device_identifier: u16) -> Self
	{
		Self::Intel(SupportedDriverIdentifier::IntelQat, pci_device_identifier)
	}

	const fn MellanoxMlx4(pci_device_identifier: u16) -> Self
	{
		Self::Mellanox(SupportedDriverIdentifier::MellanoxMlx4, pci_device_identifier)
	}

	const fn MellanoxMlx5(pci_device_identifier: u16) -> Self
	{
		Self::Mellanox(SupportedDriverIdentifier::MellanoxMlx5, pci_device_identifier)
	}

	const fn NetCopeSzedata2(pci_device_identifier: u16) -> Self
	{
		Self::NetCope(SupportedDriverIdentifier::NetCopeSzedata2, pci_device_identifier)
	}

	const fn NetronomeNfp6000(pci_device_identifier: u16) -> Self
	{
		Self::Netronome(SupportedDriverIdentifier::NetronomeNfp6000, pci_device_identifier)
	}

	const fn QLogicQedePhysical(pci_device_identifier: u16) -> Self
	{
		Self::QLogic(SupportedDriverIdentifier::QLogicQedePhysical, pci_device_identifier)
	}

	const fn QLogicQedeVirtual(pci_device_identifier: u16) -> Self
	{
		Self::QLogic(SupportedDriverIdentifier::QLogicQedeVirtual, pci_device_identifier)
	}

	const fn VirtIoNetwork(pci_device_identifier: u16) -> Self
	{
		Self::VirtIO(SupportedDriverIdentifier::VirtIoNetwork, pci_device_identifier)
	}

	const fn VmWareVmxNet3(pci_device_identifier: u16) -> Self
	{
		Self::VmWare(SupportedDriverIdentifier::VmWareVmxNet3, pci_device_identifier)
	}

	const fn Amazon(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::Amazon, driver, pci_device_identifier)
	}

	const fn Broadcom(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::Broadcom, driver, pci_device_identifier)
	}

	const fn Chelsio(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::Chelsio, driver, pci_device_identifier)
	}

	const fn Cisco(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::Cisco, driver, pci_device_identifier)
	}

	const fn Intel(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::Intel, driver, pci_device_identifier)
	}

	const fn Mellanox(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::Mellanox, driver, pci_device_identifier)
	}

	const fn NetCope(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::NetCope, driver, pci_device_identifier)
	}

	const fn Netronome(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::Netronome, driver, pci_device_identifier)
	}

	const fn QLogic(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::QLogic, driver, pci_device_identifier)
	}

	const fn VirtIO(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::VirtIO, driver, pci_device_identifier)
	}

	const fn VmWare(driver: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self::supportedDevice(PciVendorIdentifier::VmWare, driver, pci_device_identifier)
	}

	const fn supportedDevice(pci_vendor_identifier: PciVendorIdentifier, supported_driver_identifier: SupportedDriverIdentifier, pci_device_identifier: u16) -> Self
	{
		Self
		{
			pci_vendor_identifier,
			supported_driver_identifier,
			pci_device_identifier: DeviceId(pci_device_identifier),
		}
	}
}
