use serde::{Deserialize, Serialize};
use crate::database::DataBaseConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddVO {
    pub database: DataBaseConfig,
    pub auth_code: String,
    pub add: isize
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CountVO {
    pub database: DataBaseConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountDTO {
    pub status: Status,
    pub total: isize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub code: usize,
    pub msg: String,
}
impl Status {
    pub fn success() -> Self {
        Self {
            code: 0,
            msg: "success".to_string()
        }
    }
    pub fn fail(msg: String) -> Self {
        Self {
            code: 1,
            msg
        }
    }
}
