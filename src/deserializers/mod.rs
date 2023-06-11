use chrono::NaiveDate;
use serde::Deserialize;

use crate::structures::invoice::Competencia;

pub fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let valor = s.trim().parse::<f64>().map_err(serde::de::Error::custom)?;
    Ok(valor)
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
    Ok(date)
}

pub fn deserialize_date_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let date =
                NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
            Ok(Some(date))
        }
        None => Ok(None),
    }
}

pub fn deserialize_competencia<'de, D>(deserializer: D) -> Result<Competencia, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let mut split = s.split("/");
    let ano = split
        .next()
        .ok_or_else(|| serde::de::Error::custom("Invalid competencia"))?
        .parse::<u32>()
        .map_err(serde::de::Error::custom)?;
    let mes = split
        .next()
        .ok_or_else(|| serde::de::Error::custom("Invalid competencia"))?
        .parse::<u32>()
        .map_err(serde::de::Error::custom)?;
    Ok(Competencia { mes, ano })
}

pub fn deserialize_string_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(s)),
        None => Ok(None),
    }
}
