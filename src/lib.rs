use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
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
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut did = Self {
            status: UnorderedMap::default(),
            contexts: UnorderedMap::default(),
            public_key: UnorderedMap::default(),
            controller: UnorderedMap::default(),
            service: UnorderedMap::default(),
            created: UnorderedMap::default(),
            updated: UnorderedMap::default(),
        };
        did
    }
    pub fn reg_id(&mut self) -> bool {
        let account_id = env::signer_account_id();
        let account_id = env::signer_account_pk();
        true
    }
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
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
