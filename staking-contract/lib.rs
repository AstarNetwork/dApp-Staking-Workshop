#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod staking {
    use dapps_staking::{DSError, DappsStaking, EraInfo};

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum StakingError {
        DsError(DSError),
    }

    #[ink(storage)]
    pub struct Staking {}

    impl Staking {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_current_era(&self) -> u32 {
            DappsStaking::read_current_era()
        }

        #[ink(message)]
        pub fn read_era_info(&self, era: u32) -> Result<EraInfo<Balance>, StakingError> {
            DappsStaking::read_era_info(era).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn bond_and_stake(&mut self, value: Balance) -> Result<(), StakingError> {
            let contract = self.env().account_id();
            DappsStaking::bond_and_stake(contract, value)
                .map_err(|e| return StakingError::DsError(e))?;
            Ok(())
        }
    }
}
