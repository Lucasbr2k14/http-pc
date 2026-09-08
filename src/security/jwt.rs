use jsonwebtoken::{
    encode,
    decode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};

use serde:: {
    Serialize,
    Deserialize
};

pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize
}


pub fn generate_token(
    user_uuid: &str,
    role: &str,
    exp:&str,
    secret:&str
) {
    let claims = Claims {
        sub: user_uuid.to_string(),
        role: role.to_string(),
        exp: exp
    };

    
}