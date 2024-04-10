use crate::{Context, Error};

#[poise::command(prefix_command, track_edits)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "made by epictrombonekid",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits)]
pub async fn rq(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    let text = "lol";
    poise::say_reply(ctx, text).await?;
    Ok(())
}
