use crate::api::params::search_time_line::{
    Product, SearchTimeLineResponse, SearchTimeLineResponseType, SearchTimelineParams,
};
use crate::utils::headers::extract_headers;
use reqwest::redirect::Policy;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UserAgent(pub String);

impl UserAgent {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for UserAgent {
    fn default() -> Self {
        Self::new(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36",
        )
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub reqwest_client: reqwest::Client,
    pub csrf: String,
    pub bearer_token: String,
    pub cookie: String,
    pub user_agent: UserAgent,
    pub transaction_id: String,
    pub search_timeline_endpoint: String,
}

impl ApiClient {
    pub fn new(
        search_timeline_endpoint: &str,
        csrf: &str,
        bearer_token: &str,
        cookie: &str,
        transaction_id: &str,
    ) -> anyhow::Result<Self> {
        let user_agent = UserAgent::default();
        let reqwest_client = reqwest::Client::builder()
            .user_agent(user_agent.0.clone())
            .redirect(Policy::limited(10))
            .build()?;
        Ok(Self {
            transaction_id: transaction_id.to_string(),
            cookie: cookie.to_string(),
            user_agent,
            reqwest_client,
            csrf: csrf.to_string(),
            bearer_token: bearer_token.to_string(),
            search_timeline_endpoint: search_timeline_endpoint.to_string(),
        })
    }

    pub fn update_search_timeline_endpoint(&mut self, search_timeline_endpoint: &str) {
        self.search_timeline_endpoint = search_timeline_endpoint.to_string();
    }

    pub async fn search_timeline(
        &self,
        count: u32,
        raw_query: &str,
        cursor: Option<String>,
        search_mode: &Product,
    ) -> anyhow::Result<SearchTimeLineResponse> {
        let mut search_time_line_params = SearchTimelineParams::default();
        search_time_line_params.update_count(count);
        search_time_line_params.update_raw_query(raw_query);
        search_time_line_params.update_cursor(cursor.clone());
        search_time_line_params.update_product(search_mode.clone());
        let params = search_time_line_params.params()?;
        let url = format!(
            "https://x.com/i/api/graphql/{}/SearchTimeline",
            self.search_timeline_endpoint
        );
        tracing::info!("url = {url}");
        let request = self
            .reqwest_client
            .get(url)
            .query(&params)
            .bearer_auth(&self.bearer_token)
            .header("x-csrf-token", &self.csrf)
            .header("x-twitter-active-user", "yes")
            .header("x-twitter-auth-type", "OAuth2Session")
            .header("x-twitter-client-language", "en")
            .header("x-client-transaction-id", &self.transaction_id)
            .header("Cookie", &self.cookie)
            .header("accept", "*/*")
            .build()?;
        let response = self.reqwest_client.execute(request).await?;
        let headers = response.headers().clone();
        let rate_limit_headers = extract_headers(headers);
        let code = response.status();
        tracing::info!("code = {}", code);
        let response = response.text().await?;
        let response = response.trim();
        if response.contains("Rate limit exceeded") {
            Ok(SearchTimeLineResponse {
                response_type: SearchTimeLineResponseType::RateLimit,
                response_content: None,
                response_headers: rate_limit_headers,
            })
        } else if response.contains("Could not authenticate you") {
            Ok(SearchTimeLineResponse {
                response_type: SearchTimeLineResponseType::AuthError,
                response_content: None,
                response_headers: rate_limit_headers,
            })
        } else {
            let value: Value = serde_json::from_str(response)?;
            Ok(SearchTimeLineResponse {
                response_type: SearchTimeLineResponseType::Success,
                response_content: Some(value),
                response_headers: rate_limit_headers,
            })
        }
    }
}
