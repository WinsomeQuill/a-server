use std::any::Any;
use std::time::Duration;
use actix_web::dev::Extensions;
use rand::Rng;
use crate::models::dto::calculator_dto::{CalculatorDto, Operation};

pub async fn generate_delay() {
    let mut rng = rand::thread_rng();
    let time_delay = rng.gen_range(4000..5000);

    tokio::time::sleep(Duration::from_millis(time_delay)).await;
}

pub async fn calculating(calculator_dto: CalculatorDto) -> Result<f64, String> {
    if calculator_dto.last_number == 0.0 {
        return Err("Сan not divide by zero!".into());
    }

    let result = match calculator_dto.operation {
        Operation::Div => calculator_dto.first_number / calculator_dto.last_number,
        Operation::Mul => calculator_dto.first_number * calculator_dto.last_number,
        Operation::Add => calculator_dto.first_number + calculator_dto.last_number,
        Operation::Sub => calculator_dto.first_number - calculator_dto.last_number,
    };

    Ok(result)
}

pub fn connection(connection: &dyn Any, data: &mut Extensions) {
    dbg!(connection);
    dbg!(data);
}