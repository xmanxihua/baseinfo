use std::sync::Arc;

use axum::extract::{FromRequestParts, Query, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::sso::user_detail_dao::query_user_detail;
use crate::Params;
use crate::sso::auth;

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();
    let mut satoken: Option<String> = None;

    let header = parts.headers.get("satoken");
    if let Some(header) = header {
        if !header.is_empty() {
            let header = header.to_str().map_err(|e| StatusCode::UNAUTHORIZED)?;
            satoken = Some(header.into());
        }
    }

    if satoken.is_none() {
        let params = Query::<Params>::from_request_parts(&mut parts, &())
            .await
            .map_err(|e| StatusCode::UNAUTHORIZED)?;
        if params.satoken.is_empty() {
            return Err(StatusCode::UNAUTHORIZED);
        } else {
            satoken = Some(params.satoken.clone());
        }
    }

    if let Some(ref satoken) = satoken {
        // let satoken = splicing_key_token_value(satoken);
        let user_detail = query_user_detail(&satoken).await?;

        let mut req = Request::from_parts(parts, body);
        req.extensions_mut().insert(Arc::new(Some(user_detail)));
        let response = next.run(req).await;
        return Ok(response);
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}

// /**
//  * 拼接： 在保存 token - id 映射关系时，应该使用的key
//  *
//  * @param tokenValue token值
//  * @return key
//  */
// public String splicingKeyTokenValue(String tokenValue) {
// return getConfigOrGlobal().getTokenName() + ":" + loginType + ":token:" + tokenValue;
// }

fn splicing_key_token_value(satoken: &str) -> String {
    let mut string = String::new();
    string += "satoken:login:token:";
    string += satoken;
    string
}



