use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Cache error: {0}")]
    CacheError(String),
    #[error("Parse error: {0}")]
    Parse(#[from] core_lib::ParseError),
    #[error("query error: {0}")]
    Query(#[from] core_lib::QueryError),
}
