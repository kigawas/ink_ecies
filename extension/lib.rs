#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::{
    env::{chain_extension::FromStatusCode, DefaultEnvironment, Environment},
    prelude::vec::Vec,
};
use scale::{Decode, Encode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum EciesErr {
    FailEncrypt,
}

#[ink::chain_extension]
pub trait Ecies {
    type ErrorCode = EciesErr;

    #[ink(extension = 0x00030001)]
    fn encrypt(pk: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>, EciesErr>;
}

impl FromStatusCode for EciesErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailEncrypt),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for EciesErr {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum EciesEnvironment {}

impl Environment for EciesEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = Ecies;
}
