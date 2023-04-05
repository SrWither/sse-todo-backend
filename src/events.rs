use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures_util::StreamExt;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use salvo::prelude::*;
use salvo::sse::{SseEvent, SseKeepAlive};

#[derive(Debug)]
enum Todo {
    // UserId(usize),
    JsonData(String),
}

type Users = Mutex<HashMap<usize, mpsc::UnboundedSender<Todo>>>;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static ONLINE_USERS: Lazy<Users> = Lazy::new(Users::default);

#[handler]
pub async fn event_handler(res: &mut Response) {
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    tracing::info!("New ToDo user: {}", my_id);
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    ONLINE_USERS.lock().insert(my_id, tx);

    let stream = rx.map(|todo| match todo {
        // Todo::UserId(my_id) => Ok::<_, salvo::Error>(SseEvent::default().name("user").data(my_id.to_string())),
        Todo::JsonData(reply) => Ok::<SseEvent, salvo::Error>(SseEvent::default().data(reply)),
    });

    SseKeepAlive::new(stream).streaming(res).ok();
}

pub fn send_event(my_id: usize, data: String) {
    let new_msg = format!("{data}");
    ONLINE_USERS.lock().retain(|uid, tx| {
        if my_id == *uid {
            tx.send(Todo::JsonData(new_msg.clone())).is_ok()
        } else {
            tx.send(Todo::JsonData(new_msg.clone())).is_ok()
        }
    });
}
