#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{AccountId, DefaultEnvironment, Environment};
use scale::{Decode, Encode, HasCompact};

type Balance = <DefaultEnvironment as Environment>::Balance;

pub struct DappsStaking;

impl DappsStaking {
    /// Calls current_era() in the pallet-dapps-staking
    pub fn read_current_era() -> u32 {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3401u32)
            .input::<()>()
            .output::<u32>()
            .ignore_error_code()
            .call(&())
    }

    /// Calls general_era_info() in the pallet-dapps-staking
    pub fn read_era_info(era: u32) -> Result<EraInfo<Balance>, DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3402u32)
            .input::<u32>()
            .output::<Result<EraInfo<Balance>, DSError>>()
            .handle_error_code::<DSError>()
            .call(&era)?
    }

    /// Calls bond_and_stake() in the pallet-dapps-staking
    pub fn bond_and_stake(account_id: AccountId, value: Balance) -> Result<(), DSError> {
        let input = BondStakeInput { account_id, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(3403u32)
            .input::<BondStakeInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BondStakeInput {
    account_id: AccountId,
    value: Balance,
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

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DSError {
    Failed,
}

impl ink_env::chain_extension::FromStatusCode for DSError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for DSError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
