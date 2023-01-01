use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    uid: u32,
    exp: usize,
}

/// normal token exp: 1h
pub const EXP_TOKEN: u64 = 1 * 60 * 60;
/// refresh token exp: 24h
pub const EXP_REFRESH: u64 = 24 * 60 * 60;

pub fn jwt_secrets() -> String {
    match std::env::var("JWT_SECRETS") {
        Ok(v) => v,
        Err(_) => "secrets".to_string()
    }.as_str().to_string()
}

pub fn jwt_encode(uid: u32, exp: SystemTime) -> Result<String> {
    match encode(&Header::default(), &Claims {
        uid,
        exp: exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
    }, &EncodingKey::from_secret(jwt_secrets().as_ref())) {
        Ok(t) => Ok(t),
        Err(e) => Err(anyhow!(e.to_string()))
    }
}

pub fn jwt_decode(token: &str) -> Result<TokenData<Claims>> {
    match decode::<Claims>(token, &DecodingKey::from_secret(jwt_secrets().as_ref()), &Validation::default()) {
        Ok(r) => Ok(r),
        Err(e) => Err(anyhow!(e.to_string()))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenDB {
    pub token: String,
    pub exp: SystemTime,
    pub uid: u32,
}

impl Default for TokenDB {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            exp: SystemTime::UNIX_EPOCH,
            uid: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::{SystemTime, UNIX_EPOCH};
    use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
    use crate::jwt::{Claims, jwt_secrets};
    use anyhow::Result;

    #[test]
    fn test_jwt_encode_decode() -> Result<()> {
        let uid = 1_u32;
        let my_claims = Claims {
            uid,
            exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 30) as usize,
        };
        let token_encoded = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_secrets().as_ref()))?;
        println!("enc: {}", token_encoded);
        let token_decoded = decode::<Claims>(&token_encoded, &DecodingKey::from_secret(jwt_secrets().as_ref()), &Validation::default())?;
        println!("dec: {:?}", token_decoded);
        assert_eq!(uid, token_decoded.claims.uid);
        Ok(())
    }
}