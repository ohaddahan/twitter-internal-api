pub mod api;
pub mod tracing;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::api::client::ApiClient;
    use crate::api::params::search_time_line::Product;
    use crate::tracing::init::init_tracing;
    use crate::utils::env::{get_envar, load_dotenv};

    #[tokio::test]
    async fn get_profile() {
        load_dotenv();
        init_tracing();
        let search_timeline_endpoint = get_envar::<String>("GRAPHQL_ENDPOINT").unwrap();
        let csrf = get_envar::<String>("X_CSRF_TOKEN").unwrap();
        let bearer = get_envar::<String>("BEARER_TOKEN").unwrap();
        let cookie = get_envar::<String>("COOKIE").unwrap();
        let transaction_id = get_envar::<String>("X_CLIENT_TRANSACTION_ID").unwrap();
        let api_client = ApiClient::new(
            &search_timeline_endpoint,
            &csrf,
            &bearer,
            &cookie,
            &transaction_id,
        )
        .unwrap();
        let profile = "___zask___";
        let response = api_client
            .search_timeline(20, profile, None, &Product::People)
            .await;
        tracing::info!("get_profile response => {:#?}", response);
    }
}
