use sov_modules_api::Spec;
use crate::AccountAddress;

/// Events emitted by the TAccount module
#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
    Clone,
)]
#[serde(bound = "S::Address: serde::Serialize + serde::de::DeserializeOwned")]
pub enum Event<S: Spec> {
    /// Emitted when a new account is created
    AccountCreated {
        /// The address of the newly created account
        address: AccountAddress<S>,
    },
    // /// Emitted when a deposit is made to an account
    // Deposit {
    //     /// The address of the account receiving the deposit
    //     address: S::Address,
    //     /// The amount deposited
    //     amount: u128,
    // },
    // /// Emitted when a withdrawal is made from an account
    // Withdraw {
    //     /// The address of the account making the withdrawal
    //     address: S::Address,
    //     /// The amount withdrawn
    //     amount: u128,
    // },
}
