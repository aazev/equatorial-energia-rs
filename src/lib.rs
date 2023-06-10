pub mod responses;

use reqwest::{Result, StatusCode};
use responses::login::{LoginResponses, LoginResponse};

pub const AUTHORIZATION_BASIC_TOKEN: &str = "Basic RXF1YXRvcmlhbFNpdGU6R216MnNpdGU=";

pub enum States{
    AL,
    MA,
    PA,
    PI
}

impl ToString for States{
    fn to_string(&self) -> String {
        match self{
            States::AL => "AL".to_string(),
            States::MA => "MA".to_string(),
            States::PA => "PA".to_string(),
            States::PI => "PI".to_string(),
        }
    }
}

impl TryFrom<&str> for States{
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "AL" => Ok(States::AL),
            "MA" => Ok(States::MA),
            "PA" => Ok(States::PA),
            "PI" => Ok(States::PI),
            _ => Err("Invalid State".to_string())
        }
    }
}

pub enum LoginType {
    Birthday,
    MotherName,
    Cnpj,
    Cpf
}

impl ToString for LoginType{
    fn to_string(&self) -> String {
        match self{
            LoginType::Birthday => "birthday".to_string(),
            LoginType::MotherName => "motherName".to_string(),
            LoginType::Cnpj => "cnpj".to_string(),
            LoginType::Cpf => "cpf".to_string(),
        }
    }
}

impl TryFrom<&str> for LoginType{
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "birthday" => Ok(LoginType::Birthday),
            "mothername" => Ok(LoginType::MotherName),
            "cnpj" => Ok(LoginType::Cnpj),
            "cpf" => Ok(LoginType::Cpf),
            _ => Err("Invalid Login Type".to_string())
        }
    }
}

pub struct ApiSettings{
    pub state: String,
    pub namespace: String,
    pub ddd: u8,
    pub phone: String,
    pub site: String,
    pub api: String,
}

impl From<&States> for ApiSettings {
    fn from(value: &States) -> Self {
        match value {
            States::AL => ApiSettings{
                state: "Alagoas".to_string(),
                namespace: "ceal".to_string(),
                ddd: 82,
                phone: "0800 086 0800".to_string(),
                site: "https://al.equatorialenergia.com.br".to_string(),
                api: "https://api-al-cliente.equatorialenergia.com.br".to_string(),
            },
            States::MA => ApiSettings{
                state: "Maranhão".to_string(),
                namespace: "cemar".to_string(),
                ddd: 98,
                phone: "116".to_string(),
                site: "https://ma.equatorialenergia.com.br".to_string(),
                api: "https://api-ma-cliente.equatorialenergia.com.br".to_string(),
            },
            States::PA => ApiSettings{
                state: "Pará".to_string(),
                namespace: "celpa".to_string(),
                ddd: 95,
                phone: "0800 091 0196".to_string(),
                site: "https://pa.equatorialenergia.com.br".to_string(),
                api: "https://api-pa-cliente.equatorialenergia.com.br".to_string(),
            },
            States::PI => ApiSettings{
                state: "Piauí".to_string(),
                namespace: "cepisa".to_string(),
                ddd: 86,
                phone: "0800 086 0800".to_string(),
                site: "https://pi.equatorialenergia.com.br".to_string(),
                api: "https://api-pi-cliente.equatorialenergia.com.br".to_string(),
            },
        }
    }
}

pub struct WithCredentials{
    pub _type: LoginType,
    username: String,
    pub password: String,
}

impl WithCredentials {
    pub fn get_username(&self) -> String {
        match self._type {
            LoginType::Birthday => "1:".to_string() + self.username.clone().as_str(),
            LoginType::Cpf => "2:".to_string() + self.username.clone().as_str(),
            LoginType::MotherName => "3:".to_string() + self.username.clone().as_str(),
            LoginType::Cnpj => "4:".to_string() + self.username.clone().as_str(),
        }
    }
}

pub struct WithoutCredentials;

pub struct Client<State = WithoutCredentials>{
    pub state: States,
    pub settings: ApiSettings,
    credentials: State,
    token: Option<String>,
}

impl Client<WithoutCredentials> {
    pub fn new(state: States) -> std::result::Result<Client<WithoutCredentials>, String> {
        Ok(Client{
            settings: ApiSettings::from(&state),
            state,
            credentials: WithoutCredentials,
            token: None,
        })
    }

    pub fn set_credentials(self, username: String, password: String, login_type: LoginType) -> Client<WithCredentials> {
        Client{
            state: self.state,
            settings: self.settings,
            credentials: WithCredentials{
                _type: login_type,
                username,
                password,
            },
            token: None,
        }
    }
}

impl Client<WithCredentials> {
    pub fn new(state: States, credentials: WithCredentials) -> std::result::Result<Client<WithCredentials>, String> {
        match &credentials._type {
            LoginType::Birthday => {
                if credentials.username.len() != 8 {
                    return Err("Birthday must be 8 digits".to_string());
                }
                // Todo: check if birthday is valid
            },
            LoginType::Cpf => {
                if credentials.username.len() != 11 {
                    return Err("CPF must be 11 digits".to_string());
                }
                // Todo: check if cpf is valid
            },
            LoginType::MotherName => {
                if credentials.username.len() < 3 {
                    return Err("Mother Name must be at least 3 characters".to_string());
                }
            },
            LoginType::Cnpj => {
                if credentials.username.len() != 14 {
                    return Err("CNPJ must be 14 digits".to_string());
                }
                // Todo: check if cnpj is valid
            },
        }

        Ok(Client{
            settings: ApiSettings::from(&state),
            state,
            credentials,
            token: None,
        })
    }

    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    pub async fn login(&mut self) -> Result<()> {
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new()
            .text("username", self.credentials.get_username())
            .text("password", self.credentials.password.clone())
            .text("grant_type", "password");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, AUTHORIZATION_BASIC_TOKEN.parse().unwrap());
        headers.insert(reqwest::header::CONTENT_TYPE, "multipart/form-data; boundary".parse().unwrap());
        let url = format!("{}/auth/connect/token", self.settings.api);
        let response = client.post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        let response = serde_json::from_str::<LoginResponse>(&response.text().await?).unwrap();
        self.token = Some(response.access_token);

        Ok(())
    }
}
