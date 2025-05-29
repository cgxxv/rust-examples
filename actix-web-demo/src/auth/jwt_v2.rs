use actix_web::{HttpMessage, dev::ServiceRequest};
use actix_web_httpauth::extractors::{
    AuthenticationError,
    bearer::{BearerAuth, Config},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

// JWT 的 Claims（存储用户信息）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户 ID
    pub exp: usize,  // 过期时间（UNIX 时间戳）
}

// 生成 JWT
pub fn generate_jwt(user_id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your-secret-key".as_ref()),
    )
    .unwrap()
}

// 验证 JWT
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // decode::<Claims>(
    //     token,
    //     &DecodingKey::from_secret("your-secret-key".as_ref()),
    //     &Validation::default(),
    // )
    // .map(|data| data.claims)

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret("your-secret-key".as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => {
            println!("got claims: {:#?}", data.claims);
            Ok(data.claims)
        }
        Err(err) => {
            println!("validate_jwt failed: {err}");
            Err(err)
        }
    }
}

#[allow(unused)]
pub async fn validator(
    req: ServiceRequest,
    auth: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // 1. 使用标准日志输出（替代println!）
    log::debug!("Validating token for request: {:?}", req.path());

    // 2. 从请求中获取Config（保持中间件配置一致性）
    let config = req.app_data::<Config>().cloned().unwrap_or_default();

    // 3. 验证JWT并处理错误
    match validate_jwt(auth.token()) {
        Ok(claims) => {
            // 4. 将claims存入请求扩展，供后续路由使用
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(err) => {
            log::error!("validate_jwt failed: {err}");
            // 5. 正确构造错误返回（包含Config和原始请求）
            Err((
                actix_web::Error::from(AuthenticationError::from(config)),
                req, // 这里可以安全使用 `req`，因为 `match` 不会提前移动所有权
            ))
        }
    }
}
