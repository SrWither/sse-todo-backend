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
pub struct RequestBody {
    pub query: String,
}

// pub async fn get_query() -> RqResult<Vec<ApiResponse>> {
//     let client = reqwest::Client::new();
//
//     let response = client
//         .post("http://0.0.0.0:8000/sql")
//         .header(ACCEPT, "application/json")
//         .header("NS", "Todos")
//         .header("DB", "Todos")
//         .basic_auth("root", Some("root"))
//         .body("SELECT * FROM Todo;")
//         .send()
//         .await?;
//
//     let parsed_result = if response.status() == reqwest::StatusCode::OK {
//         response.json::<Vec<ApiResponse>>().await?
//     } else {
//         panic!("Error!");
//     };
//
//     // for api_element in &parsed_result {
//     //     for element in &api_element.result {
//     //         println!("{}", element.task);
//     //     }
//     // }
//     Ok(parsed_result)
// }

pub async fn todo_query(body: RequestBody) -> RqResult<Vec<ApiResponse>> {
    let client = reqwest::Client::new();

    let response = client
        .post("http://0.0.0.0:8000/sql")
        .header(ACCEPT, "application/json")
        .header("NS", "Todos")
        .header("DB", "Todos")
        .basic_auth("root", Some("root"))
        .body(body.query)
        .send()
        .await?;

    let parsed_result = if response.status() == reqwest::StatusCode::OK {
        response.json::<Vec<ApiResponse>>().await?
    } else {
        panic!("Error!");
    };

    // for api_element in &parsed_result {
    //     for element in &api_element.result {
    //         println!("{}", element.task);
    //     }
    // }
    Ok(parsed_result)
}
