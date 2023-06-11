use serde::Deserialize;

use crate::structures::consumption::Consumption;

#[derive(Debug, Deserialize)]
pub struct ConsumptionHistoryResponse {
    pub data: Vec<Consumption>,
}
