

use std::sync::{Arc,Mutex};

use axum::{Extension, Json, Router, extract::{Path, Query, State}, http::StatusCode, response::{Html, IntoResponse}, routing::get};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber;
#[derive(Clone)]
struct AppState{
    app_name: String,
    users: Arc<Mutex<Vec<String>>>
}
#[tokio::main]
async fn main() {

    let state = AppState{
        app_name:"Sample App".to_string(),
        users:Arc::new(Mutex::new(Vec::new()))
    };

    // tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());

    tracing_subscriber::fmt().init();
    let app = Router::new().
            route("/", get(handle_hello)).
            route("/users",get(list_users).post(add_user)).without_v07_checks().
            route("/user/:id", get(get_user)).
            with_state(state.clone()).layer(TraceLayer::new_for_http()).layer(Extension(state));
    let addr="127.0.0.1:3000";
    tracing::info!("server listening on {}",addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
async fn handle_hello(State(state):State<AppState>)->Html<String>{
   info!("hello");
   Html(format!("<p>hello world{}</p>",state.app_name))
}

#[derive(Serialize)]
struct UserList{
    users:Vec<String>
}

#[derive(Deserialize)]
struct NewUser{
    name:String
}

#[derive(Serialize)]
struct User{
   name:String
}
async fn get_user(State(state):State<AppState>,Path(id):Path<usize>)->impl IntoResponse{
    let users= state.users.lock().unwrap();
    let user =users.get(id);
    info!("logging id {} {:?} ",id,user);
    if let Some(value) = user{
        (StatusCode::OK,format!("user id {} --> user name {}",id,value))
    }else{
         (StatusCode::NOT_FOUND,format!("No user Found"))
    }
}
#[derive(Deserialize)]
struct Pagination{
    limit :Option<usize>,
}
async fn list_users(State(state):State<AppState>,Query(p):Query<Pagination>)->Json<UserList>{
     let limit = p.limit.unwrap_or(state.users.lock().unwrap().len());
     info!("listing users");
     let users = state.users.lock().unwrap();
     let new_users:Vec<String> = users.iter().take(limit).cloned().collect();
     Json(UserList { users:new_users })
}

async fn add_user(State(state):State<AppState>,Json(new_user):Json<NewUser>)->impl IntoResponse{
    info!("Adding Users");
    let mut users = state.users.lock().unwrap();
    users.push(new_user.name.clone());
    (
        StatusCode::CREATED,
        format!("new user data name is  {}",new_user.name)
    )
}



