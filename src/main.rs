pub mod query;
mod events;
use salvo::prelude::*;

#[handler]
async fn get_todos(res: &mut Response) {

    match query::get_query().await {
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
    let reqbody = req.parse_json::<query::CreateTodo>().await;
    let task = match reqbody {
        Ok(request_body) => request_body,

        Err(_) => panic!("error al paresar el request body")
    };

    match query::create_query(task).await {
        Ok(api_todos) => {
            let todos: Vec<query::Todo> = api_todos.iter()
                .flat_map(|api_element| api_element.result.clone())
                .collect();

            let data = serde_json::to_string(&todos).unwrap();
            events::send_event(1, format!("NewTodo: {}", data));
            res.render(Json(todos));
        }
        Err(err) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Error: {}", err));
        }
    }
}

#[handler]
async fn update_todo(req: &mut Request, res: &mut Response) {
    let reqbody = req.parse_json::<query::ModifyTodo>().await;
    let task = match reqbody {
        Ok(request_body) => request_body,

        Err(_) => panic!("error al paresar el request body")
    };

    let todo_id = task.id.clone();

    match query::update_query(task).await {
        Ok(api_todos) => {
            let todos: Vec<query::Todo> = api_todos.iter()
                .flat_map(|api_element| api_element.result.clone())
                .collect();

            events::send_event(1, format!("UpdateTodo: {}", todo_id));
            res.render(Json(todos));
        }
        Err(err) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Error: {}", err));
        }
    }
}

#[handler]
async fn delete_todo(req: &mut Request, res: &mut Response) {
    let reqbody = req.parse_json::<query::ModifyTodo>().await;
    let task = match reqbody {
        Ok(request_body) => request_body,

        Err(_) => panic!("error al paresar el request body")
    };

    let todo_id = task.id.clone();

    match query::delete_query(task).await {
        Ok(_) => {
            let resp = query::CustomResponse {
                msg: "ToDo removed successfully".to_string()
            };

            events::send_event(1, format!("DeleteTodo: {}", todo_id));
            res.render(Json(resp));
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
        .patch(update_todo)
        .delete(delete_todo)
        .push(
            Router::with_path("events")
            .get(events::event_handler)
        );

    Server::new(TcpListener::bind("192.168.60.104:7878")).serve(router).await;
}
