use crate::utils::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn chat(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("chat it might be over :(").await?;
    Ok(())
}
