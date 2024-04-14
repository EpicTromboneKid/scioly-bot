use crate::{Context, Error};
use scioly_bot::parse_file;
use String;

#[poise::command(prefix_command, track_edits)]
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
    year: Option<u32>,
    invy: Option<String>,
    school: Option<String>,
    mut event: Option<String>,
) -> Result<(), Error> {
    let year = year.expect("No year given!");
    let invy = invy.expect("No invitational given!");
    let school = school.expect("No school given!");
    let event_clone = event.clone().expect("nothing given???");
    let event = event.expect("No event given!");
    println!("{} {} {} {}", &year, &invy, &school, &event);
    let query =
        parse_file::Query::build_query(year.clone(), invy.clone(), school.clone(), event.clone());
    let x = query.find_rank().to_string();
    let out_string = format!(
        "{} {}'s placement at {} {} is: {} :)",
        &school, &event_clone, &invy, &year, &x
    );
    query.print_fields();
    println!("{x}");
    poise::say_reply(ctx, out_string).await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits)]
pub async fn chat(ctx: Context<'_>) -> Result<(), Error> {
    poise::say_reply(ctx, "chat it might be over :(").await?;
    Ok(())
}
