use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    pub error: Option<Error<T>>,
    pub data: Option<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error<T> {
    pub message: T,
    pub code: u16
}

impl <T: Serialize>Response<T> {
    pub fn error(message: T) -> Self {
        let error = Error {
            message,
            code: 1
        };

        Response {
            error: Some(error),
            data: None
        }
    }

    pub fn success(message: T) -> Self {
        Response {
            error: None,
            data: Some(message)
        }
    }
}