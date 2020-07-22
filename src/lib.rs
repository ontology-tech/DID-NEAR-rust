use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};
use serde::{Deserialize, Serialize};

mod basic;
use basic::*;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct DID {
    status: UnorderedMap<String, Status>,
    contexts: UnorderedMap<String, Vec<String>>,
    public_key: UnorderedMap<String, PublicKeyList>,
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
            .insert(&did, &PublicKeyList::new_default(&did, account_pk));
        let index: u32 = 0;
        self.authentication.insert(&did, &vec![index]);
        self.created.insert(&did, &env::block_timestamp());

        let log_message = format!("reg_did_using_account: {}", &did);
        env::log(log_message.as_bytes());
    }

    pub fn deactivate_did(&mut self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        let status = self.status.get(&did);
        assert!(status.is_some());
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        self.status.insert(&did, &Status::DEACTIVATED);
        self.contexts.remove(&did);
        self.public_key.remove(&did);
        self.authentication.remove(&did);
        self.controller.remove(&did);
        self.service.remove(&did);
        self.created.remove(&did);
        self.updated.remove(&did);

        let log_message = format!("deactivate_did: {}", &did);
        env::log(log_message.as_bytes());
    }

    pub fn add_controller(&mut self, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        let log_message = format!("add_controller, did:{}, controller: {}", &did, &controller);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);
        check_did(&controller);
        let mut controller_list = self.controller.get(&did).unwrap_or(vec![]);
        if controller_list.contains(&controller) {
            env::panic(b"add_controller, controller exists")
        };

        controller_list.push(controller);
        self.controller.insert(&did, &controller_list);
        self.updated.insert(&did, &env::block_timestamp());
        env::log(log_message.as_bytes());
    }

    pub fn remove_controller(&mut self, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let mut controller_list = self.controller.get(&did).unwrap();
        let index = controller_list
            .iter()
            .position(|x| x == &controller)
            .unwrap();
        controller_list.remove(index);
        self.controller.insert(&did, &controller_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "remove_controller, did:{}, controller: {}",
            &did, controller
        );
        env::log(log_message.as_bytes());
    }

    pub fn add_key(&mut self, pk: Vec<u8>, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        let log_message = format!(
            "add_key, did:{}, public key: {:?}, controller: {}",
            &did, &pk, &controller
        );

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);
        if public_key_list.pk_exist(&pk) {
            env::panic(b"add_key, pk exists")
        }

        public_key_list.push(PublicKey::new_pk(&did, pk));
        self.public_key.insert(&did, &public_key_list);
        self.updated.insert(&did, &env::block_timestamp());

        env::log(log_message.as_bytes());
    }

    pub fn deactivate_key(&mut self, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        public_key_list.deactivate_pk(&pk);
        self.public_key.insert(&did, &public_key_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!("deactivate_key, did:{}, public key: {:?}", &did, pk);
        env::log(log_message.as_bytes());
    }

    pub fn add_new_auth_key(&mut self, pk: Vec<u8>, controller: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);
        if public_key_list.pk_exist(&pk) {
            env::panic(b"add_new_auth_key, pk exists")
        }

        let log_message = format!(
            "add_new_auth_key, did:{}, public key: {:?}, controller: {}",
            &did, &pk, &controller
        );

        public_key_list.push(PublicKey::new_auth(&did, pk));
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        let index: u32 = (public_key_list.len() - 1) as u32;
        authentication_list.push(index);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        env::log(log_message.as_bytes());
    }

    pub fn set_auth_key(&mut self, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let index = public_key_list.set_pk_auth(&pk);
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        authentication_list.push(index as u32);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!("set_auth_key, did:{}, public key: {:?}", &did, pk);
        env::log(log_message.as_bytes());
    }

    pub fn deactivate_auth_key(&mut self, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let mut public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let index = public_key_list.remove_pk_auth(&pk);
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        let i = authentication_list
            .iter()
            .position(|x| x == &(index as u32))
            .unwrap();
        authentication_list.remove(i);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!("deactivate_auth_key, did:{}, public key: {:?}", &did, pk);
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
        controller_public_key_list.check_pk_access(&account_pk);

        let mut public_key_list = self.public_key.get(&did).unwrap();
        if public_key_list.pk_exist(&pk) {
            env::panic(b"add_new_auth_key_by_controller, pk exists")
        }

        public_key_list.push(PublicKey::new_auth(&did, pk.clone()));
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        let index: u32 = public_key_list.len() - 1;
        authentication_list.push(index);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "add_new_auth_key_by_controller, did:{}, public key: {:?}, controller: {}",
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
        controller_public_key_list.check_pk_access(&account_pk);

        let mut public_key_list = self.public_key.get(&did).unwrap();
        let index = public_key_list.set_pk_auth(&pk);
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        authentication_list.push(index as u32);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "set_auth_key_by_controller, did:{}, public key: {:?}",
            &did, pk
        );
        env::log(log_message.as_bytes());
    }

    pub fn deactivate_auth_key_by_controller(&mut self, did: String, pk: Vec<u8>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let controller_did = gen_did(&account_id);

        self.check_did_status(&did);
        self.check_did_status(&controller_did);
        let controller_list = self.controller.get(&did).unwrap();
        if !controller_list.contains(&controller_did) {
            env::panic(b"deactivate_auth_key_by_controller, signer is not controller")
        }
        let controller_public_key_list = self.public_key.get(&controller_did).unwrap();
        controller_public_key_list.check_pk_access(&account_pk);

        let mut public_key_list = self.public_key.get(&did).unwrap();
        let index = public_key_list.remove_pk_auth(&pk);
        self.public_key.insert(&did, &public_key_list);
        let mut authentication_list = self.authentication.get(&did).unwrap();
        let i = authentication_list
            .iter()
            .position(|x| x == &(index as u32))
            .unwrap();
        authentication_list.remove(i);
        self.authentication.insert(&did, &authentication_list);
        self.updated.insert(&did, &env::block_timestamp());

        let log_message = format!(
            "deactivate_auth_key_by_controller, did:{}, public key: {:?}",
            &did, pk
        );
        env::log(log_message.as_bytes());
    }

    pub fn add_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let mut sers = self.service.get(&did).unwrap_or(vec![]);
        let index = sers.iter().position(|x| &x.id == &ser.id);
        let log_message = format!("add_service, did:{}, service id: {}", &did, &ser.id);
        if !index.is_none() {
            env::panic(b"add_service, service exists")
        }
        sers.push(ser);
        self.service.insert(&did, &sers);
        env::log(log_message.as_bytes());
    }

    pub fn update_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let mut sers = self.service.get(&did).unwrap_or(vec![]);
        let index = sers.iter().position(|x| &x.id == &ser.id);
        let log_message = format!("update_service, did:{}, service id: {}", &did, &ser.id);
        match index {
            Some(ind) => {
                let res = sers.get_mut(ind).unwrap();
                res.id = ser.id;
                res.tp = ser.tp;
                res.service_endpoint = ser.service_endpoint;
                self.service.insert(&did, &sers);
            }
            _ => env::panic(b"update_service, service doesn't exist"),
        }
        env::log(log_message.as_bytes());
    }

    pub fn remove_service(&mut self, ser: Service) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let mut sers = self.service.get(&did).unwrap_or(vec![]);
        let index = sers.iter().position(|x| &x.id == &ser.id);
        let log_message = format!("remove_service, did:{}, service id: {}", &did, &ser.id);
        match index {
            Some(ind) => {
                sers.remove(ind);
                self.service.insert(&did, &sers);
            }
            _ => env::panic(b"remove_service, service doesn't exist"),
        }
        env::log(log_message.as_bytes());
    }

    pub fn add_context(&mut self, context: Vec<String>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let log_message = format!("add_context, did:{}, service id: {:?}", &did, &context);
        let mut cons = self.contexts.get(&did).unwrap_or(vec![]);
        for v in context.iter() {
            if !cons.contains(v) {
                cons.push(v.clone());
            };
        }
        self.contexts.insert(&did, &cons);
        env::log(log_message.as_bytes());
    }

    pub fn remove_context(&mut self, context: Vec<String>) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);

        let mut cons = self.contexts.get(&did).unwrap_or(vec![]);
        for v in context.iter() {
            let index = cons.iter().position(|x| x == v);
            if let Some(ind) = index {
                cons.remove(ind);
            }
        }

        let log_message = format!("remove_context, did: {}, service id: {:?}", &did, &context);
        env::log(log_message.as_bytes());
    }

    pub fn verify_signature(&self) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let did = gen_did(&account_id);

        self.check_did_status(&did);
        let public_key_list = self.public_key.get(&did).unwrap();
        public_key_list.check_pk_access(&account_pk);
    }

    pub fn verify_controller(&self, did: String) {
        let account_id = env::signer_account_id();
        let account_pk = env::signer_account_pk();
        let controller_did = gen_did(&account_id);

        self.check_did_status(&did);
        self.check_did_status(&controller_did);
        let controller_list = self.controller.get(&did).unwrap();
        if !controller_list.contains(&controller_did) {
            env::panic(b"verify_controller, signer is not controller")
        }
        let controller_public_key_list = self.public_key.get(&controller_did).unwrap();
        controller_public_key_list.check_pk_access(&account_pk);
    }

    fn check_did_status(&self, did: &String) {
        let status = self.status.get(did).unwrap();
        match status {
            Status::VALID => (),
            _ => env::panic(b"did status is not valid"),
        };
    }
}
