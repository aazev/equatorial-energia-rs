use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}


#[derive(Debug, Deserialize)]
pub struct LoginError {
    pub error: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LoginResponses {
    Success(LoginResponse),
    Error(LoginError),
}
