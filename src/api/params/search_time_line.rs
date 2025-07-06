use crate::utils::headers::RateLimitHeaders;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchTimelineParams {
    pub variables: Variables,
    pub features: Features,
    pub field_toggles: FieldToggles,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Features(Value);

impl Default for Features {
    fn default() -> Self {
        Self(serde_json::json!({
        "articles_preview_enabled": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "communities_web_enable_tweet_community_results_fetch": true,
        "creator_subscriptions_quote_tweet_preview_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "longform_notetweets_inline_media_enabled": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "payments_enabled": false,
        "premium_content_api_read_enabled": false,
        "profile_label_improvements_pcf_label_in_post_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "responsive_web_enhance_cards_enabled": false,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_grok_analysis_button_from_backend": true,
        "responsive_web_grok_analyze_button_fetch_trends_enabled": false,
        "responsive_web_grok_analyze_post_followups_enabled": true,
        "responsive_web_grok_image_annotation_enabled": true,
        "responsive_web_grok_share_attachment_enabled": true,
        "responsive_web_grok_show_grok_translated_post": false,
        "responsive_web_jetfuel_frame": false,
        "responsive_web_media_download_video_enabled": false,
        "responsive_web_twitter_article_tweet_consumption_enabled": false,
        "responsive_web_twitter_article_tweet_consumption_enabled": true,
        "rweb_lists_timeline_redesign_enabled": true,
        "rweb_tipjar_consumption_enabled": true,
        "rweb_video_screen_enabled": false,
        "standardized_nudges_misinfo": true,
        "tweet_awards_web_tipping_enabled": false,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "tweetypie_unmention_optimization_enabled": true,
        "verified_phone_label_enabled": false,
        "view_counts_everywhere_api_enabled": true,
        }))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct FieldToggles {
    pub with_article_rich_content_state: bool,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub raw_query: String,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub query_source: QuerySource,
    pub product: Product,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub enum QuerySource {
    #[serde(rename = "typed_query")]
    #[default]
    TypedQuery,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, strum::EnumString, strum::Display)]
pub enum Product {
    #[default]
    #[serde(rename = "Top")]
    Top,
    #[serde(rename = "Latest")]
    Latest,
    #[serde(rename = "Photos")]
    Photos,
    #[serde(rename = "People")]
    People,
    #[serde(rename = "Videos")]
    Videos,
}

impl SearchTimelineParams {
    pub fn params(&self) -> anyhow::Result<Vec<(&str, String)>> {
        let params = &[
            ("variables", serde_json::to_string(&self.variables)?),
            ("features", serde_json::to_string(&self.features)?),
            ("fieldToggles", serde_json::to_string(&self.field_toggles)?),
        ];
        Ok(params.to_vec())
    }

    pub fn update_cursor(&mut self, cursor: Option<String>) {
        if let Some(cursor) = cursor {
            self.variables.cursor = Some(cursor);
        }
    }

    pub fn update_raw_query(&mut self, raw_query: &str) {
        self.variables.raw_query = raw_query.to_string();
    }

    pub fn update_count(&mut self, count: u32) {
        if count > 50 {
            self.variables.count = 50
        } else {
            self.variables.count = count;
        }
    }

    pub fn update_product(&mut self, product: Product) {
        self.variables.product = product;
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SearchTimeLineResponseType {
    Success,
    AuthError,
    RateLimit,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchTimeLineResponse {
    pub response_type: SearchTimeLineResponseType,
    pub response_content: Option<Value>,
    pub response_headers: RateLimitHeaders,
}
