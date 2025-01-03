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

    let invoke_embed = CreateEmbed::default()
        .description("Please select an event to start a test!")
        .title("Start a test!");

    let options = vec![CreateSelectMenuOption::new("what.", "1")];
    let invoke_components = vec![CreateActionRow::SelectMenu(CreateSelectMenu::new(
        "what.",
        poise::serenity_prelude::CreateSelectMenuKind::String { options },
    ))];

    let invoke_reply = CreateReply::default()
        .embed(invoke_embed)
        .ephemeral(false)
        .components(invoke_components);

    let _ = ctx.send(invoke_reply).await?;

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

    let name = match user.nick_in(ctx, ctx.guild_id().unwrap()).await {
        Some(name) => name,
        None => user.name.clone(),
    };

    let builder =
        CreateMessage::new().content(format!("This is a reminder from {}: {}", name, message));

    userid.dm(ctx, builder).await?;

    let _ = &ctx.say(format!("Reminder sent to {}!", user.name)).await?;
    Ok(())
}
