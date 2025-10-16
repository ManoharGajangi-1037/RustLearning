use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize,Deserialize};
async fn hello()->impl Responder{
    HttpResponse::Ok().body("hello")
}
#[derive(Debug,Serialize,Deserialize)]
struct Note{
    title:String,
    description:String
}
type AppState=Arc<Mutex<Vec<Note>>>;
async fn add_note(state:web::Data<AppState>,note:web::Json<Note>)->impl Responder{
     let mut  notes=state.lock().unwrap();
     notes.push(Note { title: note.title.clone(), description: note.description.clone()});
     HttpResponse::Ok().json(note)
}
async fn get_notes(state:web::Data<AppState>){
    let notes=state.lock().unwrap();
    HttpResponse::Ok().json(&*notes);
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state=web::Data::new(Arc::new(Mutex::new(Vec::<Note>::new())));
    HttpServer::new(move 
         || App::new()
        .app_data(state.clone())
        .route("/", web::get().to(hello)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
