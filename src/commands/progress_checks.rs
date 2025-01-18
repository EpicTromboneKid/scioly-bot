use crate::commands::{google, pc};
use crate::secrets;
use crate::utils::{self, Context, Error};
use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateEmbed, CreateMessage, CreateSelectMenu,
    CreateSelectMenuOption, MessageFlags, UserId,
};

use poise::CreateReply;

use crate::utils::*;

/// Sends a reminder to fill out the progress check form to all users who are in a team.
#[poise::command(prefix_command, slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn spcr(ctx: Context<'_>) -> Result<(), Error> {
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

#[poise::command(prefix_command, slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn pc(ctx: Context<'_>) -> Result<(), Error> {
    let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename()).await?;
    let scioly_sheets = google::gsheets::instantiate_hub(secrets::servicefilename()).await?;
    let prog_check_file_id = "1_ba7HMQUUVRWTPHi8DQLGWulvisxWn8fuyhqht8gaxw";
    let ctx_id = ctx.id();
    let mut event: Option<String> = None;

    // (event, event_id) is the format of the tuple
    let event_id_list = utils::user_handling::get_event_id_list(ctx)?;

    let abort_id = format!("{}abort", &ctx_id);

    pc::pc_start_embed(ctx, &event_id_list, &abort_id).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .await
    {
        if press.data.custom_id == abort_id {
            pc::pc_abort_embed(ctx, &press).await?;
        } else if event_id_list
            .iter()
            .any(|(_, event_id)| event_id == &press.data.custom_id)
        {
            event = Some(
                event_id_list
                    .iter()
                    .find(|(_, event_id)| event_id == &press.data.custom_id)
                    .unwrap()
                    .0
                    .to_string(),
            );
            press.message.delete(ctx).await?;
            pc::pc_event_handling(ctx, &event.expect("no event")).await?;
        }
    }
    Ok(())
}
