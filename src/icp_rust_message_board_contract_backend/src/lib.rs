#[macro_use]
extern crate serde;
use std::{collections::HashMap, cell::RefCell, sync::atomic::AtomicUsize, sync::atomic::Ordering};
use ic_cdk::api::time;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Message {
    id: usize,
    title: String,
    body: String,
    attachment_url: String,
    created_at: u64,
    updated_at: Option<u64>
}

#[derive(candid::CandidType, Clone, Deserialize)]
struct MessageBoard {
    pub message_storage: HashMap<usize, Message>,
}

impl Default for MessageBoard {
    fn default() -> Self {
        MessageBoard { message_storage: HashMap::new() }
    }
}

// a struct to hold an ID counter since uuid does not seem to work for wasm32-unknown-unknown
// so we will use int ids
struct Counter {
    pub counter: AtomicUsize
}

impl Default for Counter {
    fn default() -> Self {
        Counter {counter: AtomicUsize::new(0)}
    }
}

thread_local! {
    static SERVICE: RefCell<MessageBoard> = RefCell::default();
    static ID_COUNTER: RefCell<Counter> = RefCell::default();
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct MessagePayload {
    title: String,
    body: String,
    attachment_url: String,
}

#[ic_cdk_macros::query]
fn get_message(id: usize) ->  Result<Message, String> {
    match _get_message(&id) {
        Some(message) => Ok(message),
        None => Err(format!("a message with id={} not found", id))
    }
}

#[ic_cdk_macros::update]
fn add_message(message: MessagePayload) -> Option<Message> {
    let id = ID_COUNTER.with(|service| service.borrow_mut().counter.fetch_add(1, Ordering::SeqCst));
    let message = Message { 
        id: id.clone(),
        title: message.title, 
        body: message.body, 
        attachment_url: message.attachment_url, 
        created_at: time(), 
        updated_at: None 
    };
    SERVICE.with(|service| service.borrow_mut().message_storage.insert(id, message.clone()));
    Some(message)
}

#[ic_cdk_macros::update]
fn update_message(id: usize, payload: MessagePayload) -> Result<Message, String> {
    match SERVICE.with(|service| service.borrow_mut().message_storage.get(&id).cloned()) {
        Some(mut message) => {
            message.attachment_url = payload.attachment_url;
            message.body = payload.body;
            message.title = payload.title;
            message.updated_at = Some(time());
            Ok(message)
        },
        None => Err(format!("couldn't update a message with id={}. message not found", id))
    }
}

#[ic_cdk_macros::update]
fn delete_message(id: usize) -> Result<Message, String> {
    match SERVICE.with(|service| service.borrow_mut().message_storage.remove(&id)) {
        Some(message) => Ok(message),
        None => Err(format!("couldn't delete a message with id={}. message not found.", id))
    }
}


// a helper method to get a message by id. used in get_message/update_message
fn _get_message(id: &usize) -> Option<Message> {
    SERVICE.with(|service| service.borrow().message_storage.get(id).cloned())
}

// need this to generate candid
ic_cdk_macros::export_candid!();
