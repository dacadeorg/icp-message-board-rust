#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::{
    BTreeMap as StableBTreeMap, BoundedStorable, DefaultMemoryImpl, Storable,
};
use std::{borrow::Cow, cell::RefCell, ops::Add};

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Message {
    id: u64,
    title: String,
    body: String,
    attachment_url: String,
    created_at: u64,
    updated_at: Option<u64>,
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
  static ID_COUNTER: RefCell<u64> = RefCell::new(0);

  static STORAGE: RefCell<StableBTreeMap<u64, Message, DefaultMemoryImpl>> =
    RefCell::new(StableBTreeMap::init(DefaultMemoryImpl::default()));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct MessagePayload {
    title: String,
    body: String,
    attachment_url: String,
}

#[ic_cdk::query]
fn get_message(id: u64) -> Result<Message, Error> {
    match _get_message(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a message with id={} not found", id).to_owned(),
        }),
    }
}

#[ic_cdk::update]
fn add_message(message: MessagePayload) -> Option<Message> {
    let id = ID_COUNTER.with(|counter: &RefCell<u64>| counter.borrow_mut().add(1));
    let message = Message {
        id,
        title: message.title,
        body: message.body,
        attachment_url: message.attachment_url,
        created_at: time(),
        updated_at: None,
    };
    do_insert(&message);
    Some(message)
}

#[ic_cdk::update]
fn update_message(id: u64, payload: MessagePayload) -> Result<Message, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut message) => {
            message.attachment_url = payload.attachment_url;
            message.body = payload.body;
            message.title = payload.title;
            message.updated_at = Some(time());
            do_insert(&message);
            Ok(message)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a message with id={}. message not found",
                id
            )
            .to_owned(),
        }),
    }
}

// helper method to perform insert.
fn do_insert(message: &Message) {
    STORAGE.with(|service| service.borrow_mut().insert(message.id, message.clone()));
}

#[ic_cdk::update]
fn delete_message(id: u64) -> Result<Message, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a message with id={}. message not found.",
                id
            )
            .to_owned(),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// a helper method to get a message by id. used in get_message/update_message
fn _get_message(id: &u64) -> Option<Message> {
    STORAGE.with(|service| service.borrow().get(id))
}

// need this to generate candid
ic_cdk::export_candid!();
