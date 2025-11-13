use std::result;
use std::sync::Arc;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::http::header;
use serde_json::from_slice;
use tokio::sync::Mutex;
use tower::ServiceExt;
use crate::AppState;
use crate::Thing;
use crate::create_app;
use http_body_util::BodyExt; 
use http_body_util::Full;
// Helper function to extract the body as a string.
async fn body_to_string(body: Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_auth_success() {
    let state = AppState { count: Arc::new(Mutex::new(0)) };
    let app = create_app(state);
    
    let request = Request::builder().uri("/").header(header::AUTHORIZATION, "Bearer Secret-token").body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();

    let expected_result = Thing{
        name:"Manohar".to_string()
    };
    let bytes= response.into_body().collect().await.unwrap().to_bytes();
    let result:Thing= serde_json::from_slice(&bytes).unwrap();
    assert_eq!(result,expected_result);
}

#[tokio::test]
async fn test_auth_failure() {
    let state = AppState { count: Arc::new(Mutex::new(0)) };
    let app = create_app(state);
    
    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_rate_limit_exceeded() {
    let state = AppState { count: Arc::new(Mutex::new(0)) };
    let app = create_app(state.clone());
    
    // Simulate 5 requests that should pass
    for _ in 0..5 {
        let request = Request::builder()
            .uri("/")
            .header(header::AUTHORIZATION, "Bearer Secret-token")
            .body(Body::empty())
            .unwrap();
        app.clone().oneshot(request).await.unwrap();
    }
    
    // The 6th request should fail due to rate-limiting
    let request = Request::builder()
        .uri("/")
        .header(header::AUTHORIZATION, "Bearer Secret-token")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn test_ping_handler_returns_404() {
    let state = AppState { count: Arc::new(Mutex::new(0)) };
    let app = create_app(state);
    
    let request = Request::builder()
        .uri("/")
        .header(header::AUTHORIZATION, "Bearer Secret-token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
