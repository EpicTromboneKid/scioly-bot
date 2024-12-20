use crate::utils::{user_handling, Context, Error};

use poise::{
    serenity_prelude::{self as serenity, CreateEmbedFooter},
    CreateReply,
};

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

#[poise::command(prefix_command, slash_command)]
pub async fn set_email(ctx: Context<'_>, email: String) -> Result<(), Error> {
    let username = &ctx.author().name;
    println!("username: {}", username);
    println!("email: {}", email);

    let mut users = user_handling::get_user_data("userdata.json");

    let user = users.iter_mut().find(|u| &u.username == username);

    if let Some(user) = user {
        user.default_email = email.clone();
    } else {
        let new_user = user_handling::SciolyUser {
            username: username.to_string(),
            default_email: email.clone(),
            team: 'z',
            events: Vec::new(),
        };
        users.push(new_user);
    }

    user_handling::write_user_data("userdata.json", users)?;

    let _ = &ctx
        .say(format!("Your default email has been set to {}!", &email))
        .await?;

    Ok(())
}
