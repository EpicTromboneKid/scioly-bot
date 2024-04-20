use crate::{Context, Error};
use chrono::{Datelike, Utc};
use poise::serenity_prelude::{self, Error as serenityError};
use scioly_bot::parse_file;
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

#[poise::command(track_edits, slash_command)]
// this is the rank query function, taking in 4 arguments: year, invy, school, event
pub async fn rq(
    ctx: Context<'_>,
    #[description = "Year of Invitational, defaults to the current year"] year: Option<u32>,
    #[description = "Invitational, if regionals or states, specify nCA or sCA"] invy: Option<
        String,
    >,
    #[description = "School of interest"] school: Option<String>,
    #[description = "Event of interest (i.e. Chem Lab)"] event: Option<String>,
    #[description = "Division, defaults to Div. C"] division: Option<String>,
) -> Result<(), Error> {
    let arg_hash_map = HashMap::from([
        (0, "year"),
        (1, "invitational"),
        (2, "school"),
        (3, "event"),
        (4, "division"),
    ]);

    let qyear = year.unwrap_or(Utc::now().year().try_into()?);
    let qinvy = invy
        .unwrap_or("-1".to_string())
        .to_string()
        .trim()
        .to_string();
    let qschool = school.unwrap_or("-1".to_string()).trim().to_string();
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
            }
        }
    }
    if input.len() != 0 {
        panic_any(input);
    }

    let query = parse_file::Query::build_query(
        qyear.clone().try_into()?,
        qinvy.clone().to_string(),
        qschool.clone(),
        qevent.clone(),
        qdivision.clone(),
    );

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
