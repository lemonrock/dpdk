// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A PCI device class.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum PciDeviceClass
{
	UnclassifiedDevice(UnclassifiedDevice),
	MassStorageController(MassStorageController),
	NetworkController(NetworkController),
	DisplayController(DisplayController),
	MultimediaController(MultimediaController),
	MemoryController(MemoryController),
	Bridge(Bridge),
	CommunicationController(CommunicationController),
	GenericSystemPeripheral(GenericSystemPeripheral),
	InputDeviceController(InputDeviceController),
	DockingStation(DockingStation),
	Processor(Processor),
	SerialBusController(SerialBusController),
	WirelessController(WirelessController),
	IntelligentController(IntelligentController),
	SatelliteCommunicationsController(SatelliteCommunicationsController),
	EncryptionController(EncryptionController),
	SignalProcessingController(SignalProcessingController),
	ProcessingAccelerators,
	NonEssentialInstrumentation,
	Coprocessor,
	Unassigned(Unassigned),
}

impl PciDeviceClass
{
	/// Unassigned subclass.
	pub const UnassignedSubClass: u16 = 0xff;
	
	/// A PCI device (class, subclass) as an u32.
	#[inline(always)]
	pub fn to_u32(&self) -> u32
	{
		let (class, subclass) = self.major_minor();
		
		(class as u32) << 16 + (subclass as u32)
	}
	
	/// A PCI device (class, subclass) as an (u16, u16) tuple.
	#[inline(always)]
	#[allow(missing_docs)]
	pub fn major_minor(&self) -> (u16, u16)
	{
		use self::PciDeviceClass::*;
		
		match *self
		{
			UnclassifiedDevice(subclass) => (0x00, subclass as u16),
			MassStorageController(subclass) => (0x01, subclass as u16),
			NetworkController(subclass) => (0x02, subclass as u16),
			DisplayController(subclass) => (0x03, subclass as u16),
			MultimediaController(subclass) => (0x04, subclass as u16),
			MemoryController(subclass) => (0x05, subclass as u16),
			Bridge(subclass) => (0x06, subclass as u16),
			CommunicationController(subclass) => (0x07, subclass as u16),
			GenericSystemPeripheral(subclass) => (0x08, subclass as u16),
			InputDeviceController(subclass) => (0x09, subclass as u16),
			DockingStation(subclass) => (0x0a, subclass as u16),
			Processor(subclass) => (0x0b, subclass as u16),
			SerialBusController(subclass) => (0x0c, subclass as u16),
			WirelessController(subclass) => (0x0d, subclass as u16),
			IntelligentController(subclass) => (0x0e, subclass as u16),
			SatelliteCommunicationsController(subclass) => (0x0f, subclass as u16),
			EncryptionController(subclass) => (0x10, subclass as u16),
			SignalProcessingController(subclass) => (0x11, subclass as u16),
			ProcessingAccelerators => (0x12, Self::UnassignedSubClass),
			NonEssentialInstrumentation => (0x13, Self::UnassignedSubClass),
			Coprocessor => (0x40, Self::UnassignedSubClass),
			Unassigned(subclass) => (0xff, subclass as u16),
		}
	}
}
