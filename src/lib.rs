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
    service: UnorderedMap<String, Vec<Service>>,
    created: UnorderedMap<String, u64>,
    updated: UnorderedMap<String, u64>,
}

#[near_bindgen]
impl DID {
    pub fn reg_did_using_account(&mut self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        let status = self.status.get(&did);
        assert!(status.is_none());

        self.status.insert(&did, &Status::VALID);
        self.public_key
            .insert(&did, &vec![PublicKey::new_pk_and_auth(&did, account_pk)]);
        let index: u32 = 0;
        self.authentication.insert(&did, &vec![index]);
        self.created.insert(&did, &env::block_timestamp());

        let log_message = format!("reg_did_using_account: {}", &did);
        env::log(log_message.as_bytes());
    }
    pub fn deactive_did(&mut self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        let status = self.status.get(&did);
        assert!(status.is_some());
        let public_key_list = self.public_key.get(&did).unwrap();
        check_pk_access(&public_key_list, &account_pk);

        self.status.insert(&did, &Status::DeActive);
        self.contexts.remove(&did);
        self.public_key.remove(&did);
        self.authentication.remove(&did);
        self.controller.remove(&did);
        self.service.remove(&did);
        self.created.remove(&did);
        self.updated.remove(&did);

        let log_message = format!("deactive_did: {}", &did);
        env::log(log_message.as_bytes());
    }

    pub fn add_controller(&mut self, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        check_pk_access(&public_key_list, &account_pk);
        check_did(&controller);
        let mut controller_list = self.controller.get(&did).unwrap();
        if controller_exist(&controller_list, &controller) {
            env::panic(b"add_controller, controller exists")
        };

        controller_list.push(controller.clone());
        self.controller.insert(&did, &controller_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!("add_controller, id:{}, controller: {}", &did, controller);
        env::log(log_message.as_bytes());
    }

    pub fn remove_controller(&mut self, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        check_pk_access(&public_key_list, &account_pk);

        let mut controller_list = self.controller.get(&did).unwrap();
        let index = controller_list
            .iter()
            .position(|x| x == &controller)
            .unwrap();
        controller_list.remove(index);
        self.controller.insert(&did, &controller_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!("remove_controller, id:{}, controller: {}", &did, controller);
        env::log(log_message.as_bytes());
    }

    pub fn add_key(&mut self, pk: Vec<u8>, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        check_pk_access(&public_key_list, &account_pk);
        if pk_exist(&public_key_list, &pk) {
            env::panic(b"add_key, pk exists")
        }

        public_key_list.push(PublicKey::new_pk(&did, pk.clone()));
        self.public_key.insert(&did, &public_key_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "add_key, id:{}, public key: {:?}, controller: {}",
            &did, pk, controller
        );
        env::log(log_message.as_bytes());
    }

    pub fn deactive_key(&mut self, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        check_pk_access(&public_key_list, &account_pk);

        deactive_pk(&mut public_key_list, &pk);
        self.public_key.insert(&did, &public_key_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!("deactive_key, id:{}, public key: {:?}", &did, pk);
        env::log(log_message.as_bytes());
    }

    pub fn add_new_auth_key(&mut self, pk: Vec<u8>, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        check_pk_access(&public_key_list, &account_pk);
        if pk_exist(&public_key_list, &pk) {
            env::panic(b"add_new_auth_key, pk exists")
        }

        public_key_list.push(PublicKey::new_auth(&did, pk.clone()));
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        let index: u32 = (public_key_list.len() - 1) as u32;
        authentication_list.push(index);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "add_new_auth_key, id:{}, public key: {:?}, controller: {}",
            &did, pk, controller
        );
        env::log(log_message.as_bytes());
    }

    pub fn set_auth_key(&mut self, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        check_pk_access(&public_key_list, &account_pk);

        let index = set_pk_auth(&mut public_key_list, &pk);
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        authentication_list.push(index as u32);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!("set_auth_key, id:{}, public key: {:?}", &did, pk);
        env::log(log_message.as_bytes());
    }

    pub fn add_new_auth_key_by_controller(&mut self, did: String, pk: Vec<u8>, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let controller_did = gen_did(&account_id);

        self.check_did_status(&did);
        self.check_did_status(&controller_did);
        let controller_list = self.controller.get(&did).unwrap();
        if !controller_list.contains(&controller_did) {
            env::panic(b"add_new_auth_key_by_controller, signer is not controller")
        }
        let controller_public_key_list = self.public_key.get(&controller_did).unwrap();
        check_pk_access(&controller_public_key_list, &account_pk);

        let mut public_key_list = self.public_key.get(&did).unwrap();
        if pk_exist(&public_key_list, &pk) {
            env::panic(b"add_new_auth_key_by_controller, pk exists")
        }

        public_key_list.push(PublicKey::new_auth(&did, pk.clone()));
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        let index: u32 = (public_key_list.len() - 1) as u32;
        authentication_list.push(index);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "add_new_auth_key_by_controller, id:{}, public key: {:?}, controller: {}",
            &did, pk, controller
        );
        env::log(log_message.as_bytes());
    }

    pub fn set_auth_key_by_controller(&mut self, did: String, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let controller_did = gen_did(&account_id);

        self.check_did_status(&did);
        self.check_did_status(&controller_did);
        let controller_list = self.controller.get(&did).unwrap();
        if !controller_list.contains(&controller_did) {
            env::panic(b"set_auth_key_by_controller, signer is not controller")
        }
        let controller_public_key_list = self.public_key.get(&controller_did).unwrap();
        check_pk_access(&controller_public_key_list, &account_pk);

        let mut public_key_list = self.public_key.get(&did).unwrap();
        let index = set_pk_auth(&mut public_key_list, &pk);
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        authentication_list.push(index as u32);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "set_auth_key_by_controller, id:{}, public key: {:?}",
            &did, pk
        );
        env::log(log_message.as_bytes());
    }

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

    fn check_did_status(&self, did: &String) {
        let status = self.status.get(did).unwrap();
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
