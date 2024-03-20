use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CalculatorDto {
    pub first_number: f64,
    pub last_number: f64,
    pub operation: Operation
}

#[derive(Debug, Deserialize)]
pub enum Operation {
    #[serde(rename = "div")]
    Div,
    #[serde(rename = "mul")]
    Mul,
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "sub")]
    Sub,
}