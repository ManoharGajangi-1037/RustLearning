use actix_web::{web::{self, route}, App, HttpResponse, HttpServer, Responder};

async fn hello()->impl Responder{
    HttpResponse::Ok().body("Hello from actix")
}
#[actix_web::main]
async fn  main()->Result<(), std::io::Error>{
  HttpServer::new(||{
    App::new().route("/", web::get().to(hello))
  }).bind("127.0.0.1:8000")?.run().await
}