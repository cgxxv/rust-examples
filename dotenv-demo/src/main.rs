use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
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
