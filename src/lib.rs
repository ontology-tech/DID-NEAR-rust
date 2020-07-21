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
    key_index: UnorderedMap<String, u32>,
    contexts: UnorderedMap<String, Vec<String>>,
    public_key: UnorderedMap<String, Vec<PublicKey>>,
    authentication: UnorderedMap<String, Vec<u32>>,
    controller: UnorderedMap<String, Vec<String>>,
    service: UnorderedMap<String, Vec<Service>>,
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

        let log_message = format!("register: {}", &account_id);
        env::log(log_message.as_bytes());
    }
    pub fn get_did(&self, id: AccountId) -> u64 {
        self.created.get(&id).unwrap()
    }

    pub fn deactive_did(&mut self) {
        let account_id = env::signer_account_id();
        let status = self.status.get(&account_id);
        assert!(status.is_some());
        self.status.insert(&account_id, &Status::DeActive);
        self.contexts.remove(&account_id);
        self.public_key.remove(&account_id);
        self.controller.remove(&account_id);
        self.service.remove(&account_id);
        self.created.remove(&account_id);
        self.updated.remove(&account_id);
        let log_message = format!("deactive: {}", &account_id);
        env::log(log_message.as_bytes());
    }
    pub fn add_key(&mut self, pk: Vec<u8>) {}
    pub fn remove_key(&mut self, pk: Vec<u8>) {}
    pub fn add_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let mut sers = self.service.get(&account_id).unwrap_or(vec![]);
        let index = sers.iter().position(|x| &x.id == &ser.id);
        let log_message = format!("method:{}, service id: {}", "add_service", &ser.id);
        if index.is_none() {
            sers.push(ser);
            self.service.insert(&account_id, &sers);
        }
        env::log(log_message.as_bytes());
    }

    pub fn update_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let mut sers = self.service.get(&account_id).unwrap_or(vec![]);
        let index = sers.iter().position(|x| &x.id == &ser.id);
        let log_message = format!("method:{}, service id: {}", "update_service", &ser.id);
        if let Some(ind) = index {
            let res = sers.get_mut(ind).unwrap();
            res.id = ser.id;
            res.tp = ser.tp;
            res.service_endpoint = ser.service_endpoint;
        }
        env::log(log_message.as_bytes());
    }
    pub fn remove_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let mut sers = self.service.get(&account_id).unwrap_or(vec![]);
        let index = sers.iter().position(|x| &x.id == &ser.id);
        let log_message = format!("method:{}, service id: {}", "remove_service", &ser.id);
        if let Some(ind) = index {
            let res = sers.get_mut(ind).unwrap();
            res.id = ser.id;
            res.tp = ser.tp;
            res.service_endpoint = ser.service_endpoint;
        }
        env::log(log_message.as_bytes());
    }
    pub fn add_context(&mut self, context: String) {
        let log_message = format!("method:{}, service id: {}", "add_context", &context);
        let account_id = env::signer_account_id();
        let mut cons = self.contexts.get(&account_id).unwrap_or(vec![]);
        if !cons.contains(&context) {
            cons.push(context);
        }
        env::log(log_message.as_bytes());
    }
    pub fn remove_context(&mut self, context: String) {
        let log_message = format!("method:{}, service id: {}", "remove_context", &context);
        let account_id = env::signer_account_id();
        let mut cons = self.contexts.get(&account_id).unwrap_or(vec![]);
        let index = cons.iter().position(|x| x == &context);
        if let Some(ind) = index {
            cons.remove(ind);
        }
        env::log(log_message.as_bytes());
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
    }
}
