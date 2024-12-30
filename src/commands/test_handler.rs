use crate::commands::embeds;
use crate::commands::google;
use crate::secrets;
use crate::utils;
use crate::utils::{Context, Error};
use poise::serenity_prelude::{self as serenity};

const TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_secs(3600);

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
    const EVENT_TIME: u32 = 3000;
    let ctx_id = ctx.id();
    let abort_id = format!("{}abort", &ctx_id);
    let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename()).await?;
    let scioly_docs = google::gdocs::instantiate_hub(secrets::servicefilename()).await?;
    let scioly_sheets = google::gsheets::instantiate_hub(secrets::servicefilename()).await?;
    let finish_id = format!("{}finish", &ctx_id);
    let mut emails: Option<Vec<String>> = None;
    let mut file_id = String::new();
    let mut perms = Vec::new();

    let event_list = match utils::user_handling::find_user(&ctx.author().id.to_string()) { Ok(user) => user.events, Err(_) => std::panic::panic_any("No events found; please register with `/set_defaults`!, or check your roles in this server."), };

    for event in event_list {
        let event_id = format!("{}{}", &ctx.id(), &event);
        event_id_list.push((event, event_id));
    }
    embeds::send_init_embed(ctx, &event_id_list, &invoke_time, &abort_id).await?;

    let mut the_event_id = String::new();
    let mut the_event = String::new();

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(TIMEOUT_DURATION)
        .await
    {
        println!("event id: {}", the_event_id);
        if press.data.custom_id == abort_id {
            embeds::send_abort_embed(ctx, &press).await?;
            return Ok(());
        } else if press.data.custom_id == the_event_id {
            press.defer(ctx).await?;
            ctx.channel_id().broadcast_typing(ctx).await?;
            let user = crate::utils::user_handling::find_user(&ctx.author().id.to_string())?;
            (file_id, perms) = embeds::send_test_embed(
                ctx,
                &press,
                (&the_event, &user.team),
                &finish_id,
                emails.clone().unwrap(),
                (&scioly_docs, &scioly_drive, &scioly_sheets),
            )
            .await?;

            println!("in event_id {}", the_event_id);

            loop {
                tokio::select! {
                    Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
                        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
                        .timeout(TIMEOUT_DURATION) => {
                        let press_custom_id = press.data.custom_id.clone();
                        if press_custom_id == finish_id {
                            press.defer(ctx).await?;
                            embeds::send_finish_embed( ctx, &press, &the_event, &finish_id, &perms, &scioly_drive, &file_id ).await?;
                            return Ok(());
                        } else {
                            continue;
                        }
                    },
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs((EVENT_TIME-300).into())) => {
                        let partners = crate::utils::user_handling::get_event_partners(&the_event, &ctx.author().id.to_string(), &user.team)?;
                        let partner_ids = partners.iter().map(|partner| partner.userid.to_string()).collect::<Vec<String>>();
                        let remind_string = match partner_ids.len() {
                            1 => {
                                format!("<@{}> and <@{}>, you have 5 minutes left to submit the test. If you do not, it will be automatically submitted.", partner_ids[0], &ctx.author().id)
                            },
                            2 => {
                                format!("<@{}>, <@{}>, and <@{}>, you have 5 minutes left to submit the test. If you do not, it will be automatically submitted.", partner_ids[0], partner_ids[1], &ctx.author().id)
                            },
                            _ => return Err("You seem to have an incorrect amount of partners. Please contact an officer to fix this.".into()),
                        };

                        ctx.channel_id().say(ctx, remind_string).await?;


                            tokio::select! {
                                Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
                                    .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
                                    .timeout(TIMEOUT_DURATION) => {
                                    let press_custom_id = press.data.custom_id.clone();
                                    if press_custom_id == finish_id {
                                        press.defer(ctx).await?;
                                        embeds::send_finish_embed( ctx, &press, &the_event, &finish_id, &perms, &scioly_drive, &file_id ).await?;
                                        return Ok(());
                                    } else {
                                        continue;
                                    }
                                },
                                _ = tokio::time::sleep(tokio::time::Duration::from_secs(300)) => {
                                    embeds::send_finish_embed(ctx, &press, &the_event, &finish_id, &perms, &scioly_drive, &file_id).await?;
                                    return Ok(());
                                }
                            }

                    }
                }
            }
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
        } else {
            continue;
        }
    }
    Ok(())
}
