use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};

#[derive(serde::Serialize, Clone)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(serde::Deserialize)]
struct CreateTodo {
    title: String,
    completed: bool,
}

type AppState = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Home" }))
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo).put(update_todo))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_todos(State(state): State<AppState>) -> (StatusCode, Json<Vec<Todo>>) {
    let todos = state.lock().unwrap();
    (StatusCode::OK, Json(todos.clone()))
}

async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = state.lock().unwrap();
    let id = todos.last().map_or(1, |t| t.id + 1);
    let todo = Todo {
        id,
        title: payload.title,
        completed: payload.completed,
    };
    todos.push(todo.clone());
    (StatusCode::CREATED, Json(todo))
}

async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> (StatusCode, Json<Option<Todo>>) {
    let todos = state.lock().unwrap();
    match todos.iter().find(|t| t.id == id) {
        Some(todo) => (StatusCode::OK, Json(Some(todo.clone()))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Option<Todo>>) {
    let mut todos = state.lock().unwrap();
    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            todo.title = payload.title;
            todo.completed = payload.completed;
            (StatusCode::OK, Json(Some(todo.clone())))
        }
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
