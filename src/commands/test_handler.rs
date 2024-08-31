use poise::{
    serenity_prelude::{CreateActionRow, CreateButton, CreateEmbed, MessageBuilder},
    CreateReply,
};

use crate::utils::{Context, Error};

pub mod testing {
    use crate::utils::events;
    use chrono::Utc;

    pub struct Test {
        year: u32,
        place: String,
        event: events::Events,
        division: events::Division,
        has_parts: bool,
        parts: u32,
        allotted_time: u32,
        id: u64,
        start_time: chrono::DateTime<Utc>,
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

#[poise::command(slash_command, track_edits)]
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

    let start_button = CreateButton::new("start_button")
        .label("Start Test")
        .emoji('🔬')
        .style(poise::serenity_prelude::ButtonStyle::Success);

    let invoke_components = vec![CreateActionRow::Buttons(vec![start_button])];

    let invoke_reply = CreateReply::default()
        .embed(invoke_embed)
        .ephemeral(false)
        .components(invoke_components);

    let invoke_message = ctx.send(invoke_reply).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn end(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("end").await?;
    Ok(())
}
