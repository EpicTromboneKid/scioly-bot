use crate::{Context, Error};
use chrono::{Datelike, Utc};
use poise::serenity_prelude as serenity;
use scioly_bot::rank_query::parse_file;
use std::{collections::HashMap, panic::panic_any};
use String;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "made by ***************",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Command to get the placement of an event at an invitational (can only be used as
/// a slash command).
///
/// `/rq aggie Lynbrook nCA Chem Lab` would find the placement at aggie 2024 for LHS Chem lab.
#[poise::command(track_edits, slash_command)]
// this is the rank query function, taking in 4 arguments: year, invy, school, event
pub async fn rq(
    ctx: Context<'_>,
    #[description = "Year of Invitational, defaults to the current year"] year: Option<u32>,
    #[description = "Invitational"] invy: Option<String>,
    #[description = "School of interest"] school: Option<String>,
    #[description = "State of Invitational"] state: Option<String>,
    #[description = "Event of interest (i.e. Chem Lab)"] event: Option<String>,
    #[description = "Division, defaults to Div. C"] division: Option<String>,
) -> Result<(), Error> {
    let arg_hash_map = HashMap::from([
        (0, "year"),
        (1, "invitational"),
        (2, "school"),
        (3, "state"),
        (4, "event"),
        (5, "division"),
    ]);

    let qyear = year.unwrap_or(Utc::now().year().try_into()?);
    let qinvy = invy
        .unwrap_or("-1".to_string())
        .to_string()
        .trim()
        .to_string();
    let qschool = school.unwrap_or("-1".to_string()).trim().to_string();
    let qstate = state.unwrap_or("-1".to_string()).trim().to_string();
    let qevent = event.unwrap_or("-1".to_string()).trim().to_string();
    let qdivision: String = division.unwrap_or("c".to_string()).to_string();
    let array = [&qyear.to_string(), &qinvy, &qschool, &qevent, &qdivision];
    let mut input = String::new();

    println!(
        "{} {} {} {} {}",
        &qyear, &qinvy, &qschool, &qevent, &qdivision
    );

    for (i, element) in array.iter().enumerate() {
        if element.contains("-1") {
            if let Some(arg) = arg_hash_map.get(&i) {
                println!("found an argument with -1 lol");
                input.push_str(arg);
                input.push_str(" ");
            }
        }
    }

    if input.len() != 0 {
        panic_any(format!("Provide the following arguments: {}", input));
    }

    let query = parse_file::Input::build_input(
        qyear.clone().try_into()?,
        qinvy.clone().to_string(),
        qschool.clone(),
        qstate.clone(),
        qevent.clone(),
        qdivision.clone(),
    )?;

    let x = query.find_rank()?.to_string();
    let out_string = format!(
        "{} {}'s placement at {} {} is: {} :)",
        &qschool, &qevent, &qinvy, &qyear, &x
    );

    query.print_fields();
    println!("{x}");
    poise::say_reply(ctx, out_string).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn chat(ctx: Context<'_>) -> Result<(), Error> {
    poise::say_reply(ctx, "chat it might be over :(").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn resources(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

#[poise::command(prefix_command, slash_command, track_edits, rename = "ewr")]
pub async fn embed_with_rxns(ctx: Context<'_>) -> Result<(), Error> {
    let ctx_id = ctx.id();
    println!("ctx id: {}", ctx_id);
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji('◀'),
            serenity::CreateButton::new(&next_button_id).emoji('▶'),
        ]);
        // this is how to put hyperlinks in embed, just put this in a string "[SOINC](https://soinc.org)"
        crate::CreateReply::default()
            .ephemeral(true)
            .embed(serenity::CreateEmbed::default().title("Science Olympiad Resources!"))
            .components(vec![components])
    };

    ctx.send(reply).await?;

    Ok(())
}
