use crate::commands::embeds;
use crate::commands::google;
use crate::secrets;
use crate::utils::{Context, Error};
use google_drive3::api::Permission;
use poise::serenity_prelude::{self as serenity};

#[poise::command(slash_command, track_edits, rename = "test", global_cooldown = 10)]
pub async fn test(ctx: Context<'_>, event: String, team: char) -> Result<(), Error> {
    let invoke_time = chrono::Utc::now()
        .time()
        .format("%-I:%M %p UTC")
        .to_string();

    let actual_event = crate::utils::events::find_closest_event_name(event)?;
    const TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_secs(3600);

    println!("{invoke_time:?}");

    let ctx_id = ctx.id();
    let start_button_id = format!("{}starttest", &ctx_id);
    let finish_id = format!("{}finish", &ctx_id);
    let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename()).await?;
    let mut file_id = String::new();
    let mut perms = Permission::default();

    embeds::send_start_embed(ctx, &actual_event, &start_button_id, &invoke_time).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(TIMEOUT_DURATION)
        .await
    {
        if press.data.custom_id == start_button_id {
            (file_id, perms) =
                embeds::send_test_embed(ctx, &press, &actual_event, &finish_id, &team).await?;
            println!("perms: {:?}", perms);
        } else if press.data.custom_id == finish_id {
            let permission = google_drive3::api::Permission {
                role: Some("reader".to_string()),
                type_: Some("user".to_string()),
                email_address: Some("chaaskandregula@gmail.com".to_string()),
                ..Default::default()
            };

            scioly_drive
                .permissions()
                .delete(&file_id, perms.id.as_ref().unwrap())
                .doit()
                .await?;

            scioly_drive
                .permissions()
                .create(permission, &file_id)
                .doit()
                .await?;

            embeds::send_finish_embed(
                ctx,
                &press,
                &actual_event,
                &finish_id,
                &scioly_drive,
                file_id.as_str(),
            )
            .await?;
        } else {
            continue;
        }
    }

    Ok(())
}
