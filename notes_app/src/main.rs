use actix_web::{web, App, HttpResponse, HttpServer, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
#[derive(Serialize, Debug, Deserialize, Clone)]
struct Note {
    id: u32,
    title: String,
    content: String,
}
#[derive(Clone)]
struct AppState {
    notes: Arc<Mutex<Vec<Note>>>,
}

#[derive(Deserialize)]
struct NoteRequest {
    title: String,
    content: String,
}
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from actix")
}
async fn add_note(note:web::Json<NoteRequest>,db:web::Data<AppState>) -> impl Responder{
    let mut notes=db.notes.lock().unwrap();
    let id=notes.len() as u32 +1 ;
    let new_note=Note{
        content:note.content.clone(),
        id:id,
        title:note.title.clone()
    };
    notes.push(new_note.clone());
    HttpResponse::Created().json(new_note)
}

async fn get_notes(db:web::Data<AppState>)->impl Responder{
    let notes=db.notes.lock().unwrap();
    HttpResponse::Ok().json(notes.clone())
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(AppState{
        notes:Arc::new(Mutex::new(Vec::new()))
    });
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(hello))
            .route("/notes",web::post().to(add_note))
            .route("/notes", web::get().to(get_notes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
