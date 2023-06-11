use serde::Deserialize;

use crate::structures::invoice::Invoice;

#[derive(Debug, Deserialize)]
pub struct InvoiceListResponse {
    pub data: InvoiceData,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceData {
    pub conta_contrato: String,
    pub canal_atendimento: String,
    pub protocolo: Option<String>,
    pub total_debitos: f64,
    pub codigo: String,
    pub mensagem: String,
    pub faturas: Vec<Invoice>,
}
