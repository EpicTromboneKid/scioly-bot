use crate::secrets;

#[poise::command(prefix_command, track_edits, slash_command, owners_only, hide_in_help)]
pub async fn register_commands(ctx: crate::utils::Context<'_>) -> Result<(), crate::utils::Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command, owners_only, hide_in_help)]
pub async fn delete_file(
    _ctx: crate::utils::Context<'_>,
    file_id: String,
) -> Result<(), crate::utils::Error> {
    let drive =
        crate::commands::google::gdrive::instantiate_hub(secrets::servicefilename()).await?;

    let (_, about) = drive.about().get().param("fields", "*").doit().await?;

    println!("{:?}", about.storage_quota);

    let _ = drive.files().delete(&file_id).doit().await?;

    Ok(())
}
