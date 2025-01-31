use crate::utils::{Context, Error};
use poise::macros::command;

#[command(prefix_command, slash_command, track_edits)]
pub async fn add(ctx: Context<'_>, a: f64, b: f64) -> Result<(), Error> {
    ctx.say(format!("{} + {} = {}", a, b, a + b)).await?;
    Ok(())
}
