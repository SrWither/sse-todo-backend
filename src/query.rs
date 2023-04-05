use serde::{Serialize, Deserialize};
use reqwest::{Result as RqResult, header::ACCEPT};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub completed: bool,
    pub id: String,
    pub task: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub result: Vec<Todo>,
    status: String,
    time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodo {
    pub task: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModifyTodo {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomResponse {
    pub msg: String,
}

pub async fn get_query() -> RqResult<Vec<ApiResponse>> {
    let client = reqwest::Client::new();

    let response = client
        .post("http://0.0.0.0:8000/sql")
        .header(ACCEPT, "application/json")
        .header("NS", "Todos")
        .header("DB", "Todos")
        .basic_auth("root", Some("root"))
        .body("SELECT * FROM Todo;")
        .send()
        .await?;

    let parsed_result = if response.status() == reqwest::StatusCode::OK {
        response.json::<Vec<ApiResponse>>().await?
    } else {
        panic!("Error!");
    };

    Ok(parsed_result)
}

pub async fn create_query(body: CreateTodo) -> RqResult<Vec<ApiResponse>> {
    let client = reqwest::Client::new();

    let response = client
        .post("http://0.0.0.0:8000/sql")
        .header(ACCEPT, "application/json")
        .header("NS", "Todos")
        .header("DB", "Todos")
        .basic_auth("root", Some("root"))
        .body(format!("CREATE Todo SET task = \"{}\", completed = false;", body.task))
        .send()
        .await?;

    let parsed_result = if response.status() == reqwest::StatusCode::OK {
        response.json::<Vec<ApiResponse>>().await?
    } else {
        panic!("Error!");
    };

    Ok(parsed_result)
}

pub async fn update_query(body: ModifyTodo) -> RqResult<Vec<ApiResponse>> {
    let client = reqwest::Client::new();

    let response = client
        .post("http://0.0.0.0:8000/sql")
        .header(ACCEPT, "application/json")
        .header("NS", "Todos")
        .header("DB", "Todos")
        .basic_auth("root", Some("root"))
        .body(format!("UPDATE Todo SET completed = not(completed) WHERE id = \"{}\";", body.id))
        .send()
        .await?;

    let parsed_result = if response.status() == reqwest::StatusCode::OK {
        response.json::<Vec<ApiResponse>>().await?
    } else {
        panic!("Error!");
    };

    Ok(parsed_result)
}

pub async fn delete_query(body: ModifyTodo) -> RqResult<Vec<ApiResponse>> {
    let client = reqwest::Client::new();

    let response = client
        .post("http://0.0.0.0:8000/sql")
        .header(ACCEPT, "application/json")
        .header("NS", "Todos")
        .header("DB", "Todos")
        .basic_auth("root", Some("root"))
        .body(format!("DELETE Todo WHERE id = \"{}\";", body.id))
        .send()
        .await?;

    let parsed_result = if response.status() == reqwest::StatusCode::OK {
        response.json::<Vec<ApiResponse>>().await?
    } else {
        panic!("Error!");
    };

    Ok(parsed_result)
}
