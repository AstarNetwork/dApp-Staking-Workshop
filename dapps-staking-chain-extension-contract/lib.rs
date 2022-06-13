#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;
use scale::{Decode, Encode, HasCompact};

#[ink::chain_extension]
pub trait DappsStakingExt {
    type ErrorCode = DSErrorCode;

    #[ink(extension = 3401, returns_result = false, handle_status = false)]
    fn read_current_era() -> u32;

    #[ink(extension = 3402, handle_status = false)]
    fn read_era_info(
        era: u32,
    ) -> Result<EraInfo<<ink_env::DefaultEnvironment as Environment>::Balance>, DSError>;

    #[ink(extension = 3403)]
    fn bond_and_stake(
        account_id: <ink_env::DefaultEnvironment as Environment>::AccountId,
        value: <ink_env::DefaultEnvironment as Environment>::Balance,
    ) -> Result<(), DSError>;
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DSErrorCode {
    Failed,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DSError {
    ErrorCode(DSErrorCode),
}

impl From<DSErrorCode> for DSError {
    fn from(error_code: DSErrorCode) -> Self {
        Self::ErrorCode(error_code)
    }
}

impl From<scale::Error> for DSError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for DSErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

/// A record of rewards allocated for stakers and dapps
#[derive(PartialEq, Debug, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RewardInfo<Balance: HasCompact> {
    /// Total amount of rewards for stakers in an era
    #[codec(compact)]
    pub stakers: Balance,
    /// Total amount of rewards for dapps in an era
    #[codec(compact)]
    pub dapps: Balance,
}

/// A record for total rewards and total amount staked for an era
#[derive(PartialEq, Debug, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct EraInfo<Balance: HasCompact> {
    /// Total amount of earned rewards for an era
    pub rewards: RewardInfo<Balance>,
    /// Total staked amount in an era
    #[codec(compact)]
    pub staked: Balance,
    /// Total locked amount in an era
    #[codec(compact)]
    pub locked: Balance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = DappsStakingExt;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod dapp_staking_extension {
    use super::{DSError, EraInfo};

    #[ink(storage)]
    pub struct DappsStakingExtension {}

    #[ink(event)]
    pub struct CurrentEraUpdated {
        #[ink(topic)]
        new: u32,
    }

    impl DappsStakingExtension {
        #[ink(constructor)]
        pub fn new() -> Self {
            DappsStakingExtension {}
        }

        /// Calls current_era() in the pallet-dapps-staking
        #[ink(message)]
        pub fn read_current_era(&self) -> Result<u32, DSError> {
            let era = self.env().extension().read_current_era();
            self.env().emit_event(CurrentEraUpdated { new: era });
            Ok(era)
        }

        /// Calls general_era_info() in the pallet-dapps-staking
        #[ink(message)]
        pub fn read_era_info(&self, era: u32) -> Result<EraInfo<Balance>, DSError> {
            self.env().extension().read_era_info(era)
        }

        /// Calls bond_and_stake() in the pallet-dapps-staking
        #[ink(message)]
        pub fn bond_and_stake(
            &mut self,
            account_id: AccountId,
            value: Balance,
        ) -> Result<(), DSError> {
            self.env().extension().bond_and_stake(account_id, value)
        }
    }
}
