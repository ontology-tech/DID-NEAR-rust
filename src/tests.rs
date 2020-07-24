use super::*;
use near_sdk::MockedBlockchain;
use near_sdk::{testing_env, VMContext};

fn get_context(signer_id: String, input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
        current_account_id: "alice_near".to_string(),
        signer_account_id: signer_id,
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
fn controller_test() {
    let context = get_context("bob_near".to_string(), vec![], false);
    testing_env!(context);
    let mut contract = DID::default();
    contract.reg_did_using_account();

    //    contract.deactivate_did();
    let did = "did:near:abcde.testnet".to_string();
    contract.add_controller(did.clone());
    contract.remove_controller(did.clone());

    let doc = contract.get_document(did.clone());
    assert!(doc.is_some());
    println!("doc2:{}", doc.unwrap());
}

#[test]
fn auth_key_test() {
    let context = get_context("bob_near".to_string(), vec![], false);
    testing_env!(context);
    let mut contract = DID::default();
    contract.reg_did_using_account();
    //    contract.deactivate_did();
    contract.add_key(vec![1u8], "did:near:abcde.testnet".to_string());
    //    contract.deactivate_key(vec![1u8]);
    contract.add_new_auth_key(vec![2u8], "did:near:abcde.testnet".to_string());
    contract.set_auth_key(vec![1u8]);
    contract.deactivate_auth_key(vec![1u8]);
    let controller = "did:near:bob_near".to_string();

    contract.add_controller(controller.clone());
    let did = "did:near:bob_near".to_string();
    let pk = vec![0u8, 1u8];
    contract.add_new_auth_key_by_controller(did.clone(), pk.clone(), controller.clone());
    contract.verify_controller(did.clone());

    let pk = vec![0u8, 1u8, 3u8];
    contract.add_key(pk.clone(), "did:near:abcde.testnet".to_string());
    contract.set_auth_key_by_controller(did.clone(), pk.clone());
    contract.deactivate_auth_key_by_controller(did.clone(), pk.clone());

    let doc = contract.get_document(did.clone());
    assert!(doc.is_some());
    println!("res:{}", doc.unwrap());
}

#[test]
fn service_test() {
    let context = get_context("bob_near".to_string(), vec![], false);
    testing_env!(context);
    let mut contract = DID::default();
    contract.reg_did_using_account();
    let ser = Service::new("id".to_string(), "tp".to_string(), "ss".to_string());
    contract.add_service(ser);
    let ser = Service::new("id".to_string(), "tp2".to_string(), "ss2".to_string());
    contract.update_service(ser);
    contract.remove_service("id".to_string());
}

#[test]
fn context_test() {
    let context = get_context("bob_near".to_string(), vec![], false);
    testing_env!(context);
    let mut contract = DID::default();
    contract.reg_did_using_account();
    let con = vec!["conext".to_string()];
    contract.add_context(con.clone());
    contract.remove_context(con.clone());

    contract.verify_signature();

    let did = "did:near:bob_near".to_string();
    let res = contract.get_document(did.clone());
    assert!(res.is_some());
    println!("res:{}", res.unwrap());
}
