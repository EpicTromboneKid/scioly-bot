use std::time::SystemTime;

use crate::commands::embeds;
use crate::commands::google;
use crate::secrets;
use crate::utils;
use crate::utils::{Context, Error};
use poise::serenity_prelude::{self as serenity};

#[poise::command(
    prefix_command,
    slash_command,
    track_edits,
    rename = "test",
    user_cooldown = 20
)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    let mut event_id_list = Vec::new();
    let invoke_time = chrono::Utc::now()
        .time()
        .format("%-I:%M %p UTC")
        .to_string();
    const TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_secs(3600);
    const _EVENT_TIME: std::time::Duration = std::time::Duration::from_secs(18);

    let ctx_id = ctx.id();
    let mut file_id = String::new();

    let abort_id = format!("{}abort", &ctx_id);

    let mut the_event_id = String::new();
    let mut the_event = String::new();

    let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename()).await?;
    let scioly_docs = google::gdocs::instantiate_hub(secrets::servicefilename()).await?;
    let scioly_sheets = google::gsheets::instantiate_hub(secrets::servicefilename()).await?;
    let mut perms = Vec::new();
    let finish_id = format!("{}finish", &ctx_id);
    let mut emails: Option<Vec<String>> = None;

    let event_list = match utils::user_handling::find_user(&ctx.author().id.to_string()) {
        Ok(user) => user.events,
        Err(_) => std::panic::panic_any("No events found; please register with `/set_defaults`!, or check your roles in this server."),
    };

    for event in event_list {
        let event_id = format!("{}{}", &ctx.id(), &event);
        event_id_list.push((event, event_id));
    }

    embeds::send_init_embed(ctx, &event_id_list, &invoke_time, &abort_id).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(TIMEOUT_DURATION)
        .await
    {
        println!("event id: {}", the_event_id);
        if press.data.custom_id == abort_id {
            embeds::send_abort_embed(ctx, &press).await?;
            break;
        } else if press.data.custom_id == the_event_id {
            press.defer(ctx).await?;
            let user = crate::utils::user_handling::find_user(&ctx.author().id.to_string())?;

            println!("in event_id");

            (file_id, perms) = embeds::send_test_embed(
                ctx,
                &press,
                (&the_event, &user.team),
                &finish_id,
                emails.clone().unwrap(),
                (&scioly_docs, &scioly_drive, &scioly_sheets),
            )
            .await?;

            println!(" hi there{:?}{:?}", ctx.data(), SystemTime::now());

            println!("in event_id {}", the_event_id);
        } else if press.data.custom_id == finish_id {
            press.defer(ctx).await?;

            embeds::send_finish_embed(
                ctx,
                &press,
                &the_event,
                &finish_id,
                &perms,
                &scioly_drive,
                &file_id,
            )
            .await?;
        } else if event_id_list
            .iter()
            .any(|(_, id)| id == &press.data.custom_id)
        {
            press.defer(ctx).await?;

            let user = crate::utils::user_handling::find_user(&ctx.author().id.to_string())?;
            let (event, event_id) = event_id_list
                .iter()
                .find(|(_, id)| id == &press.data.custom_id)
                .unwrap();
            the_event = event.to_string();
            the_event_id = event_id.to_string();

            let stuff = embeds::send_start_embed(ctx, &press, event, event_id, &user.team).await?;
            emails = Some(stuff);
            println!("event id: {}", the_event_id);
        } else {
            continue;
        }
    }

    Ok(())
}
