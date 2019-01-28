// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// User identifiers (UIDs).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessUserIdentifiers
{
	/// Real user identifier (UID).
	pub real: uid_t,

	/// Effective user identifier (UID).
	pub effective: uid_t,

	/// Saved set user identifier (UID).
	pub saved_set: uid_t,

	/// File system user identifier (UID).
	pub file_system: uid_t,
}
