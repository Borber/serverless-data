use crate::api::Status;

pub fn check(auth_code: String) -> bool {
    let env_auth_code = env!("AUTH_CODE");
    auth_code.as_str() == env_auth_code
}

pub fn check_failed() -> Status {
    Status {
        code: 2,
        msg: "auth_code check failed".to_string()
    }
}