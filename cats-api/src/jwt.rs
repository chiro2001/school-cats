use serde::*;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    token: String,
    exp: usize,
}

pub fn jwt_secrets() -> String {
    match std::env::var("ENV_JWT_SECRETS") {
        Ok(v) => v,
        Err(_) => "secrets".to_string()
    }.as_str().to_string()
}

#[cfg(test)]
mod test {
    use std::time::{SystemTime, UNIX_EPOCH};
    use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
    use crate::jwt::{Claims, jwt_secrets};
    use anyhow::Result;

    #[test]
    fn test_jwt_encode_decode() -> Result<()> {
        let token = "token".to_string();
        let my_claims = Claims {
            username: "chiro".to_string(),
            token: token.clone(),
            exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 30) as usize,
        };
        let token_encoded = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_secrets().as_ref()))?;
        println!("enc: {}", token_encoded);
        let token_decoded = decode::<Claims>(&token_encoded, &DecodingKey::from_secret(jwt_secrets().as_ref()), &Validation::default())?;
        println!("dec: {:?}", token_decoded);
        assert_eq!(token, token_decoded.claims.token);
        Ok(())
    }
}