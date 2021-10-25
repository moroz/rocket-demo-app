use hmac::Hmac;
use hmac::NewMac;
use jwt::SignWithKey;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use std::env;

fn build_signer() -> Hmac<Sha256> {
    let secret = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET not set!");
    Hmac::new_from_slice(&secret.into_bytes()[..]).unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct SessionPayload {
    pub sub: i32,
}

impl SessionPayload {
    pub fn generate_and_sign(&self) -> Result<String, jwt::error::Error> {
        let signer = build_signer();
        self.sign_with_key(&signer)
    }
}
