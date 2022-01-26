use serde::{Deserialize, Serialize};
use crate::api::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfig {
    pub auth_code: String
}

pub fn check(auth_code: String) -> bool {
    let str_val = include_str!("auth.toml"); // 请按照 `auth-example.toml` 的格式设置
    let config = toml::from_str::<AuthConfig>(&str_val).expect("Parsing failed, please check your profile structure");
    auth_code.as_str() == config.auth_code.as_str()
}

pub fn check_failed() -> Status {
    Status {
        code: 2,
        msg: "auth_code check failed".to_string()
    }
}