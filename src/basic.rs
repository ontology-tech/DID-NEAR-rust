use super::*;
use base58::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Status {
    VALID = 0x00,
    DeActive = 0x01,
}

const FIELD_CONTEXT: u8 = 0;
const FIELD_PK: u8 = 1;
const FIELD_CONTROLLER: u8 = 2;
const FIELD_SERVICE: u8 = 3;
const FIELD_CREATED: u8 = 4;
const FIELD_UPDATED: u8 = 5;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PublicKey {
    tp: String,
    controller: String,
    public_key_base58: String,
    de_actived: bool,
    is_pk_list: bool,
    is_authentication: bool,
}

impl PublicKey {
    pub fn new_pk_and_auth(controller: &str, pk: Vec<u8>) -> Self {
        PublicKey {
            tp: "".to_string(),
            controller: controller.to_string(),
            public_key_base58: pk.to_base58(),
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
