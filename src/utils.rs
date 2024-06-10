pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {
    pub _votes: Mutex<HashMap<String, u32>>,
}
use std::{collections::HashMap, sync::Mutex};
