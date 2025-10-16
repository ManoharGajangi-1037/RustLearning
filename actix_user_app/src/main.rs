use std::sync::{Arc, Mutex};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix")
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct User {
    name: String,
    age: u32,
}

type AppState = Arc<Mutex<Vec<User>>>;
async fn create_user(state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut users = state.lock().unwrap();
    let new_user = User {
        age: user.age,
        name: user.name.clone(),
    };
    users.push(new_user);
    HttpResponse::Ok().json(user)
}

async fn get_users(state:web::Data<AppState>)->impl Responder{
    let users=state.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state=web::Data::new(Arc::new(Mutex::new(Vec::<User>::new())));
    HttpServer::new(move||
         App::new()
        .app_data(state.clone()) 
        .route("/user",web::post().to(create_user))
        .route("/", web::get().to(get_users)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
