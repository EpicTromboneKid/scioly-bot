use crate::utils::{Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "made by epictrombonekid",
            ephemeral: false,
            ..Default::default()
        },
    )
    .await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    //ctx.say("<:sciolybot1:1395186612559810580>").await?;
    Ok(())
}
