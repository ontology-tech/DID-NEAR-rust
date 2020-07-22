use super::*;
use near_sdk::MockedBlockchain;
use near_sdk::{testing_env, VMContext};

fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
        current_account_id: "alice_near".to_string(),
        signer_account_id: "bob_near".to_string(),
        signer_account_pk: vec![0, 1, 2],
        predecessor_account_id: "carol_near".to_string(),
        input,
        block_index: 0,
        block_timestamp: 0,
        account_balance: 0,
        account_locked_balance: 0,
        storage_usage: 0,
        attached_deposit: 0,
        prepaid_gas: 10u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view,
        output_data_receivers: vec![],
        epoch_height: 0,
    }
}

#[test]
fn reg_did_using_account() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = DID::default();
    contract.reg_did_using_account();
    //    contract.deactive_did();
    contract.add_controller("did:near:abcde.testnet".to_string());
    contract.remove_controller("did:near:abcde.testnet".to_string());
    contract.add_key(vec![1u8], "did:near:abcde.testnet".to_string());
    //    contract.deactive_key(vec![1u8]);
    contract.add_new_auth_key(vec![2u8], "did:near:abcde.testnet".to_string());
    contract.set_auth_key(vec![1u8]);
}
