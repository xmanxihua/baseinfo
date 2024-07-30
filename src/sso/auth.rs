use axum::extract::{FromRequestParts, Query, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use crate::Params;
use crate::sso::user_detail_dao::query_user_detail;

pub async fn auth_middleware(req: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    let (mut parts, body) = req.into_parts();
    if let Some(satoken) = parts.headers.get("satoken") {
        if satoken.is_empty() {
            let params = Query::<Params>::from_request_parts(&mut parts, &())
                .await
                .map_err(|e| StatusCode::UNAUTHORIZED)?;
            if params.satoken.is_empty() {
                return Err(StatusCode::UNAUTHORIZED);
            }
        } else {}
    }

    let params = Query::<Params>::from_request_parts(&mut parts, &())
        .await
        .map_err(|e| StatusCode::UNAUTHORIZED)?;
    if params.satoken.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user_detail = query_user_detail(&params.satoken).await?;

    let req = Request::from_parts(parts, body);
    Ok(next.run(req))
}
