use axum::{
    Error,
    body::Body,
    extract::State,
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::IntoResponse,
};

use crate::{error::AppError, state::AppState};

pub async fn auth_middle_ware(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    println!("---->auth");
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|x| x.to_str().ok())
        .unwrap_or_default();
    let authorized = token == "Bearer Secret-token";

    if !authorized {
        return Err(AppError::AuthError("notmatched".to_string()));
    }
    Ok(next.run(req).await)
}

pub async fn auth_rate_limitng(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    println!("ratelimit");
    let mut count = state.counter.lock().await;
    *count += 1;
    let value = *count;
    if value > 5 {
        return Err(AppError::RateLimitError);
    }
    Ok((next.run(req).await))
}

// pub async fn auth_rate_limitng(
//     State(_state): State<AppState>,
//     req: Request<axum::body::Body>,
//     next: Next,
// ) -> Result<impl IntoResponse, AppError> {
//     // Your rate-limiting logic here
//     Ok(next.run(req).await)
// }
