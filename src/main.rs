pub mod query;
mod events;
use salvo::prelude::*;

#[handler]
async fn get_todos(res: &mut Response) {

    let sentence = query::RequestBody { query: "SELECT * FROM Todo;".to_string() };

    match query::todo_query(sentence).await {
        Ok(api_todos) => {
            let todos: Vec<query::Todo> = api_todos.iter()
                .flat_map(|api_element| api_element.result.clone())
                .collect();
            res.render(Json(todos));
        }
        Err(err) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Error: {}", err));
        }
    }
}

#[handler]
async fn create_todo(req: &mut Request, res: &mut Response) {
    let reqbody = req.parse_json::<query::RequestBody>().await;
    let sentence = match reqbody {
        Ok(request_body) => request_body,

        Err(_) => panic!("error al paresar el request body")
    };

    match query::todo_query(sentence).await {
        Ok(api_todos) => {
            let todos: Vec<query::Todo> = api_todos.iter()
                .flat_map(|api_element| api_element.result.clone())
                .collect();
            events::send_todo(1, serde_json::to_string(&todos).unwrap());
            res.render(Json(todos));
        }
        Err(err) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Error: {}", err));
        }
    }

}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::with_path("todos")
        .get(get_todos)
        .post(create_todo)
        .push(
            Router::with_path("events")
            .get(events::event_handler)
        );

    Server::new(TcpListener::bind("192.168.60.104:7878")).serve(router).await;
}
