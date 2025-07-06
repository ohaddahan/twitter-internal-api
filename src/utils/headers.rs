use reqwest::header::{HeaderMap, HeaderName};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RateLimitHeaders {
    pub rate_limit_limit: Option<i64>,
    pub rate_limit_remaining: Option<i64>,
    pub rate_limit_rest: Option<i64>,
}

#[tracing::instrument(name = "extract_headers", skip_all)]
pub fn extract_headers(headers: HeaderMap) -> RateLimitHeaders {
    let mut output = RateLimitHeaders {
        rate_limit_limit: None,
        rate_limit_remaining: None,
        rate_limit_rest: None,
    };
    for (key, value) in headers.into_iter() {
        let key = key
            .unwrap_or(HeaderName::from_static("tmp"))
            .as_str()
            .to_lowercase();
        let value = value.to_str().unwrap_or_default();
        match key.as_str() {
            "x-rate-limit-limit" => {
                let v = value.parse::<i64>().unwrap_or(1);
                output.rate_limit_limit = Some(v);
            }
            "x-rate-limit-remaining" => {
                let v = value.parse::<i64>().unwrap_or(1);
                output.rate_limit_remaining = Some(v);
            }
            "x-rate-limit-reset" => {
                let v = value.parse::<i64>().unwrap_or(1);
                output.rate_limit_rest = Some(v);
            }
            _ => {}
        }
    }
    output
}
