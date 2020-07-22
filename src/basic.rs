use super::*;
use base58::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Status {
    VALID = 0x00,
    DEACTIVATED = 0x01,
}

#[derive(Debug)]
pub enum KeyType {
    Ed25519VerificationKey2018,
    EcdsaSecp256k1VerificationKey2019,
}

pub fn gen_did(account_id: &str) -> String {
    String::from("did:near:") + account_id
}

pub fn check_did(did: &str) {
    let head = &did[0..9];
    assert_eq!(head, "did:near:")
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PublicKey {
    controller: String,
    public_key: Vec<u8>,
    deactivated: bool,
    is_pk_list: bool,
    is_authentication: bool,
}

impl PublicKey {
    pub fn new_pk_and_auth(controller: &str, pk: Vec<u8>) -> Self {
        //        let mut tp: String = "".to_string();
        //        match pk[0] {
        //            0 => tp = ED25519,
        //            1 => tp = SECP256K1,
        //            _ => {}
        //        }
        PublicKey {
            controller: controller.to_string(),
            public_key: pk,
            deactivated: false,
            is_pk_list: true,
            is_authentication: true,
        }
    }

    pub fn new_pk(controller: &str, pk: Vec<u8>) -> Self {
        PublicKey {
            controller: controller.to_string(),
            public_key: pk,
            deactivated: false,
            is_pk_list: true,
            is_authentication: false,
        }
    }

    pub fn new_auth(controller: &str, pk: Vec<u8>) -> Self {
        PublicKey {
            controller: controller.to_string(),
            public_key: pk,
            deactivated: false,
            is_pk_list: false,
            is_authentication: true,
        }
    }
}

pub fn pk_exist(key_list: &Vec<PublicKey>, pk: &Vec<u8>) -> bool {
    for v in key_list.iter() {
        if &v.public_key == pk {
            return true;
        }
    }
    return false;
}

pub fn deactivate_pk(key_list: &mut Vec<PublicKey>, pk: &Vec<u8>) {
    for v in key_list.iter_mut() {
        if &v.public_key == pk {
            if v.deactivated {
                env::panic(b"deactivate_pk, pk is deactivated")
            }
            v.deactivated = true;
            return;
        }
    }
    env::panic(b"deactivate_pk, pk doesn't exist")
}

pub fn check_pk_access(key_list: &Vec<PublicKey>, pk: &Vec<u8>) {
    for v in key_list.iter() {
        if &v.public_key == pk {
            if v.deactivated {
                env::panic(b"check_pk_access, pk is deactivated")
            }
            if !v.is_authentication {
                env::panic(b"check_pk_access, pk is not authentication")
            }
            return;
        }
    }
    env::panic(b"check_pk_access, pk doesn't exist")
}

pub fn set_pk_auth(key_list: &mut Vec<PublicKey>, pk: &Vec<u8>) -> usize {
    for (index, v) in key_list.iter_mut().enumerate() {
        if &v.public_key == pk {
            if v.deactivated {
                env::panic(b"set_pk_auth, pk is deactivated")
            }
            if v.is_authentication {
                env::panic(b"set_pk_auth, pk is already auth key")
            }
            v.is_authentication = true;
            return index;
        }
    }
    env::panic(b"set_pk_auth, pk doesn't exist")
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct PublicKeyJson {
    id: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    tp: String,
    controller: String,
    #[serde(rename(serialize = "publicKeyBase58", deserialize = "publicKeyBase58"))]
    public_key_base58: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub tp: String,
    #[serde(rename(serialize = "serviceEndpoint", deserialize = "serviceEndpoint"))]
    pub service_endpoint: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct DocumentJson<T> {
    #[serde(rename(serialize = "@contexts", deserialize = "@contexts"))]
    contexts: Vec<String>,
    id: String,
    #[serde(rename(serialize = "publicKey", deserialize = "publicKey"))]
    public_key: Vec<PublicKeyJson>,
    authentication: Vec<T>,
    controller: Vec<String>,
    service: Vec<Service>,
    created: u32,
    updated: u32,
}
