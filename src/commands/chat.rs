use crate::utils::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn chat(ctx: Context<'_>) -> Result<(), Error> {
    poise::say_reply(ctx, "chat it might be over :(").await?;
    Ok(())
}
