use chrono::NaiveDate;
use serde::Deserialize;

use crate::deserializers::{
    deserialize_competencia, deserialize_date, deserialize_date_opt, deserialize_f64,
    deserialize_string_opt,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub numero_fatura: String,
    #[serde(deserialize_with = "deserialize_f64")]
    pub valor: f64,
    #[serde(deserialize_with = "deserialize_date")]
    pub data_vencimento: NaiveDate,
    #[serde(deserialize_with = "deserialize_date_opt")]
    pub data_pagamento: Option<NaiveDate>,
    #[serde(deserialize_with = "deserialize_competencia")]
    pub competencia: Competencia,
    #[serde(deserialize_with = "deserialize_string_opt")]
    pub codigo_barras: Option<String>,
    #[serde(deserialize_with = "deserialize_string_opt")]
    pub nota_fiscal: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Competencia {
    pub mes: u32,
    pub ano: u32,
}

impl ToString for Competencia {
    fn to_string(&self) -> String {
        format!("{}/{}", self.ano, self.mes)
    }
}
