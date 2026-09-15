use crate::{commands::roster, utils::{self, Context, Error, user_handling::{self, SciolyUser}}};

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
        "FOR ALL EVENTS: CHECK OUT THE RULES!!! [2025 Rules](http://soinc.org/rules-2025) \n
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
#[poise::command(prefix_command, slash_command, rename = "reg")]
pub async fn register(
    ctx: Context<'_>,
	#[description = "your email"] email: String,
    #[description = "default email to use with the bot"] #[rest] name: String,
) -> Result<(), Error> {
	println!("{name}");
    let userid = &ctx.author().id.to_string();
    let author_member = &ctx.author_member().await.unwrap();
    let member_roles = author_member.roles(ctx).unwrap();
    let mut roles = Vec::new();
    let mut team = 'z';
    let mut officer = member_roles.iter().find(|r| r.name.contains("officer")).is_some();
	let mut the_user = SciolyUser::default();

    // for role in member_roles {
    //     if role.name.contains("Team") && role.name.len() == 6 {
    //         team = role.name.chars().last().unwrap();
    //         println!("team: {}", team);
    //     } else if role.name.contains("Officer") {
    //         officer = true;
    //     }
    //     roles.push(role.name);
    // }

    let events = utils::events::extract_events(&roles);

    let mut users = user_handling::get_user_data("userdata.json")?;

    let the_user = roster::roster_sync(ctx, name, userid.to_string(), officer, email.clone()).await?;

    let _ = &ctx
        .say(format!(
            "Your defaults have been set to: email: {}, team: {}, events: {:?}",
            the_user.default_email, 
            match &the_user.team {
                'd' => "Alternates".to_string(),
                _ => the_user.team.to_string(),
            }, 
            the_user.events
        ))
        .await?;

    if let Some(guild_id) = ctx.guild_id() {
        if let Ok(guild_roles) = guild_id.roles(&ctx.http()).await {
            let mut author_member = ctx.author_member().await.unwrap();

            for event_name in &the_user.events {
                let role_id = if let Some((id, _role)) = guild_roles.iter().find(|(_, r)| r.name.eq_ignore_ascii_case(event_name)) {
                    *id
                } else {
                    match guild_id.create_role(&ctx.http(), serenity::builder::EditRole::new().name(event_name)).await {
                        Ok(new_role) => {
                            println!("Created new role: {}", event_name);
                            new_role.id
                        }
                        Err(e) => {
                            println!("Failed to create role {}: {:?}", event_name, e);
                            continue;
                        }
                    }
                };

                let _ = author_member.add_role(&ctx.http(), role_id).await;
            }
        }
    }

    if let Some(existing_user) = users.iter_mut().find(|u| &u.userid == userid) {
        *existing_user = the_user;
    } else {
        users.push(the_user);
    }

    user_handling::write_user_data("userdata.json", users)?;

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    rename = "ssd"
)]
pub async fn set_server_defaults(
    ctx: Context<'_>,
    #[description = "default email to use with the bot"] server_email: String,
    #[description = "file id of the google sheet that contains the main roster"] roster_file_id: String,
    #[description = "file id of the google sheet that the progress checks will be written to"]
    pc_file_id: String,
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
        server.server_email = server_email.trim().to_string();
        server.roster_file_id = roster_file_id.trim().to_string();
        server.pc_file_id = pc_file_id.trim().to_string();
    } else {
        let new_server = crate::utils::server_handling::Server {
            server_id: server_id.clone(),
            server_name,
            server_email: server_email.trim().to_string(),
            roster_file_id: roster_file_id.trim().to_string(),
            pc_file_id: pc_file_id.trim().to_string(),
        };
        servers.servers.push(new_server);
    }

    crate::utils::server_handling::write_server_data("serverdata.json", servers)?;

    let _ = &ctx
        .say(format!(
            "Your server defaults have been set to: email: {}, \nroster: https://docs.google.com/spreadsheets/d/{}/edit, \nprogress checks: https://docs.google.com/spreadsheets/d/{}/edit",
            &server_email, &roster_file_id, &pc_file_id
        ))
        .await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    rename = "rsd"
)]
pub async fn read_server_defaults(
    ctx: Context<'_>,
) -> Result<(), crate::utils::Error> {
    let mut servers = crate::utils::server_handling::get_server_data("serverdata.json")?;
	let server_email;
	let roster_file_id;
	let pc_file_id;
    let server_id = ctx.guild_id().unwrap().to_string();

    let server = servers
        .servers
        .iter_mut()
        .find(|s| s.server_id == server_id);

    if let Some(server) = server {
        server_email = server.server_email.trim().to_string();
        roster_file_id = server.roster_file_id.trim().to_string();
        pc_file_id = server.pc_file_id.trim().to_string();
    } else {
		let _ = &ctx.say("server does not seem to be registered :(");
		return Ok(())
    }


    let _ = &ctx
        .say(format!(
            "Your server defaults are: email: {}, \nroster: https://docs.google.com/spreadsheets/d/{}/edit, \nprogress checks: https://docs.google.com/spreadsheets/d/{}/edit. \nTo make changes, run `/ssd`.",
            server_email, roster_file_id, pc_file_id
        ))
        .await?;
    Ok(())
}
