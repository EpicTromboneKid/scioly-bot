#[poise::command(prefix_command, track_edits, slash_command)]

pub async fn register_commands(ctx: crate::utils::Context<'_>) -> Result<(), crate::utils::Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
