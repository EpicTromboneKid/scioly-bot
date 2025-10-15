use crate::utils::{Context, Error};
use chrono::{offset, TimeZone};
use poise::{
    serenity_prelude::{self as serenity, CreateMessage},
    CreateReply,
};
use std::collections::BinaryHeap;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Priority {
    High = 3,
    Medium = 2,
    Low = 1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reminder {
    user_id: serenity::UserId,
    message: String,
    time: chrono::DateTime<chrono::Utc>,
    priority: Priority,
}

impl Ord for Reminder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Reminder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Reminder {
    pub fn new(
        user_id: serenity::UserId,
        message: String,
        time: chrono::DateTime<chrono::Utc>,
        priority: Priority,
    ) -> Self {
        Self {
            user_id,
            message,
            time,
            priority,
        }
    }
}

#[poise::command(slash_command)]
pub async fn createreminder(
    ctx: Context<'_>,
    message: String,
    #[rename = "when"]
    #[description = "When should I remind you? You can use natural language like 'in 5 minutes' or 'tomorrow 3 pm'."]
    mut unparsed_time: String,
    priority: String,
) -> Result<(), Error> {
    let user_id = ctx.author().id;
    if unparsed_time.starts_with("in ") {
        unparsed_time = unparsed_time.trim_start_matches("in ").to_string();
        unparsed_time.push_str(" from now");
    }

    let parsed_time = fuzzydate::parse(&unparsed_time);

    if let Err(e) = parsed_time {
        match e {
            fuzzydate::Error::ParseError => {
                let error_message = format!(
                    "Could not parse the time: `{e}`. Please use a valid date or time format.",
                );
                let _ = ctx
                    .send(CreateReply::default().content(&error_message))
                    .await;
                return Ok(());
            }
            fuzzydate::Error::InvalidDate(e) => {
                let error_message = format!(
                    "Invalid date provided: `{e}`. Please use a valid date or time format.",
                );
                let _ = ctx
                    .send(CreateReply::default().content(&error_message))
                    .await;
                return Ok(());
            }
            fuzzydate::Error::UnrecognizedToken(token) => {
                let error_message = format!(
                    "Unrecognized word in date: `{token}`. Please use a valid date or time format.",
                );
                let _ = ctx
                    .send(CreateReply::default().content(&error_message))
                    .await;
                return Ok(());
            }
        }
    }

    let parsed_time = parsed_time.unwrap();
    let reminder_message = format!(
        "Your reminder about {message} is set for: {unparsed_time} ({} PT)",
        &parsed_time.format("%Y-%m-%d at %H:%M")
    );

    let _ = user_id
        .dm(ctx, CreateMessage::new().content(&reminder_message))
        .await;

    let _ = ctx
        .send(CreateReply::default().content(&reminder_message))
        .await;

    let reminder = Reminder::new(
        user_id,
        message,
        chrono::Utc.from_local_datetime(&parsed_time).unwrap(), // Fixed line
        match priority.to_lowercase().as_str() {
            "high" | "h" => Priority::High,
            "medium" | "m" => Priority::Medium,
            "low" | "l" => Priority::Low,
            _ => Priority::Low,
        },
    );

    let mut queue = BinaryHeap::new();
    queue.push(reminder.clone());

    println!("Created reminder: {:?}", reminder);

    Ok(())
}
