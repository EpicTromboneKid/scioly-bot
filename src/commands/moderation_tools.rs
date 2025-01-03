use poise::command;
use poise::serenity_prelude as serenity;

use crate::utils::*;
#[command(prefix_command, slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn ban(
    ctx: Context<'_>,
    user: serenity::User,
    #[description = "number of days to delete messages from this user (0-7)"]
    delete_messages: Option<u8>,
) -> Result<(), Error> {
    let userid = user.id;

    let _ = &ctx
        .guild_id()
        .unwrap()
        .ban(ctx, userid, delete_messages.unwrap_or(0))
        .await?;

    let _ = &ctx.say(format!("Banned {}!", user.name)).await?;
    Ok(())
}

#[command(prefix_command, slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn kick(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let userid = user.id;

    let _ = &ctx.guild_id().unwrap().kick(ctx, userid).await?;

    let _ = &ctx.say(format!("Kicked {}!", user.name)).await?;
    Ok(())
}
