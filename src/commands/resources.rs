use crate::utils::{self, user_handling, Context, Error};

use poise::{
    serenity_prelude::{self as serenity, CreateEmbedFooter},
    CreateReply,
};

/// List of some helpful resources for scioly
#[poise::command(prefix_command, slash_command)]
pub async fn resources(ctx: Context<'_>) -> Result<(), Error> {
    let ctx_id = ctx.id();
    println!("ctx id: {}", ctx_id);
    let description = String::from(
        "FOR ALL EVENTS: CHECK OUT THE RULES!!! [2024 Rules](http://soinc.org/rules-2024) \n
        scioly.org has some tests through the test exchange: [scioly.org test exchange](https://scioly.org/tests) \n
        Materials from previous years: [LHS Scioly Material (from past years)](https://tinyurl.com/lhssciolymaterial) \n 
        huge test banks: [Scioly Test Bank](https://tinyurl.com/sciolytestbank) and [LHS test bank (sorted by event)](https://tinyurl.com/lhscse23)\n",
    );

    let reply = {
        // this is how to put hyperlinks in embed, just put this in a string "[SOINC](https://soinc.org)"
        CreateReply::default().embed(
            serenity::CreateEmbed::default()
                .title("Science Olympiad Resources!")
                .description(description)
                .color(serenity::Colour::DARK_GREEN)
                .footer(CreateEmbedFooter::new(
                    "More resources will be added as they are found! If any of the above links are outdated, or you have any other resources, please let one of the officers know!",
                )),
        )
    };

    ctx.send(reply).await?;

    Ok(())
}

/// This sets your default email for the bot, which you provide; everything else is auto-detected.
#[poise::command(prefix_command, slash_command)]
pub async fn set_defaults(
    ctx: Context<'_>,
    #[description = "default email to use with the bot"] email: String,
) -> Result<(), Error> {
    let userid = &ctx.author().id.to_string();
    let author_member = &ctx.author_member().await.unwrap();
    let member_roles = author_member.roles(ctx).unwrap();
    let mut roles = Vec::new();
    let mut team = 'z';
    let mut officer = false;

    for role in member_roles {
        if role.name.contains("Team") && role.name.len() == 6 {
            team = role.name.chars().last().unwrap();
            println!("team: {}", team);
        } else if role.name.contains("Officer") {
            officer = true;
        }
        roles.push(role.name);
    }

    let events = utils::events::extract_events(&roles);

    let mut users = user_handling::get_user_data("userdata.json")?;

    let user = users.iter_mut().find(|u| &u.userid == userid);

    if let Some(user) = user {
        user.default_email = email.clone();
        user.team = team;
        user.events = events.clone();
    } else {
        let new_user = user_handling::SciolyUser {
            userid: userid.to_string(),
            default_email: email.clone(),
            team,
            events: events.clone(),
            officer,
        };
        users.push(new_user);
    }

    user_handling::write_user_data("userdata.json", users)?;

    let _ = &ctx
        .say(format!(
            "Your defaults have been set to: email: {}, team: {}, events: {:?}",
            email, team, events
        ))
        .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn set_server_defaults(
    ctx: Context<'_>,
    club_email: String,
) -> Result<(), crate::utils::Error> {
    let server_id = ctx.guild_id().unwrap().to_string();
    let server_name = ctx.guild().unwrap().name.to_string();
    println!("guild id: {}", server_id);
    println!("server name: {}", server_name);

    let mut servers = crate::utils::server_handling::get_server_data("serverdata.json")?;

    let server = servers
        .servers
        .iter_mut()
        .find(|s| s.server_id == server_id);

    if let Some(server) = server {
        server.server_email = club_email.clone();
    } else {
        let new_server = crate::utils::server_handling::Server {
            server_id,
            server_name,
            server_email: club_email.clone(),
        };
        servers.servers.push(new_server);
    }

    crate::utils::server_handling::write_server_data("serverdata.json", servers)?;

    let _ = &ctx
        .say(format!(
            "Your server defaults have been set to: email: {}",
            club_email
        ))
        .await?;
    Ok(())
}
