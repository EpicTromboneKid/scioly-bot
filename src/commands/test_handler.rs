#[allow(unused_imports)]
use poise::{
    serenity_prelude::{
        self as serenity, ButtonStyle, Color, CreateActionRow, CreateButton, CreateEmbed,
        CreateEmbedFooter,
    },
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
    subcommands("end", "test_start"),
    subcommand_required,
    global_cooldown = 10
)]
pub async fn test(_ctx: Context<'_>) -> Result<(), Error> {
    println!("ok no subcommand given but its ok");
    Ok(())
}

#[poise::command(slash_command, track_edits, rename = "start")]
pub async fn test_start(ctx: Context<'_>, event: String) -> Result<(), Error> {
    let invoke_time = chrono::Utc::now()
        .time()
        .format("%-I:%M %p UTC")
        .to_string();

    const TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_secs(60);

    println!("{invoke_time:?}");
    let invoke_title = &event;

    let invoke_footer = CreateEmbedFooter::new(format!(
        "Your invocation of this command was at {}.",
        invoke_time,
    ));

    let invoke_embed = CreateEmbed::default()
        .color(Color::PURPLE)
        .footer(invoke_footer)
        .title(invoke_title);

    let ctx_id = ctx.id();

    let start_button_id = format!("{}starttest", ctx_id);

    let start_button = CreateButton::new(&start_button_id)
        .label("Start Test")
        .emoji('🔬')
        .style(ButtonStyle::Success);

    let invoke_components = vec![CreateActionRow::Buttons(vec![start_button])];

    let invoke_reply = CreateReply::default()
        .embed(invoke_embed)
        .ephemeral(false)
        .components(invoke_components);

    let _ = ctx.send(invoke_reply).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(TIMEOUT_DURATION)
        .await
    {
        let finish_id = format!("{}finish", ctx_id);
        if press.data.custom_id == start_button_id {
            send_test_embed(ctx, &press, &event, &finish_id).await?;
        } else if press.data.custom_id == finish_id {
            println!("salut");
        } else {
            continue;
        }
    }

    Ok(())
}

#[poise::command(slash_command, track_edits)]
pub async fn end(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("end").await?;
    Ok(())
}

async fn send_test_embed(
    ctx: Context<'_>,
    press: &serenity::ComponentInteraction,
    event: &String,
    finish_id: &String,
) -> Result<(), Error> {
    // insert link to answer doc here
    let doc_url = "https://docs.rs/poise/latest/poise/serenity_prelude/struct.CreateEmbed.html";

    // insert link to test here; must be input onto a sheet ig
    let test_url =
                "[Link to test](https://github.com/serenity-rs/poise/blob/current/examples/event_handler/main.rs)";

    let ctx_id = ctx.id();

    let test_components = CreateActionRow::Buttons(vec![CreateButton::new(finish_id)
        .emoji('✅')
        .style(ButtonStyle::Danger)
        .label("Submit Test")]);

    let test_embed = CreateEmbed::default()
        .color(Color::BLUE)
        .title(format!("Answer Google Doc for {}", &event))
        .url(doc_url)
        .description(format!("This is the link to the test: {}", test_url));

    let builder = serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .embed(test_embed)
            .components(vec![test_components]),
    );

    press
        .create_response(ctx.serenity_context(), builder)
        .await?;

    Ok(())
}
