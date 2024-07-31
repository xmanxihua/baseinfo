use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserDetailResult {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<UserDetail>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserDetail {
    #[serde(rename = "userId", default)]
    pub user_id: Option<String>,

    #[serde(rename = "userId", default)]
    pub user_name: Option<String>,

    #[serde(rename = "userCode", default)]
    pub user_code: Option<String>,

    /**
     * 用户状态
     * @see com.micun.ucenter.api.consts.UserConstants.UserStatus
     */
    #[serde(default)]
    pub status: Option<i32>,

    /**
     * 用户类型
     * @see com.micun.ucenter.api.consts.UserConstants.UserIdentity
     */
    #[serde(default)]
    pub identity: Option<String>,
}
