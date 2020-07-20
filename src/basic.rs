use super::*;

pub enum Status {
    NotExist = 0x00,
    VALID = 0x01,
    DeActive = 0x02,
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

pub struct DocumentJson {
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
