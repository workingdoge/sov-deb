use crate::event::Event;
use crate::{AccountAddress, BalanceType, TAccount, TAccountModule};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sov_modules_api::{CallResponse, Context, Error, EventEmitter, TxState};
use std::fmt::Debug;

/// This enumeration represents the available call messages for interacting with
/// the `TAccountModule`.
#[cfg_attr(feature = "native", derive(schemars::JsonSchema))]
#[derive(
    borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone, Serialize, Deserialize,
)]
pub enum CallMessage {
    CreateAccount,
    // Deposit { amount: u128 },
    // Withdraw { amount: u128 },
}

impl<S: sov_modules_api::Spec> TAccountModule<S> {
    /// Creates a new account for the sender
    pub(crate) fn create_account(
        &self,
        context: &Context<S>,
        state: &mut impl TxState<S>,
    ) -> Result<CallResponse, Error> {
        let acc = TAccount::new(0, 0);
        self.accounts
            .set(&AccountAddress::new(context.sender()), &acc, state)
            .map_err(|_e| anyhow!("StateAccessorError: Unable to set account as key"))?;

        self.emit_event(
            state,
            Event::AccountCreated {
                address: context.sender().clone(),
            },
        );
        Ok(CallResponse::default())
    }

    /// Gets the balance of the sender's account
    pub fn get_balance(
        &self,
        owner: &AccountAddress<S>,
        state: &mut impl TxState<S>,
    ) -> Result<(BalanceType, u128)> {
        let balance = self
            .accounts
            .get(owner, state)
            .map_err(|_e| anyhow!("Unable to access accounts"))
            .and_then(|tacc| {
                tacc.map(|tacc| tacc.balance())
                    .ok_or_else(|| anyhow!("Account not found"))
            })?;

        Ok(balance)
    }

    // /// Deposits the specified amount into the sender's account
    // pub(crate) fn deposit(
    //     &self,
    //     amount: u128,
    //     context: &Context<S>,
    //     state: &mut impl TxState<S>,
    // ) -> Result<CallResponse> {
    //     let mut account = self
    //         .accounts
    //         .get(&OwnerAddress::new(context.sender()), state)?
    //         .ok_or_else(|| anyhow::anyhow!("Account not found"))?;
    //     account.credit += amount;
    //     self.accounts
    //         .set(OwnerAddress::new(context.sender()), account, state);
    //     self.emit_event(
    //         state,
    //         "deposit",
    //         Event::Deposit {
    //             address: context.sender().clone(),
    //             amount,
    //         },
    //     );
    //     Ok(CallResponse::default())
    // }
    //
    // /// Withdraws the specified amount from the sender's account
    // pub(crate) fn withdraw(
    //     &self,
    //     amount: u128,
    //     context: &Context<S>,
    //     state: &mut impl TxState<S>,
    // ) -> Result<CallResponse> {
    //     let mut account = self
    //         .accounts
    //         .get(&OwnerAddress::new(context.sender()), state)?
    //         .ok_or_else(|| anyhow::anyhow!("Account not found"))?;
    //     if account.debit < amount {
    //         return Err(anyhow::anyhow!("Insufficient funds"));
    //     }
    //     account.debit -= amount;
    //     self.accounts
    //         .set(OwnerAddress::new(context.sender()), account, state);
    //     self.emit_event(
    //         state,
    //         "withdraw",
    //         Event::Withdraw {
    //             address: context.sender().clone(),
    //             amount,
    //         },
    //     );
    //     Ok(CallResponse::default())
    // }
}
