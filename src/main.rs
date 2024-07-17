use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

struct AppState {
    todo_list: Mutex<Vec<Todo>>,
}

async fn index() -> impl Responder {
    fs::NamedFile::open_async("./static/index.html").await.unwrap()
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todo_list = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todo_list)
}

async fn create_todo(data: web::Data<AppState>, todo: web::Json<Todo>) -> impl Responder {
    let mut todo_list = data.todo_list.lock().unwrap();
    let new_todo = Todo {
        id: 123, 
        title: todo.title.clone(),
        completed: todo.completed,
    };
    
    todo_list.push(new_todo.clone());
    HttpResponse::Ok().json(new_todo)
}

async fn update_todo(data: web::Data<AppState>, id: web::Path<usize>, todo: web::Json<Todo>) -> impl Responder {
    let mut todo_list = data.todo_list.lock().unwrap();
    if let Some(t) = todo_list.iter_mut().find(|t| t.id == *id) {
        t.title = todo.title.clone();
        t.completed = todo.completed;
        HttpResponse::Ok().json(t)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_todo(data: web::Data<AppState>, id: web::Path<usize>) -> impl Responder {
    let mut todo_list = data.todo_list.lock().unwrap();
    if let Some(index) = todo_list.iter().position(|t| t.id == *id) {
        todo_list.remove(index);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(vec![]),
    });
     println!("Starting server");


    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
