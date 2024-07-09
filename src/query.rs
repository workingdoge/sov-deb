use jsonrpsee::core::RpcResult;
use jsonrpsee::types::ErrorCode;
use serde::{Deserialize, Serialize};
use sov_modules_api::macros::rpc_gen;
use sov_modules_api::prelude::UnwrapInfallible;
use sov_modules_api::{ApiStateAccessor, Spec, StateReader};
use sov_state::User;

use crate::{AccountAddress, BalanceType, Deb};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountResponse {
    pub debit: u128,
    pub credit: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub balance_type: BalanceType,
    pub amount: u128,
}

impl<S: Spec> Deb<S> {
    pub fn account<Reader: StateReader<User>>(
        &self,
        owner: AccountAddress<S>,
        accessor: &mut Reader,
    ) -> Result<Option<AccountResponse>, Reader::Error> {
        let account = self.accounts.get(&owner, accessor)?;

        Ok(account.map(|account| AccountResponse {
            debit: account.debit(),
            credit: account.credit(),
        }))
    }

    // pub fn balance<Reader: StateReader<User>>(
    //     &self,
    //     owner: OwnerAddress<S>,
    //     accessor: &mut Reader,
    // ) -> Result<Option<BalanceResponse>, Reader::Error> {
    //     let account = self.accounts.get(&owner, accessor)?;
    //
    //     Ok(account.map(|account| {
    //         let (balance_type, amount) = account.balance();
    //         BalanceResponse {
    //             balance_type,
    //             amount,
    //         }
    //     }))
    // }
}

#[rpc_gen(client, server, namespace = "tacc")]
impl<S: Spec> Deb<S> {
    #[rpc_method(name = "getAccount")]
    pub fn get_account(
        &self,
        owner: AccountAddress<S>,
        state: &mut ApiStateAccessor<S>,
    ) -> RpcResult<AccountResponse> {
        self.account(owner, state)
            .unwrap_infallible()
            .ok_or(ErrorCode::InvalidParams.into())
    }

    // #[rpc_method(name = "getBalance")]
    // pub fn get_balance(
    //     &self,
    //     owner: OwnerAddress<S>,
    //     state: &mut ApiStateAccessor<S>,
    // ) -> RpcResult<BalanceResponse> {
    //     self.balance(owner, state)
    //         .unwrap_infallible()
    //         .ok_or(ErrorCode::InvalidParams.into())
    // }
}
