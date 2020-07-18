use super::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ID {
    context: Vec<Vec<u8>>,
    id: Vec<u8>,
    public_key: Vec<PublicKey>,
    owners: UnorderedSet<String>,
    controllers: UnorderedSet<String>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PublicKey {
    id: Vec<u8>,
    ty: Vec<u8>,
    controller: Vec<u8>,
    public_key_hex: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Attribute {
    key:Vec<u8>,
    ty:Vec<u8>,
    value:Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Controller {
    threshold:u16,
    members:Vec<Vec<u8>>,
}