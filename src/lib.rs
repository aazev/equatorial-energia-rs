pub mod deserializers;
pub mod responses;
pub mod structures;

use base64::{
    alphabet,
    engine::{general_purpose, GeneralPurpose},
    Engine,
};
use chrono::NaiveDateTime;
use responses::login::LoginResponse;
use serde::{Deserialize, Serialize};
use std::error::Error;
use structures::{consumption::Consumption, invoice::Invoice};

pub const AUTHORIZATION_BASIC_TOKEN: &str = "Basic RXF1YXRvcmlhbFNpdGU6R216MnNpdGU=";

#[derive(Clone, Debug)]
pub enum States {
    AL,
    MA,
    PA,
    PI,
}

impl ToString for States {
    fn to_string(&self) -> String {
        match self {
            States::AL => "AL".to_string(),
            States::MA => "MA".to_string(),
            States::PA => "PA".to_string(),
            States::PI => "PI".to_string(),
        }
    }
}

impl TryFrom<&str> for States {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "AL" => Ok(States::AL),
            "MA" => Ok(States::MA),
            "PA" => Ok(States::PA),
            "PI" => Ok(States::PI),
            _ => Err("Invalid State".to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LoginType {
    Birthday,
    MotherName,
    Cnpj,
    Cpf,
}

impl ToString for LoginType {
    fn to_string(&self) -> String {
        match self {
            LoginType::Birthday => "birthday".to_string(),
            LoginType::MotherName => "motherName".to_string(),
            LoginType::Cnpj => "cnpj".to_string(),
            LoginType::Cpf => "cpf".to_string(),
        }
    }
}

impl TryFrom<&str> for LoginType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "birthday" => Ok(LoginType::Birthday),
            "mothername" => Ok(LoginType::MotherName),
            "cnpj" => Ok(LoginType::Cnpj),
            "cpf" => Ok(LoginType::Cpf),
            _ => Err("Invalid Login Type".to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ApiSettings {
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
            States::AL => ApiSettings {
                state: "Alagoas".to_string(),
                namespace: "ceal".to_string(),
                ddd: 82,
                phone: "0800 086 0800".to_string(),
                site: "https://al.equatorialenergia.com.br".to_string(),
                api: "https://api-al-cliente.equatorialenergia.com.br".to_string(),
            },
            States::MA => ApiSettings {
                state: "Maranhão".to_string(),
                namespace: "cemar".to_string(),
                ddd: 98,
                phone: "116".to_string(),
                site: "https://ma.equatorialenergia.com.br".to_string(),
                api: "https://api-ma-cliente.equatorialenergia.com.br".to_string(),
            },
            States::PA => ApiSettings {
                state: "Pará".to_string(),
                namespace: "celpa".to_string(),
                ddd: 95,
                phone: "0800 091 0196".to_string(),
                site: "https://pa.equatorialenergia.com.br".to_string(),
                api: "https://api-pa-cliente.equatorialenergia.com.br".to_string(),
            },
            States::PI => ApiSettings {
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

#[derive(Clone, Debug)]
pub struct Credentials {
    pub _type: LoginType,
    username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(_type: LoginType, username: String, password: String) -> Self {
        Credentials {
            _type,
            username,
            password,
        }
    }

    pub fn get_username(&self) -> String {
        match self._type {
            LoginType::Birthday => "1:".to_string() + self.username.clone().as_str(),
            LoginType::Cpf => "2:".to_string() + self.username.clone().as_str(),
            LoginType::MotherName => "3:".to_string() + self.username.clone().as_str(),
            LoginType::Cnpj => "4:".to_string() + self.username.clone().as_str(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Authenticated {
    pub token: String,
    pub expires: NaiveDateTime,
    pub cpf: String,
    pub email: String,
    pub credenciado: bool,
    pub nome: String,
    pub sobrenome: String,
    pub quantidade_contas_contrato: u8,
    pub contas_contrato: Vec<ContaContrato>,
}

pub struct Unauthenticated;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContaContrato {
    #[serde(rename = "Numero")]
    pub numero: String,
    #[serde(rename = "Endereco")]
    pub endereco: String,
    #[serde(rename = "Bairro")]
    pub bairro: String,
    #[serde(rename = "Cidade")]
    pub cidade: String,
    #[serde(rename = "NumeroInstalacao")]
    pub numero_instalacao: String,
}

#[derive(Debug)]
pub struct Client<State = Unauthenticated> {
    pub state: States,
    pub settings: ApiSettings,
    credentials: Credentials,
    pub authorization: State,
}

impl Client<Unauthenticated> {
    pub fn new(state: States, credentials: Credentials) -> Result<Client<Unauthenticated>, String> {
        match &credentials._type {
            LoginType::Birthday => {
                if credentials.password.len() != 10 {
                    return Err("Birthday must be 10 digits".to_string());
                }
                // Todo: check if birthday is valid
            }
            LoginType::Cpf => {
                if credentials.password.len() != 11 {
                    return Err("CPF must be 11 digits".to_string());
                }
                // Todo: check if cpf is valid
            }
            LoginType::MotherName => {
                if credentials.password.len() < 3 {
                    return Err("Mother Name must be at least 3 characters".to_string());
                }
            }
            LoginType::Cnpj => {
                if credentials.password.len() != 14 {
                    return Err("CNPJ must be 14 digits".to_string());
                }
                // Todo: check if cnpj is valid
            }
        }

        Ok(Client {
            settings: ApiSettings::from(&state),
            state,
            credentials,
            authorization: Unauthenticated,
        })
    }

    pub async fn login(&self) -> Result<Client<Authenticated>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new()
            .text("username", self.credentials.get_username())
            .text("password", self.credentials.password.clone())
            .text("grant_type", "password");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            "application/json, text/plain, */*".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            AUTHORIZATION_BASIC_TOKEN.parse().unwrap(),
        );
        // headers.insert(
        //     reqwest::header::CONTENT_TYPE,
        //     format!("multipart/form-data; boundary={}", form.boundary())
        //         .parse()
        //         .unwrap(),
        // );
        let url = format!("{}/auth/connect/token", self.settings.api);
        let response = client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;
        let body = response.text().await?;

        // println!("{:#?}", &body);

        let response = serde_json::from_str::<LoginResponse>(&body).unwrap();

        let payload = Client::decode_token(&response.access_token)?;

        // println!("{:?}", &payload);

        Ok(Client {
            settings: self.settings.clone(),
            state: self.state.clone(),
            credentials: self.credentials.clone(),
            authorization: payload,
        })
    }

    fn decode_token(token: &str) -> Result<Authenticated, Box<dyn Error>> {
        let parts = token.clone().split(".").collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err("Invalid token".into());
        }
        let payload = parts
            .into_iter()
            .filter_map(|part| {
                if !part.starts_with("eyJ") {
                    return None;
                }
                let part = part.trim();
                //let bytes = general_purpose::STANDARD.decode(part).unwrap();
                let bytes = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
                    .decode(part)
                    .unwrap();
                match serde_json::from_slice::<serde_json::Value>(&bytes) {
                    Ok(value) => Some(value),
                    Err(_) => None,
                }
            })
            .collect::<Vec<serde_json::Value>>();

        // println!("{:#?}", &payload);

        let expires = NaiveDateTime::from_timestamp_opt(
            payload[1]["exp"].as_i64().unwrap(),
            ((payload[1]["exp"].as_u64().unwrap() * 1000 as u64) % 1000 as u64 * 1_000_000 as u64)
                as u32,
        )
        .unwrap();

        Ok(Authenticated {
            token: token.to_string(),
            expires,
            cpf: payload[1]["userData"]["Cpf"].as_str().unwrap().to_string(),
            contas_contrato: serde_json::from_value(
                payload[1]["userData"]["ContasContrato"].clone(),
            )
            .unwrap(),
            credenciado: payload[1]["userData"]["Credenciado"].as_bool().unwrap(),
            email: payload[1]["email"].as_str().unwrap().to_string(),
            nome: payload[1]["userData"]["Nome"].as_str().unwrap().to_string(),
            sobrenome: payload[1]["userData"]["Sobrenome"]
                .as_str()
                .unwrap()
                .to_string(),
            quantidade_contas_contrato: payload[1]["userData"]["QuantidadeContasContrato"]
                .as_u64()
                .unwrap() as u8,
        })
    }
}

impl Client<Authenticated> {
    #[allow(dead_code)]
    fn get_token(&self) -> String {
        self.authorization.token.clone()
    }

    pub fn list_invoice(&self) -> Vec<Invoice> {
        let _client = reqwest::Client::new();

        vec![]
    }

    pub fn consumption_history(&self) -> Vec<Consumption> {
        vec![]
    }
}
