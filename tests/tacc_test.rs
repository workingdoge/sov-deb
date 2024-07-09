use std::convert::Infallible;

use deb::{AccountAddress, CallMessage, TAccountModule, TAccountModuleConfig};
use sov_modules_api::{
    utils::generate_address, ApiStateAccessor, Context, Module, StateCheckpoint, WorkingSet,
};
use sov_prover_storage_manager::new_orphan_storage;

type S = sov_test_utils::TestSpec;
#[test]
fn create_tacc() -> Result<(), Infallible> {
    let tacc = TAccountModule::<S>::default();
    let tmpdir = tempfile::tempdir().unwrap();
    let state = StateCheckpoint::<S>::new(new_orphan_storage(tmpdir.path()).unwrap());

    let owner = generate_address::<S>("owner");
    let owner_address = AccountAddress::<S>::new(&owner);
    let sequencer_address = generate_address::<S>("sequencer");
    let owner_context = Context::<S>::new(owner, Default::default(), sequencer_address, 1);

    let config = TAccountModuleConfig {};
    let mut genesis = state.to_genesis_state_accessor::<TAccountModule<S>>(&config);
    tacc.genesis(&config, &mut genesis).unwrap();

    let mut state = genesis.checkpoint().to_working_set_unmetered();

    let acc_message = CallMessage::CreateAccount;
    tacc.call(acc_message.clone(), &owner_context, &mut state)
        .expect("Failed to create account");

    assert_eq!(state.events().len(), 1);

    let balance = tacc.get_balance(&owner_address, &mut state).unwrap();
    assert_eq!(balance.1, 0);

    Ok(())
}
