use crate::api::params::search_time_line::Product;
use chrono::NaiveDate;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct TweetOptions {
    #[clap(long)]
    pub profile: String,
    #[clap(long)]
    pub since: Option<NaiveDate>,
    #[clap(long)]
    pub until: Option<NaiveDate>,
    #[clap(long, default_value_t = Product::Top)]
    pub search_mode: Product,
    #[clap(long, default_value_t = 50)]
    pub count: u32,
    #[clap(long)]
    pub cursor: Option<String>,
    #[clap(long)]
    pub output: String,
    #[clap(long, default_value_t = Direction::Desc)]
    pub direction: Direction,
}

#[derive(Parser, Debug, Clone)]
pub struct ProfilesOptions {
    #[clap(long, default_value_t = 50)]
    pub count: u32,
    #[clap(long)]
    pub output: String,
    #[clap(long)]
    pub profile: String,
}

#[derive(Parser, Debug, Clone, Default, strum::EnumString, strum::Display)]
pub enum Direction {
    #[default]
    Desc,
    Asc,
}

#[derive(Parser)]
pub enum Options {
    Tweets(TweetOptions),
    Profiles(ProfilesOptions),
}

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Output {
    PrettyPrint,
    Json,
}

pub fn from_args() -> Options {
    Options::parse()
}
