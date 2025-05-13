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
    #[serde(default = "default_port", deserialize_with = "deserialize_from_string")]
    port2: u16,
    #[serde(default, deserialize_with = "deserialize_from_string")]
    enabled: bool,
    #[serde(with = "humantime_serde")]
    timeout2: Duration,
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

/// 通用的反序列化函数：支持从 `String` 或直接类型解析
pub fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    // 定义枚举：允许直接解析为 T，或先解析为 String 再转换
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOr<T> {
        String(String),
        Direct(T),
    }

    match StringOr::<T>::deserialize(deserializer)? {
        StringOr::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOr::Direct(x) => Ok(x),
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
