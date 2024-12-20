use std::panic::panic_any;

use crate::commands::embeds;
use crate::commands::google;
use crate::secrets;
use crate::utils::{Context, Error};
use poise::serenity_prelude::{self as serenity};

#[poise::command(slash_command, track_edits, rename = "test", global_cooldown = 20)]
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
    let mut perms = Vec::new();

    embeds::send_start_embed(ctx, &actual_event, &start_button_id, &invoke_time).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(TIMEOUT_DURATION)
        .await
    {
        if press.data.custom_id == start_button_id {
            println!("{}", ctx.author().name.as_str());
            let email = match crate::utils::user_handling::find_user(ctx.author().name.as_str()) {
                Some(email) => email,
                None => {
                    panic_any("You need to set your email first! Use `/set_email <email>` to set your email.");
                }
            };
            let scioly_docs = google::gdocs::instantiate_hub(secrets::servicefilename())
                .await
                .expect("gdocs instantiation failed");

            let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename()).await?;
            (file_id, perms) = embeds::send_test_embed(
                ctx,
                &press,
                (&actual_event, &team),
                &finish_id,
                &email,
                (&scioly_docs, &scioly_drive),
            )
            .await?;
            println!("perms: {:?}", perms);
        } else if press.data.custom_id == finish_id {
            for perm in &perms {
                let (newemail, permission) = perm;
                println!("email: {}, permission: {:?}", newemail, permission);
                google::gdrive::change_perms(
                    &scioly_drive,
                    &file_id,
                    crate::utils::Perms::Viewer(),
                    vec![newemail],
                    (true, permission.clone()),
                )
                .await?;
            }

            //scioly_drive
            //    .permissions()
            //    .delete(&file_id, perms.id.as_ref().unwrap())
            //    .doit()
            //    .await?;
            //
            //scioly_drive
            //    .permissions()
            //    .create(permission, &file_id)
            //    .doit()
            //    .await?;

            embeds::send_finish_embed(
                ctx,
                &press,
                &actual_event,
                &finish_id,
                &scioly_drive,
                &file_id,
            )
            .await?;
        } else {
            continue;
        }
    }

    Ok(())
}
