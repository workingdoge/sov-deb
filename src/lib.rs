mod call;
mod genesis;
#[cfg(feature = "native")]
mod query;

use borsh::{BorshDeserialize, BorshSerialize};
pub use call::CallMessage;

pub mod event;
pub use crate::event::Event;
#[cfg(feature = "native")]
pub use query::*;
use serde::{Deserialize, Serialize};
use sov_modules_api::{
    macros::address_type, Context, Error, GenesisState, ModuleId, ModuleInfo, Spec, StateMap,
    TxState,
};

use anyhow::Result;
use std::fmt;

// use ethnum::U256;

// TODO: RPC is having issues with generics
// pub trait UInt:
//
//     'static
//     + Default
//     + Sized
//     + Clone
//     + Copy
//     + PartialEq
//     + Eq
//     + PartialOrd
//     + Ord
//     + Add<Output = Self>
//     + Sub<Output = Self>
//     + BorshSerialize
//     + BorshDeserialize
//     + serde::Serialize
//     + for<'de> serde::Deserialize<'de>
//     + Sync
//     + Send
// {
//     fn zero() -> Self {
//         Self::default()
//     }
// }
//
// // TODO: Borsch for U256
// // impl UInt for U256 {}
// impl UInt for u64 {}
// impl UInt for u128 {}
//
#[address_type]
#[derive(Copy)]
pub struct AccountAddress;

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TAccount {
    debit: u128,
    credit: u128,
}

impl TAccount {
    pub fn new(debit: u128, credit: u128) -> Self {
        Self { debit, credit }
    }

    pub fn debit(&self) -> u128 {
        self.debit
    }

    pub fn credit(&self) -> u128 {
        self.credit
    }

    pub fn reduced(&self) -> Self {
        let min = std::cmp::min(self.debit, self.credit);
        Self {
            debit: self.debit - min,
            credit: self.credit - min,
        }
    }

    pub fn balance(&self) -> (BalanceType, u128) {
        if self.debit > self.credit {
            (BalanceType::Debit, self.debit - self.credit)
        } else {
            (BalanceType::Credit, self.credit - self.debit)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceType {
    Debit,
    Credit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAccountModuleConfig {}

#[derive(ModuleInfo)]
pub struct TAccountModule<S: Spec> {
    #[id]
    pub id: ModuleId,
    #[state]
    pub accounts: StateMap<AccountAddress<S>, TAccount>,
}

impl<S: Spec> sov_modules_api::Module for TAccountModule<S> {
    type Spec = S;
    type Config = TAccountModuleConfig;
    type CallMessage = call::CallMessage;
    type Event = Event<S>;

    fn genesis(
        &self,
        config: &Self::Config,
        state: &mut impl GenesisState<S>,
    ) -> Result<(), Error> {
        // The initialization logic
        Ok(self.init_module(config, state)?)
    }

    fn call(
        &self,
        msg: Self::CallMessage,
        context: &Context<Self::Spec>,
        state: &mut impl TxState<S>,
    ) -> Result<sov_modules_api::CallResponse, Error> {
        match msg {
            CallMessage::CreateAccount => self.create_account(context, state),
            // CallMessage::Deposit { amount } => self.deposit(amount, context, state),
            // CallMessage::Withdraw { amount } => self.withdraw(amount, context, state),
        }
    }
}
