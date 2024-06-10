use crate::utils::{Context, Error};

#[poise::command(prefix_command, slash_command, subcommands("start", "end", "upload"))]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("test").await?;
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("start").await?;
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn end(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("end").await?;
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn upload(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("upload").await?;
    Ok(())
}

pub mod test_help {}
