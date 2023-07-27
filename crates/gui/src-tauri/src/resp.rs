use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Resp<T> {
    pub code: i64,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> From<Result<T>> for Resp<T>
where
    T: Serialize,
{
    fn from(item: Result<T>) -> Self {
        match item {
            Ok(data) => Resp::success(data),
            Err(e) => Resp::fail(1, &e.to_string()),
        }
    }
}

impl<T> Resp<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Resp {
            code: 0,
            msg: Some("success".to_string()),
            data: Some(data),
        }
    }

    pub fn fail(code: i64, e: &str) -> Self {
        Resp {
            code,
            msg: Some(e.to_owned()),
            data: None,
        }
    }
}
