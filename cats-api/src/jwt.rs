use serde::*;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
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
    use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
    use crate::jwt::{Claims, jwt_secrets};
    use anyhow::Result;

    #[test]
    fn test_jwt_encode_decode() -> Result<()> {
        let my_claims = Claims { username: "chiro".to_string(), exp: 1000 };
        let token_encoded = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_secrets().as_ref()))?;
        println!("enc: {}", token_encoded);
        let token_decoded = decode::<Claims>(&token_encoded, &DecodingKey::from_secret(jwt_secrets().as_ref()), &Validation::default())?;
        println!("dec: {:?}", token_decoded);
        Ok(())
    }
}