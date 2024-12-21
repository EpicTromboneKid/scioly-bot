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

    let event_list = match utils::user_handling::find_user(&ctx.author().id.to_string()) {
        Ok(user) => user.events,
        Err(e) => return Err("No events found; please register with `/set_defaults`!".into()),
    };

    for event in event_list {
        let event_id = format!("{}{}", &ctx.id(), &event);
        event_id_list.push((event, event_id));
    }

    const TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_secs(3600);

    println!("{invoke_time:?}");

    let mut event = String::new();
    let ctx_id = ctx.id();
    let finish_id = format!("{}finish", &ctx_id);
    let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename()).await?;
    let mut file_id = String::new();
    let mut perms = Vec::new();

    embeds::send_start_embed(ctx, &event_id_list, &invoke_time).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(TIMEOUT_DURATION)
        .await
    {
        if event_id_list
            .iter()
            .any(|(_, id)| id == &press.data.custom_id)
        {
            let (actual_event, _) = event_id_list
                .iter()
                .find(|(_, id)| id == &press.data.custom_id)
                .unwrap();
            event = actual_event.clone();
            println!("{}", ctx.author().name.as_str());
            let user = crate::utils::user_handling::find_user(&ctx.author().id.to_string())?;
            let scioly_docs = google::gdocs::instantiate_hub(secrets::servicefilename())
                .await
                .expect("gdocs instantiation failed");

            let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename()).await?;
            (file_id, perms) = embeds::send_test_embed(
                ctx,
                &press,
                (actual_event, &user.team),
                &finish_id,
                &user.default_email,
                (&scioly_docs, &scioly_drive),
            )
            .await?;
        } else if press.data.custom_id == finish_id {
            for perm in &perms {
                let (newemail, permission) = perm;
                google::gdrive::change_perms(
                    &scioly_drive,
                    &file_id,
                    crate::utils::Perms::Viewer(),
                    vec![newemail],
                    (true, permission.clone()),
                )
                .await?;
            }

            embeds::send_finish_embed(ctx, &press, &event, &finish_id, &scioly_drive, &file_id)
                .await?;
        } else {
            continue;
        }
    }

    Ok(())
}
