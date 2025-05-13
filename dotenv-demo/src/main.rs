use std::{str::FromStr, time::Duration};

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct Config {
    host: String,
    port: u16,
    #[serde(flatten)]
    port_config: PortConfig,
    debug: bool,
    #[serde(with = "humantime_serde")]
    timeout: Duration,
}

#[derive(Debug, Deserialize)]
struct PortConfig {
    #[serde(
        default = "default_port",
        deserialize_with = "deserialize_string_to_number"
    )]
    port2: u16,
}

fn default_port() -> u16 {
    8080
}

pub fn deserialize_string_to_number<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            let s = s.trim();
            s.parse::<T>().map_err(|e| {
                serde::de::Error::custom(format!("{}, {} can not parse to number", e, s))
            })
        }
        StringOrInt::Number(i) => Ok(i),
    }
}

fn main() {
    // 自动加载 .env 文件
    dotenvy::dotenv().ok();

    // 从环境变量中解析为 Config 结构体
    let config: Config = envy::prefixed("DEMO_")
        .from_env::<Config>()
        .expect("Failed to load config from environment");

    println!("{:#?}", config);
}
