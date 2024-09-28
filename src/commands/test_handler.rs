use crate::commands::embeds;
use crate::utils::{Context, Error};
#[allow(unused_imports)]
use poise::serenity_prelude::colours::branding::GREEN;
use poise::serenity_prelude::{
    self as serenity, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed,
};

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

#[poise::command(slash_command, track_edits, rename = "test", global_cooldown = 10)]
pub async fn test(ctx: Context<'_>, event: String) -> Result<(), Error> {
    let invoke_time = chrono::Utc::now()
        .time()
        .format("%-I:%M %p UTC")
        .to_string();
    let actual_event = crate::utils::events::find_closest_event_name(event)?;
    const TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_secs(60);

    println!("{invoke_time:?}");

    let ctx_id = ctx.id();
    let start_button_id = format!("{}starttest", &ctx_id);
    let finish_id = format!("{}finish", &ctx_id);

    embeds::send_start_embed(ctx, &actual_event, &start_button_id, &invoke_time).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(TIMEOUT_DURATION)
        .await
    {
        if press.data.custom_id == start_button_id {
            embeds::send_test_embed(ctx, &press, &actual_event, &finish_id).await?;
        } else if press.data.custom_id == finish_id {
            let finish_components = CreateActionRow::Buttons(vec![CreateButton::new(&finish_id)
                .emoji('✅')
                .style(ButtonStyle::Success)
                .label("Submit Test")
                .disabled(true)]);
            let finish_embed = CreateEmbed::default()
                .color(GREEN)
                .title("Your test has been submitted!");
            let finish_builder = serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new()
                    .embed(finish_embed)
                    .components(vec![finish_components]),
            );
            press
                .create_response(ctx.serenity_context(), finish_builder)
                .await?
        } else {
            continue;
        }
    }

    Ok(())
}
