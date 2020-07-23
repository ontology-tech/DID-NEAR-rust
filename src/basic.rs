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

impl KeyType {
    pub fn to_string(&self) -> String {
        match self {
            KeyType::Ed25519VerificationKey2018 => "Ed25519VerificationKey2018".to_string(),
            KeyType::EcdsaSecp256k1VerificationKey2019 => {
                "EcdsaSecp256k1VerificationKey2019".to_string()
            }
        }
    }
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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PublicKeyList {
    public_key_list: Vec<PublicKey>,
}

impl PublicKeyList {
    pub fn new_default(controller: &str, pk: Vec<u8>) -> Self {
        PublicKeyList {
            public_key_list: vec![PublicKey::new_pk_and_auth(controller, pk)],
        }
    }

    pub fn push(&mut self, pk: PublicKey) {
        self.public_key_list.push(pk);
    }

    pub fn len(&self) -> u32 {
        self.public_key_list.len() as u32
    }

    pub fn pk_exist(&self, pk: &Vec<u8>) -> bool {
        for v in self.public_key_list.iter() {
            if &v.public_key == pk {
                return true;
            }
        }
        return false;
    }

    pub fn deactivate_pk(&mut self, pk: &Vec<u8>) {
        for v in self.public_key_list.iter_mut() {
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

    pub fn check_pk_access(&self, pk: &Vec<u8>) {
        for v in self.public_key_list.iter() {
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

    pub fn set_pk_auth(&mut self, pk: &Vec<u8>) -> usize {
        for (index, v) in self.public_key_list.iter_mut().enumerate() {
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

    pub fn remove_pk_auth(&mut self, pk: &Vec<u8>) -> usize {
        for (index, v) in self.public_key_list.iter_mut().enumerate() {
            if &v.public_key == pk {
                if v.deactivated {
                    env::panic(b"remove_pk_auth, pk is deactivated")
                }
                if !v.is_authentication {
                    env::panic(b"remove_pk_auth, pk is not auth key")
                }
                v.is_authentication = false;
                return index;
            }
        }
        env::panic(b"remove_pk_auth, pk doesn't exist")
    }

    pub fn get_pk_json(&self, did: &str) -> Vec<PublicKeyJson> {
        let mut result = vec![];
        for (i, v) in self.public_key_list.iter().enumerate() {
            let mut tp: String = "".to_string();
            match v.public_key[0] {
                0 => tp = KeyType::Ed25519VerificationKey2018.to_string(),
                1 => tp = KeyType::EcdsaSecp256k1VerificationKey2019.to_string(),
                _ => {}
            }
            let public_key_json = PublicKeyJson {
                id: format!("{}#keys-{}", did, i + 1),
                tp,
                controller: v.controller.clone(),
                public_key_base58: v.public_key.to_base58(),
            };
            result.push(public_key_json);
        }
        result
    }

    pub fn get_authentication_json(
        &self,
        did: &str,
        authentication_list: Vec<u32>,
    ) -> Vec<Authentication> {
        let mut result = vec![];
        for i in authentication_list.iter() {
            let public_key: &PublicKey = self.public_key_list.get(*i as usize).unwrap();
            if public_key.is_pk_list {
                let authentication = Authentication::Pk(format!("{}#keys-{}", did, i + 1));
                result.push(authentication);
            } else {
                let mut tp: String = "".to_string();
                match public_key.public_key[0] {
                    0 => tp = KeyType::Ed25519VerificationKey2018.to_string(),
                    1 => tp = KeyType::EcdsaSecp256k1VerificationKey2019.to_string(),
                    _ => {}
                }
                let authentication = Authentication::NotPK(PublicKeyJson {
                    id: format!("{}#keys-{}", did, i + 1),
                    tp,
                    controller: public_key.controller.clone(),
                    public_key_base58: public_key.public_key.to_base58(),
                });
                result.push(authentication);
            }
        }
        result
    }
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

#[cfg(test)]
impl Service {
    pub fn new(id: String, tp: String, service_endpoint: String) -> Self {
        Service {
            id,
            tp,
            service_endpoint,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Authentication {
    Pk(String),
    NotPK(PublicKeyJson),
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename(serialize = "@contexts", deserialize = "@contexts"))]
    pub contexts: Vec<String>,
    pub id: String,
    #[serde(rename(serialize = "publicKey", deserialize = "publicKey"))]
    pub public_key: Vec<PublicKeyJson>,
    pub authentication: Vec<Authentication>,
    pub controller: Vec<String>,
    pub service: Vec<Service>,
    pub created: u64,
    pub updated: u64,
}
