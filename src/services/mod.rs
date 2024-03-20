use std::str::FromStr;
use actix_web::HttpRequest;
use actix_web::web::{BytesMut, Payload};
use qstring::QString;
use serde::de;
use futures_util::StreamExt;

const MAX_SIZE_BUFFER_REQUEST: usize = 16_777_216; // максимальный размер буфера - 256кб

pub async fn read_body_bytes(payload: &mut Payload) -> Result<BytesMut, ()> {
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        if (body.len() + chunk.len()) > MAX_SIZE_BUFFER_REQUEST {
            return Err(());
        }
        body.extend_from_slice(&chunk);
    }

    Ok(body)
}

pub async fn convert_body_to_struct<'a, T>(bytes_mut: &'a BytesMut) -> Result<T, serde_json::error::Error>
    where T: de::Deserialize<'a> {
    let item = match serde_json::from_slice::<T>(bytes_mut) {
        Ok(o) => o,
        Err(e) => {
            println!("{}", e);
            return Err(e);
        },
    };

    Ok(item)
}

pub async fn get_query_param<T: FromStr>(req: &HttpRequest, query_key: &str) -> Result<T, String> {
    let query_str = req.query_string();
    let qs = QString::from(query_str);

    let result = match qs.get(query_key) {
        Some(o) => match o.parse::<T>() {
            Ok(o) => o,
            Err(_) => return Err(format!("Invalid query type for {}!", &query_key)),
        },
        None => return Err(format!("Not found {}!", &query_key)),
    };

    Ok(result)
}

pub mod not_found;
pub mod work_service;