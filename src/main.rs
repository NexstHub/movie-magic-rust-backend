use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Movie {
    id: u32,
    name: String,
}

// Global mutable state (this is unsafe in a multi-threaded context)
static mut MOVIES: Vec<Movie> = Vec::new();

async fn get_items() -> impl Responder {
    unsafe {
        HttpResponse::Ok().json(&MOVIES)  // Access global state
    }
}

async fn create_item(item: web::Json<Movie>) -> impl Responder {
    unsafe {
        MOVIES.push(item.into_inner());
    }

    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/movies", web::get().to(get_items))
            .route("/api/movies", web::post().to(create_item))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

