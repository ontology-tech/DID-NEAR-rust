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
    public_key: UnorderedMap<String, Vec<PublicKey>>,
    authentication: UnorderedMap<String, Vec<u32>>,
    controller: UnorderedMap<String, Vec<String>>,
    service: UnorderedMap<String, Service>,
    created: UnorderedMap<String, u64>,
    updated: UnorderedMap<String, u64>,
}

#[near_bindgen]
impl DID {
    pub fn reg_did_using_account(&mut self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        let status = self.status.get(&account_id);
        assert!(status.is_none());

        self.status.insert(&account_id, &Status::VALID);
        self.public_key.insert(
            &account_id,
            &vec![PublicKey::new_pk_and_auth(&account_id, account_pk)],
        );
        let index: u32 = 0;
        self.authentication.insert(&account_id, &vec![index]);
        self.created.insert(&account_id, &env::block_timestamp());

        let log_message = format!("reg_did_using_account: {}", &account_id);
        env::log(log_message.as_bytes());
    }

    pub fn deactive_did(&mut self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        let status = self.status.get(&account_id);
        assert!(status.is_some());
        let public_key_list = self.public_key.get(&account_id).unwrap();
        check_pk_access(&public_key_list, &account_pk);

        self.status.insert(&account_id, &Status::DeActive);
        self.contexts.remove(&account_id);
        self.public_key.remove(&account_id);
        self.authentication.remove(&account_id);
        self.controller.remove(&account_id);
        self.service.remove(&account_id);
        self.created.remove(&account_id);
        self.updated.remove(&account_id);

        let log_message = format!("deactive_did: {}", &account_id);
        env::log(log_message.as_bytes());
    }

    pub fn add_controller(&mut self, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        self.check_did_status(&account_id);
        let public_key_list = self.public_key.get(&account_id).unwrap();
        check_pk_access(&public_key_list, &account_pk);
        check_did(&controller);
        let mut controller_list = self.controller.get(&account_id).unwrap();
        if controller_exist(&controller_list, &controller) {
            env::panic(b"add_controller, controller exists")
        };

        controller_list.push(controller.clone());
        self.controller.insert(&account_id, &controller_list);
        self.updated.insert(&account_id, &env::block_timestamp());

        let log_message = format!(
            "add_controller, id:{}, controller: {}",
            &account_id, controller
        );
        env::log(log_message.as_bytes());
    }

    pub fn remove_controller(&mut self, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        self.check_did_status(&account_id);
        let public_key_list = self.public_key.get(&account_id).unwrap();
        check_pk_access(&public_key_list, &account_pk);

        let mut controller_list = self.controller.get(&account_id).unwrap();
        let index = controller_list
            .iter()
            .position(|x| x == &controller)
            .unwrap();
        controller_list.remove(index);
        self.controller.insert(&account_id, &controller_list);
        self.updated.insert(&account_id, &env::block_timestamp());

        let log_message = format!(
            "remove_controller, id:{}, controller: {}",
            &account_id, controller
        );
        env::log(log_message.as_bytes());
    }

    pub fn add_key(&mut self, pk: Vec<u8>, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        self.check_did_status(&account_id);
        let mut public_key_list = self.public_key.get(&account_id).unwrap();
        check_pk_access(&public_key_list, &account_pk);
        if pk_exist(&public_key_list, &pk) {
            env::panic(b"add_key, pk exists")
        }

        public_key_list.push(PublicKey::new_pk(&account_id, pk.clone()));
        self.public_key.insert(&account_id, &public_key_list);
        self.updated.insert(&account_id, &env::block_timestamp());

        let log_message = format!(
            "add_key, id:{}, public key: {:?}, controller: {}",
            &account_id, pk, controller
        );
        env::log(log_message.as_bytes());
    }

    pub fn deactive_key(&mut self, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();

        self.check_did_status(&account_id);
        let mut public_key_list = self.public_key.get(&account_id).unwrap();
        check_pk_access(&public_key_list, &account_pk);

        deactive_pk(&mut public_key_list, &pk);
        self.public_key.insert(&account_id, &public_key_list);
        self.updated.insert(&account_id, &env::block_timestamp());

        let log_message = format!("deactive_key, id:{}, public key: {:?}", &account_id, pk);
        env::log(log_message.as_bytes());
    }

    pub fn add_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let did = gen_did(&account_id);
        self.service.insert(&did, &ser);
    }

    fn check_did_status(&self, account_id: &AccountId) {
        let status = self.status.get(account_id).unwrap();
        match status {
            Status::VALID => (),
            _ => env::panic(b"did status is not valid"),
        };
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
        let mut contract = DID::default();
    }
}
