// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// PCI device identification.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PciVendorAndDevice
{
	/// Vendor.
	pub vendor: PciVendorIdentifier,
	
	/// Device.
	pub device: PciDeviceIdentifier,
}

#[allow(missing_docs)]
impl PciVendorAndDevice
{
	pub const Amazon_Ena_LLQ_VF: Self = Self::new(PciVendorIdentifier::Amazon, PciDeviceIdentifier(0xEC21));
	
	pub const Amazon_Ena_VF: Self = Self::new(PciVendorIdentifier::Amazon, PciDeviceIdentifier(0xEC20));
	
	pub const Broadcom_Bnx2x_ChipNumber57711: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x164f));
	
	pub const Broadcom_Bnx2x_ChipNumber57711E: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x1650));
	
	pub const Broadcom_Bnx2x_ChipNumber57712: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x1662));
	
	pub const Broadcom_Bnx2x_ChipNumber57712MF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x1663));
	
	pub const Broadcom_Bnx2x_ChipNumber57712VF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x166f));
	
	pub const Broadcom_Bnx2x_ChipNumber57800: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x168a));
	
	pub const Broadcom_Bnx2x_ChipNumber57800MF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16a5));
	
	pub const Broadcom_Bnx2x_ChipNumber57800VF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16a9));
	
	pub const Broadcom_Bnx2x_ChipNumber57810: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x168e));
	
	pub const Broadcom_Bnx2x_ChipNumber57810MF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16ae));
	
	pub const Broadcom_Bnx2x_ChipNumber57810VF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16af));
	
	pub const Broadcom_Bnx2x_ChipNumber57811: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x163d));
	
	pub const Broadcom_Bnx2x_ChipNumber57811MF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x163e));
	
	pub const Broadcom_Bnx2x_ChipNumber57811VF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x163f));
	
	pub const Broadcom_Bnx2x_ChipNumber57840220: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16a2));
	
	pub const Broadcom_Bnx2x_ChipNumber57840410: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16a1));
	
	pub const Broadcom_Bnx2x_ChipNumber57840MF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16a4));
	
	pub const Broadcom_Bnx2x_ChipNumber57840OBS: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x168d));
	
	pub const Broadcom_Bnx2x_ChipNumber57840OBSMF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16ab));
	
	pub const Broadcom_Bnx2x_ChipNumber57840VF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16ad));
	
	pub const Broadcom_Bnxt_ChipNumber57301: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16c8));
	
	pub const Broadcom_Bnxt_ChipNumber57302: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16c9));
	
	pub const Broadcom_Bnxt_ChipNumber57304PF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16ca));
	
	pub const Broadcom_Bnxt_ChipNumber57304VF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16cb));
	
	pub const Broadcom_Bnxt_ChipNumber57314: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16df));
	
	pub const Broadcom_Bnxt_ChipNumber57402: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16d0));
	
	pub const Broadcom_Bnxt_ChipNumber57404: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16d1));
	
	pub const Broadcom_Bnxt_ChipNumber57406MF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16d4));
	
	pub const Broadcom_Bnxt_ChipNumber57406PF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16d2));
	
	pub const Broadcom_Bnxt_ChipNumber57406VF: Self = Self::new(PciVendorIdentifier::Broadcom, PciDeviceIdentifier(0x16d3));
	
	pub const Chelsio_T5_B504_BT: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x500c));
	
	pub const Chelsio_T5_B520_SR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x500b));
	
	pub const Chelsio_T5_Custom_2x_T580_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5086));
	
	pub const Chelsio_T5_Custom_3x_T580_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5085));
	
	pub const Chelsio_T5_Custom_T504_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5082));
	
	pub const Chelsio_T5_Custom_T520_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5089));
	
	pub const Chelsio_T5_Custom_T520_CR_Alternative: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5092));
	
	pub const Chelsio_T5_Custom_T522_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5091));
	
	pub const Chelsio_T5_Custom_T540_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5080));
	
	pub const Chelsio_T5_Custom_T540_CR_Alternative: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5090));
	
	pub const Chelsio_T5_Custom_T540_LL_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5081));
	
	pub const Chelsio_T5_Custom_T540_LP_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5083));
	
	pub const Chelsio_T5_Custom_T570_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5088));
	
	pub const Chelsio_T5_Custom_T580_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5084));
	
	pub const Chelsio_T5_Custom_T580_CR_Alternative: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5087));
	
	pub const Chelsio_T5_T502_BT: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5015));
	
	pub const Chelsio_T5_T504_BT: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x500a));
	
	pub const Chelsio_T5_T520_BCH: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5004));
	
	pub const Chelsio_T5_T520_BT: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5009));
	
	pub const Chelsio_T5_T520_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5001));
	
	pub const Chelsio_T5_T520_CX: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5008));
	
	pub const Chelsio_T5_T520_LL_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5011));
	
	pub const Chelsio_T5_T520_SO: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5007));
	
	pub const Chelsio_T5_T522_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5002));
	
	pub const Chelsio_T5_T540_BCH: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5005));
	
	pub const Chelsio_T5_T540_CH: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5006));
	
	pub const Chelsio_T5_T540_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5003));
	
	pub const Chelsio_T5_T540_LP_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x500e));
	
	pub const Chelsio_T5_T560_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5012));
	
	pub const Chelsio_T5_T580_CHR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5013));
	
	pub const Chelsio_T5_T580_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x500d));
	
	pub const Chelsio_T5_T580_DBG: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5000));
	
	pub const Chelsio_T5_T580_LP_CR: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5010));
	
	pub const Chelsio_T5_T580_SO: Self = Self::new(PciVendorIdentifier::Chelsio, PciDeviceIdentifier(0x5014));
	
	pub const Cisco_Vic_Enet: Self = Self::new(PciVendorIdentifier::Cisco, PciDeviceIdentifier(0x0043));
	
	pub const Cisco_Vic_EnetVF: Self = Self::new(PciVendorIdentifier::Cisco, PciDeviceIdentifier(0x0071));
	
	pub const Intel_E1000_Em82540EM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x100E));
	
	pub const Intel_E1000_Em82545EM_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x100F));
	
	pub const Intel_E1000_Em82545EM_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1011));
	
	pub const Intel_E1000_Em82546EB_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1010));
	
	pub const Intel_E1000_Em82546EB_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1012));
	
	pub const Intel_E1000_Em82546EB_Quad_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x101D));
	
	pub const Intel_E1000_Em82571EB_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x105E));
	
	pub const Intel_E1000_Em82571EB_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x105F));
	
	pub const Intel_E1000_Em82571EB_Quad_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10A4));
	
	pub const Intel_E1000_Em82571EB_Quad_Copper_LP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10BC));
	
	pub const Intel_E1000_Em82571EB_Quad_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10A5));
	
	pub const Intel_E1000_Em82571EB_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1060));
	
	pub const Intel_E1000_Em82571EB_Serdes_Dual: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10D9));
	
	pub const Intel_E1000_Em82571EB_Serdes_Quad: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10DA));
	
	pub const Intel_E1000_Em82571PT_Quad_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10D5));
	
	pub const Intel_E1000_Em82572EI: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10B9));
	
	pub const Intel_E1000_Em82572EI_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x107D));
	
	pub const Intel_E1000_Em82572EI_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x107E));
	
	pub const Intel_E1000_Em82572EI_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x107F));
	
	pub const Intel_E1000_Em82573L: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x109A));
	
	pub const Intel_E1000_Em82574L: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10D3));
	
	pub const Intel_E1000_Em82574LA: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10F6));
	
	pub const Intel_E1000_Em82583V: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x150C));
	
	pub const Intel_E1000_Em_PCH_I218_LM2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A0));
	
	pub const Intel_E1000_Em_PCH_I218_LM3: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A2));
	
	pub const Intel_E1000_Em_PCH_I218_V2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A1));
	
	pub const Intel_E1000_Em_PCH_I218_V3: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A3));
	
	pub const Intel_E1000_Em_PCH_LPTLP_I218_LM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x155A));
	
	pub const Intel_E1000_Em_PCH_LPTLP_I218_V: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1559));
	
	pub const Intel_E1000_Em_PCH_LPT_I217_LM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x153A));
	
	pub const Intel_E1000_Em_PCH_LPT_I217_V: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x153B));
	
	pub const Intel_E1000_Igb_82575EB_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10A7));
	
	pub const Intel_E1000_Igb_82575EB_Fibre_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10A9));
	
	pub const Intel_E1000_Igb_82575GB_Quad_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10D6));
	
	pub const Intel_E1000_Igb_82576: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10C9));
	
	pub const Intel_E1000_Igb_82576_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10E6));
	
	pub const Intel_E1000_Igb_82576_NS: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x150A));
	
	pub const Intel_E1000_Igb_82576_NS_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1518));
	
	pub const Intel_E1000_Igb_82576_Quad_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10E8));
	
	pub const Intel_E1000_Igb_82576_Quad_Copper_ET2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1526));
	
	pub const Intel_E1000_Igb_82576_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10E7));
	
	pub const Intel_E1000_Igb_82576_Serdes_Quad: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x150D));
	
	pub const Intel_E1000_Igb_82580_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x150E));
	
	pub const Intel_E1000_Igb_82580_Copper_Dual: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1516));
	
	pub const Intel_E1000_Igb_82580_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x150F));
	
	pub const Intel_E1000_Igb_82580_Quad_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1527));
	
	pub const Intel_E1000_Igb_82580_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1510));
	
	pub const Intel_E1000_Igb_82580_SGMII: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1511));
	
	pub const Intel_E1000_Igb_DH89XXCC_Backplane: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x043C));
	
	pub const Intel_E1000_Igb_DH89XXCC_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x043A));
	
	pub const Intel_E1000_Igb_DH89XXCC_SFP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x0440));
	
	pub const Intel_E1000_Igb_DH89XXCC_SGMII: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x0438));
	
	pub const Intel_E1000_Igb_I210_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1533));
	
	pub const Intel_E1000_Igb_I210_Copper_IT: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1535));
	
	pub const Intel_E1000_Igb_I210_Copper_OEM1: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1534));
	
	pub const Intel_E1000_Igb_I210_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1536));
	
	pub const Intel_E1000_Igb_I210_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1537));
	
	pub const Intel_E1000_Igb_I210_SGMII: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1538));
	
	pub const Intel_E1000_Igb_I211_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1539));
	
	pub const Intel_E1000_Igb_I350_Copper: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1521));
	
	pub const Intel_E1000_Igb_I350_DA4: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1546));
	
	pub const Intel_E1000_Igb_I350_Fibre: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1522));
	
	pub const Intel_E1000_Igb_I350_Serdes: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1523));
	
	pub const Intel_E1000_Igb_I350_SGMII: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1524));
	
	pub const Intel_E1000_Igb_I354_Backplane_1GBPS: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1F40));
	
	pub const Intel_E1000_Igb_I354_Backplane_2_5GBPS: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1F45));
	
	pub const Intel_E1000_Igb_I354_SGMII: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1F41));
	
	pub const Intel_FM10K_PF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A4));
	
	pub const Intel_FM10K_SDI_FM10420_DA2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15D5));
	
	pub const Intel_FM10K_SDI_FM10420_QDA2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15D0));
	
	pub const Intel_FM10K_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A5));
	
	pub const Intel_i40e_Physical_10G_BASE_T4: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1589));
	
	pub const Intel_i40e_Physical_10G_BASE_T: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1586));
	
	pub const Intel_i40e_Physical_10G_BASE_T_X722: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37D2));
	
	pub const Intel_i40e_Physical_1G_BASE_T_X722: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37D1));
	
	pub const Intel_i40e_Physical_20G_KR2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1587));
	
	pub const Intel_i40e_Physical_20G_KR2_A: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1588));
	
	pub const Intel_i40e_Physical_25G_B: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x158A));
	
	pub const Intel_i40e_Physical_25G_SFP28: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x158B));
	
	pub const Intel_i40e_Physical_KX_B: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1580));
	
	pub const Intel_i40e_Physical_KX_C: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1581));
	
	pub const Intel_i40e_Physical_KX_X722: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37CE));
	
	pub const Intel_i40e_Physical_QEMU: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1574));
	
	pub const Intel_i40e_Physical_QSFP_A: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1583));
	
	pub const Intel_i40e_Physical_QSFP_B: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1584));
	
	pub const Intel_i40e_Physical_QSFP_C: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1585));
	
	pub const Intel_i40e_Physical_QSFP_I_X722: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37D4));
	
	pub const Intel_i40e_Physical_QSFP_X722: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37CF));
	
	pub const Intel_i40e_Physical_SFP_I_X722: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37D3));
	
	pub const Intel_i40e_Physical_SFP_X722: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37D0));
	
	pub const Intel_i40e_Physical_SFP_XL710: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1572));
	
	pub const Intel_i40e_Physical_X722_A0: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x374C));
	
	pub const Intel_i40e_Virtual_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x154C));
	
	pub const Intel_i40e_Virtual_VF_HV: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1571));
	
	pub const Intel_i40e_Virtual_X722_A0_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x374D));
	
	pub const Intel_i40e_Virtual_X722_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37CD));
	
	pub const Intel_i40e_Virtual_X722_VF_HV: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x37D9));
	
	pub const Intel_Ixgbe_Physical_82598: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10B6));
	
	pub const Intel_Ixgbe_Physical_82598AF_Dual_PORT: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10C6));
	
	pub const Intel_Ixgbe_Physical_82598AF_Single_PORT: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10C7));
	
	pub const Intel_Ixgbe_Physical_82598AT2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x150B));
	
	pub const Intel_Ixgbe_Physical_82598AT: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10C8));
	
	pub const Intel_Ixgbe_Physical_82598EB_CX4: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10DD));
	
	pub const Intel_Ixgbe_Physical_82598EB_SFP_LOM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10DB));
	
	pub const Intel_Ixgbe_Physical_82598EB_XF_LR: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10F4));
	
	pub const Intel_Ixgbe_Physical_82598_BX: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1508));
	
	pub const Intel_Ixgbe_Physical_82598_CX4_Dual_PORT: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10EC));
	
	pub const Intel_Ixgbe_Physical_82598_DA_Dual_PORT: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10F1));
	
	pub const Intel_Ixgbe_Physical_82598_SR_Dual_PORT_EM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10E1));
	
	pub const Intel_Ixgbe_Physical_82599EN_SFP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1557));
	
	pub const Intel_Ixgbe_Physical_82599_Backplane_FCOE: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x152A));
	
	pub const Intel_Ixgbe_Physical_82599_Bypass: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x155D));
	
	pub const Intel_Ixgbe_Physical_82599_COMBO_Backplane: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10F8));
	
	pub const Intel_Ixgbe_Physical_82599_CX4: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10F9));
	
	pub const Intel_Ixgbe_Physical_82599_KR: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1517));
	
	pub const Intel_Ixgbe_Physical_82599_KX4: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10F7));
	
	pub const Intel_Ixgbe_Physical_82599_KX4_Mezzanine: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1514));
	
	pub const Intel_Ixgbe_Physical_82599_LS: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x154F));
	
	pub const Intel_Ixgbe_Physical_82599_QSFP_SF_QP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1558));
	
	pub const Intel_Ixgbe_Physical_82599_SFP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10FB));
	
	pub const Intel_Ixgbe_Physical_82599_SFP_EM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1507));
	
	pub const Intel_Ixgbe_Physical_82599_SFP_FCOE: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1529));
	
	pub const Intel_Ixgbe_Physical_82599_SFP_SF2: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x154D));
	
	pub const Intel_Ixgbe_Physical_82599_SFP_SF_QP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x154A));
	
	pub const Intel_Ixgbe_Physical_82599_T3_LOM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x151C));
	
	pub const Intel_Ixgbe_Physical_82599_XAUI_LOM: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10FC));
	
	pub const Intel_Ixgbe_Physical_SubDevice82599_560FLR: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x17D0));
	
	pub const Intel_Ixgbe_Physical_SubDevice82599_ECNA_DP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x0470));
	
	pub const Intel_Ixgbe_Physical_SubDevice82599_KX4_KR_Mezzanine: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x000C));
	
	pub const Intel_Ixgbe_Physical_SubDevice82599_RNDC: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1F72));
	
	pub const Intel_Ixgbe_Physical_SubDevice82599_SFP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x11A9));
	
	pub const Intel_Ixgbe_Physical_X540T1: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1560));
	
	pub const Intel_Ixgbe_Physical_X540T: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1528));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_10G_T: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15C8));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_1G_T: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15E4));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_1G_T_L: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15E5));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_KR: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15C2));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_KR_L: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15C3));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_QSFP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15CA));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_QSFP_N: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15CC));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_SFP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15CE));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_SFP_N: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15C4));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_SGMII: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15C6));
	
	pub const Intel_Ixgbe_Physical_X550EM_A_SGMII_L: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15C7));
	
	pub const Intel_Ixgbe_Physical_X550EM_X_10G_T: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15AD));
	
	pub const Intel_Ixgbe_Physical_X550EM_X_1G_T: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15AE));
	
	pub const Intel_Ixgbe_Physical_X550EM_X_KR: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15AB));
	
	pub const Intel_Ixgbe_Physical_X550EM_X_KX4: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15AA));
	
	pub const Intel_Ixgbe_Physical_X550EM_X_SFP: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15AC));
	
	pub const Intel_Ixgbe_Physical_X550T1: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15D1));
	
	pub const Intel_Ixgbe_Physical_X550T: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1563));
	
	pub const Intel_Ixgbe_Virtual_82599_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x10ED));
	
	pub const Intel_Ixgbe_Virtual_82599_VF_HV: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x152E));
	
	pub const Intel_Ixgbe_Virtual_X540_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1515));
	
	pub const Intel_Ixgbe_Virtual_X540_VF_HV: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1530));
	
	pub const Intel_Ixgbe_Virtual_X550EM_A_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15C5));
	
	pub const Intel_Ixgbe_Virtual_X550EM_A_VF_HV: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15B4));
	
	pub const Intel_Ixgbe_Virtual_X550EM_X_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A8));
	
	pub const Intel_Ixgbe_Virtual_X550EM_X_VF_HV: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x15A9));
	
	pub const Intel_Ixgbe_Virtual_X550_VF: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1565));
	
	pub const Intel_Ixgbe_Virtual_X550_VF_HV: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x1564));
	
	pub const Intel_QAT: Self = Self::new(PciVendorIdentifier::Intel, PciDeviceIdentifier(0x0443));
	
	pub const Mellanox_Mlx4_ConnectX3: Self = Self::new(PciVendorIdentifier::Mellanox, PciDeviceIdentifier(0x1003));
	
	pub const Mellanox_Mlx4_ConnectX3_PRO: Self = Self::new(PciVendorIdentifier::Mellanox, PciDeviceIdentifier(0x1007));
	
	pub const Mellanox_Mlx4_ConnectX3_VF: Self = Self::new(PciVendorIdentifier::Mellanox, PciDeviceIdentifier(0x1004));
	
	pub const Mellanox_Mlx5_ConnectX4: Self = Self::new(PciVendorIdentifier::Mellanox, PciDeviceIdentifier(0x1013));
	
	pub const Mellanox_Mlx5_ConnectX4_LX: Self = Self::new(PciVendorIdentifier::Mellanox, PciDeviceIdentifier(0x1015));
	
	pub const Mellanox_Mlx5_ConnectX4_LX_VF: Self = Self::new(PciVendorIdentifier::Mellanox, PciDeviceIdentifier(0x1016));
	
	pub const Mellanox_Mlx5_ConnectX4_VF: Self = Self::new(PciVendorIdentifier::Mellanox, PciDeviceIdentifier(0x1014));
	
	pub const NetCope_Szedata2_Combo100G2: Self = Self::new(PciVendorIdentifier::NetCope, PciDeviceIdentifier(0xc2c1));
	
	pub const NetCope_Szedata2_Combo100G: Self = Self::new(PciVendorIdentifier::NetCope, PciDeviceIdentifier(0xc1c1));
	
	pub const NetCope_Szedata2_Combo80G: Self = Self::new(PciVendorIdentifier::NetCope, PciDeviceIdentifier(0xcb80));
	
	pub const Netronome_Nfp6000_PF: Self = Self::new(PciVendorIdentifier::Netronome, PciDeviceIdentifier(0x6000));
	
	pub const Netronome_Nfp6000_VF: Self = Self::new(PciVendorIdentifier::Netronome, PciDeviceIdentifier(0x6003));
	
	pub const QLogic_QedePhysical_57980S_100: Self = Self::new(PciVendorIdentifier::QLogic, PciDeviceIdentifier(0x1644));
	
	pub const QLogic_QedePhysical_57980S_25: Self = Self::new(PciVendorIdentifier::QLogic, PciDeviceIdentifier(0x1656));
	
	pub const QLogic_QedePhysical_57980S_40: Self = Self::new(PciVendorIdentifier::QLogic, PciDeviceIdentifier(0x1634));
	
	pub const QLogic_QedePhysical_NX2_57980E: Self = Self::new(PciVendorIdentifier::QLogic, PciDeviceIdentifier(0x1634));
	
	pub const QLogic_QedePhysical_NX2_57980S: Self = Self::new(PciVendorIdentifier::QLogic, PciDeviceIdentifier(0x1629));
	
	pub const QLogic_QedeVirtual_57980S_IOV: Self = Self::new(PciVendorIdentifier::QLogic, PciDeviceIdentifier(0x1664));
	
	pub const QLogic_QedeVirtual_NX2_VF: Self = Self::new(PciVendorIdentifier::QLogic, PciDeviceIdentifier(0x1630));
	
	pub const VirtIO_Network: Self = Self::new(PciVendorIdentifier::VirtIO, PciDeviceIdentifier(0x1000));
	
	pub const VmWare_VmxNet3: Self = Self::new(PciVendorIdentifier::VmWare, PciDeviceIdentifier(0x07B0));
}

impl PciVendorAndDevice
{
	/// Constructs a new instance.
	#[inline(always)]
	pub const fn new(vendor: PciVendorIdentifier, device: PciDeviceIdentifier) -> Self
	{
		Self
		{
			vendor,
			device,
		}
	}
	
	/// Does this support device match a `rte_pci_id`?
	///
	/// This test does not return true is Any (0xFFFF) is set for a vendor or device id.
	#[inline(always)]
	pub fn is_exactly_rte_pci_id(&self, other: &rte_pci_id) -> bool
	{
		self.vendor.is(other.vendor_id) && self.device.is(other.device_id)
	}
}
