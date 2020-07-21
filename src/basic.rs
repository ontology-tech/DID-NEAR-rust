use super::*;
use base58::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Status {
    VALID = 0x00,
    DeActive = 0x01,
}

#[derive(Debug)]
pub enum KeyType {
    Ed25519VerificationKey2018,
    EcdsaSecp256k1VerificationKey2019,
}

const FIELD_CONTEXT: u8 = 0;
const FIELD_PK: u8 = 1;
const FIELD_CONTROLLER: u8 = 2;
const FIELD_SERVICE: u8 = 3;
const FIELD_CREATED: u8 = 4;
const FIELD_UPDATED: u8 = 5;
const PUBLIC_KEY_TOTAL_SIZE: u32 = 1024 * 1024;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PublicKey {
    controller: String,
    public_key: Vec<u8>,
    de_actived: bool,
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
            de_actived: false,
            is_pk_list: true,
            is_authentication: true,
        }
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
    key: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    tp: String,
    #[serde(rename(serialize = "serviceEndpoint", deserialize = "serviceEndpoint"))]
    service_endpoint: String,
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
