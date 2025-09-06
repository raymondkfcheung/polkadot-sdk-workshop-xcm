use super::RelayLocation;

use frame_support::traits::{Equals, EverythingBut};
use xcm_builder::{AllowTopLevelPaidExecutionFrom, TakeWeightCredit};

pub type Barrier =
	(TakeWeightCredit, AllowTopLevelPaidExecutionFrom<EverythingBut<Equals<RelayLocation>>>);
