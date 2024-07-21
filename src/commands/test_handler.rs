use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::utils::{Context, Error};

pub mod testing {
    use crate::utils::Error;
    use std::default::Default;

    pub struct Test {
        year: u32,
        place: String,
        event: String,
        division: String,
        file: Vec<u8>,
        filetype: String,
        has_parts: bool,
        parts: u32,
        allotted_time: u32,
        id: u64,
    }
    impl Test {
        pub fn start_test(&self) -> Result<(), Error> {
            todo!();
        }
        pub fn end_test(&self) -> Result<(), Error> {
            todo!();
        }
        pub fn year(&mut self, year: u32) -> Result<(), Error> {
            self.year = year;
            Ok(())
        }
        pub fn place(&mut self, place: String) -> Result<(), Error> {
            self.place = place;
            Ok(())
        }
        pub fn event(&mut self, event: String) -> Result<(), Error> {
            self.event = event;
            Ok(())
        }
        pub fn allotted_time(&mut self, allotted_time: u32) -> Result<(), Error> {
            self.allotted_time = allotted_time;
            Ok(())
        }
    }
}

#[poise::command(
    slash_command,
    subcommands("start", "end"),
    subcommand_required,
    global_cooldown = 10
)]
pub async fn test(_ctx: Context<'_>) -> Result<(), Error> {
    println!("ok no subcommand given but its ok");
    Ok(())
}

#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>, event: String) -> Result<(), Error> {
    let invoke_time = chrono::Utc::now()
        .time()
        .format("%-I:%M %p UTC")
        .to_string();
    println!("{invoke_time:?}");
    let invoke_title = event;
    let invoke_footer = poise::serenity_prelude::CreateEmbedFooter::new(format!(
        "Your invocation of this command was at {}.",
        invoke_time,
    ));
    let invoke_embed = CreateEmbed::default()
        .color(poise::serenity_prelude::Color::PURPLE)
        .footer(invoke_footer)
        .title(invoke_title);
    let invoke_reply = CreateReply::default().embed(invoke_embed).ephemeral(false);
    ctx.send(invoke_reply).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn end(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("end").await?;
    Ok(())
}
