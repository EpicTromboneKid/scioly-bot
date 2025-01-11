use poise::serenity_prelude::{
    CreateActionRow, CreateEmbed, CreateMessage, CreateSelectMenu, CreateSelectMenuOption,
    MessageFlags, UserId,
};

use poise::CreateReply;

use crate::utils::*;

#[poise::command(prefix_command, slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn sendprogchks(ctx: Context<'_>) -> Result<(), Error> {
    let users = user_handling::get_user_data("userdata.json")?;
    let prog_check_message =
        "Reminder to fill out the progress check form [here](https://tinyurl.com/sciolyprog2425)";

    ctx.channel_id().broadcast_typing(ctx).await?;

    for user in users {
        //let _ = &ctx
        //    .say(format!("User_id: {}, team: {}", user.userid, user.team,))
        //    .await?;

        if let 'z' = user.team {
            ctx.say(format!("<@{}> has not set a team", user.userid))
                .await?;
            continue;
        }

        let userid = UserId::new(user.userid.parse::<u64>().unwrap());
        let serenity_user = userid.to_user(ctx).await?;

        let username = match serenity_user.nick_in(ctx, ctx.guild_id().unwrap()).await {
            Some(name) => name,
            None => serenity_user.name,
        };

        let builder = CreateMessage::new()
            .flags(MessageFlags::SUPPRESS_EMBEDS)
            .content(prog_check_message.to_string() + &format!(", {}!", username));

        userid.dm(ctx, builder).await?;

        let _ = &ctx.say(format!("Sent to {}!", username)).await?;
    }
    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn remind(
    ctx: Context<'_>,
    user: poise::serenity_prelude::User,
    #[rest] message: String,
) -> Result<(), Error> {
    let userid = user.id;

    let name = match ctx.author().nick_in(ctx, ctx.guild_id().unwrap()).await {
        Some(name) => name,
        None => ctx.author().name.clone(),
    };

    let builder = CreateMessage::new().content(format!("This is a reminder from {}:", name));

    userid.dm(ctx, builder).await?;

    let builder = CreateMessage::new().content(message);

    userid.dm(ctx, builder).await?;

    let reply = CreateReply::default()
        .content(format!("Reminder sent to {}!", user.name))
        .ephemeral(true);

    let _ = &ctx.send(reply).await?;
    Ok(())
}
