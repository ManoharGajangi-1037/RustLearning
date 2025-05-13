use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from actix")
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: String,
    title: String,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

type Db = Mutex<Vec<Todo>>;

async fn add_todo(todo: web::Json<CreateTodo>, db: web::Data<Db>) -> impl Responder {
    let mut todos = db.lock().unwrap();

    let new_todo = Todo {
        id: Uuid::new_v4().to_string(),
        done: false,
        title: todo.title.clone(),
    };
    todos.push(new_todo.clone());
    HttpResponse::Ok().json(new_todo)
}

async fn get_todos(db: web::Data<Db>) -> impl Responder {
    let todos = db.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}
async fn mark_todo(path: web::Path<String>, db: web::Data<Db>) -> impl Responder {
    let id = path.into_inner();
    let mut todos = db.lock().unwrap();
    for todo in todos.iter_mut() {
        if (todo.id == id) {
            todo.done = true;
            return HttpResponse::Ok().json(todo.clone());
        }
    }
    HttpResponse::NotFound().body("Not found")
}

async fn delete_todo(path:web::Path<String>,db:web::Data<Db>)-> impl Responder{
    let id=path.into_inner();
    let mut todos=db.lock().unwrap();
    let original_len=todos.len();
    todos.retain(|todo| todo.id !=id);
    if original_len >todos.len(){
       return HttpResponse::Ok().body(format!("Deleted todo id with {}",id))
    }
    
    HttpResponse::Ok().body(format!("The task is not found with the id {}",id))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Mutex::new(Vec::<Todo>::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(hello))
            .route("/todo", web::post().to(add_todo))
            .route("/todos", web::get().to(get_todos))
            .route("todo/{id}", web::put().to(mark_todo))
            .route("/todo/{id}",web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
