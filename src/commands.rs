use crate::{Context, Error};

#[poise::command(prefix_command, track_edits)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "made by ***************",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits)]
// this is the rank query function, taking in 4 arguments: year, invy, school, event
pub async fn rq(ctx: Context<'_>, _command: Option<String>) -> Result<(), Error> {
    poise::say_reply(ctx, "oof").await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits)]
pub async fn chat(ctx: Context<'_>, _command: Option<String>) -> Result<(), Error> {
    poise::say_reply(ctx, "chat it might be over :(").await?;
    Ok(())
}
