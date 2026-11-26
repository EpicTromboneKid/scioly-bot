use google_docs1::hyper_rustls::HttpsConnector;
use google_docs1::hyper_util::client::legacy::connect::HttpConnector;
use std::collections::BinaryHeap;

use crate::commands::reminder::Reminder;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub type SciolyHubs<'a> = (
    &'a google_docs1::api::Docs<HttpsConnector<HttpConnector>>,
    &'a google_drive3::api::DriveHub<HttpsConnector<HttpConnector>>,
    &'a google_sheets4::api::Sheets<HttpsConnector<HttpConnector>>,
);

pub static REMINDER_QUEUE: OnceCell<Arc<Mutex<BinaryHeap<Reminder>>>> = OnceCell::const_new();

#[derive(Debug)]
pub struct Data {}

pub enum Perms {
    Viewer(),
    Commenter(),
    Editor(),
    Owner(),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Teams {
	TeamA,
	TeamB,
	TeamC,
	Alternates,
	NoTeam
}

// Sub-modules
pub mod user;
pub mod server;
pub mod events;

// Re-exports for convenience
pub use user::{get_user_data, write_user_data, find_user, get_event_partners, get_officers_emails, get_event_id_list, SciolyUser};
pub use server::{get_server_data, write_server_data, get_server};
pub use events::{extract_events, find_closest_event_name, match_event_type, Types};

// Legacy compatibility: Thing struct stays here
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Thing {
    pub users: Vec<SciolyUser>,
}
