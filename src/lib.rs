use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};
use serde::{Deserialize, Serialize};

mod basic;
use basic::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct DID {
    status: UnorderedMap<String, Status>,
    contexts: UnorderedMap<String, Vec<String>>,
    public_key: UnorderedMap<String, PublicKey>,
    controller: UnorderedMap<String, Vec<String>>,
    service: UnorderedMap<String, Service>,
    created: UnorderedMap<String, u32>,
    updated: UnorderedMap<String, u32>,
}

#[near_bindgen]
impl DID {
    pub fn reg_id_with_public_key(&mut self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        let did = gen_did(&account_id);
        let status = self.status.get(&did);
        assert!(status.is_none());
        self.status.insert(&did, &Status::VALID);
        self.public_key.insert(&did, &PublicKey::new_pk_and_auth(&did, account_pk));
    }
}

fn gen_did(account_id: &str) -> String {
    String::from("did:near:") + account_id
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
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
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = DID::default();
        contract.reg_id_with_public_key();
    }
}
