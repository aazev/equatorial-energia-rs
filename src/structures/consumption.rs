use serde::Deserialize;

use super::invoice::Competencia;
use crate::deserializers::{deserialize_competencia, deserialize_f64};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumption {
    #[serde(deserialize_with = "deserialize_competencia")]
    pub competencia: Competencia,
    #[serde(deserialize_with = "deserialize_f64")]
    pub valor_consumo: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub energy_percentual: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub distribuicao_valor: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub transmissao_valor: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub transmissao_percentual: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub encargo_valor: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub encargo_percentual: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub perdas_valor: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub perdas_percentual: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub tributos_valor: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub tributos_percentual: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub outros_valor: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub outros_percentual: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub energia_valor: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub kwh: f64,
}
