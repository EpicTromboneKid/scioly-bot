use crate::utils::{Context, Error};

pub mod test_starter {}

pub mod terminator {}

pub mod upload_handler {}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("start", "end", "upload"),
    subcommand_required
)]
pub async fn test(_ctx: Context<'_>) -> Result<(), Error> {
    println!("no subocmand given :(");
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
