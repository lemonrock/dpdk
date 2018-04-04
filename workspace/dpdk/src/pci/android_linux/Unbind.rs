// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unbind
{
	pub networkPortIdentifier: NetworkPortIdentifier,
	pub pciDevice: PciDevice,
	dpdkPciDriverToUnbindFrom: PciDriver,
	bindBackToOriginal: Option<PciDriver>,
}

impl Unbind
{
	pub fn unbindOnTermination(&self, sysPath: &Path)
	{
		if let Some(ref bindBackToOriginal) = self.bindBackToOriginal
		{
			if bindBackToOriginal != &self.dpdkPciDriverToUnbindFrom
			{
				if let Err(error) = self.dpdkPciDriverToUnbindFrom.unbindPciDevice(sysPath, &self.pciDevice.0)
				{
					warn!("Could not unbind {:?} from DPDK because {:?}", self.pciDevice, error);
				}
				else
				{
					if let Err(error) = bindBackToOriginal.bindPciDevice(sysPath, &self.pciDevice.0)
					{
						warn!("Could not rebind to original {:?} from DPDK because {:?}", self.pciDevice, error);
					}
				}
			}
		}
	}
}
