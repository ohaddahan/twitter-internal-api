use anyhow::anyhow;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::{env, fs};

#[tracing::instrument(name = "get_envar", skip_all, err)]
pub fn get_envar<T: FromStr + Debug>(name: &str) -> anyhow::Result<T>
where
    <T as FromStr>::Err: Display,
{
    let var = env::var(name).map_err(|e| {
        tracing::error!("Didn't find envar name: {}", name);
        anyhow!("get_envvar error: {}", e)
    })?;

    let value = var.parse::<T>().map_err(|e| {
        tracing::error!(
            "Couldn't parse variable '{}' with value: '{:?}' and error: '{}'",
            name,
            var,
            e
        );
        anyhow!("get_envvar error: {}", e)
    })?;
    Ok(value)
}

#[tracing::instrument(name = "load_dotenv", skip_all)]
pub fn load_dotenv() {
    let dotenv_file = format!(
        ".env.{}",
        env::var("APP_ENVIRONMENT").unwrap_or("local".into())
    );
    if fs::exists(&dotenv_file).unwrap_or_default() {
        tracing::info!("Loading {}", dotenv_file);
        dotenv::from_filename(&dotenv_file).ok();
        return;
    }
    if fs::exists(".env").unwrap_or_default() {
        tracing::info!("Loading .env");
        dotenv::from_filename(".env").ok();
    }
}
