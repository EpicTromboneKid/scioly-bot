use crate::{Context, Error};
use scioly_bot::parse_file;

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

#[poise::command(prefix_command, track_edits)]
// this is the rank query function, taking in 4 arguments: year, invy, school, event
pub async fn rq(ctx: Context<'_>, _command: Option<String>) -> Result<(), Error> {
    let year = 2024;
    let invy = String::from("states");
    let event = String::from("Chemistry Lab");
    let school = String::from("Lynbrook");
    let query =
        parse_file::Query::build_query(year.clone(), invy.clone(), school.clone(), event.clone());
    let x = query.find_rank().to_string();
    let mut out_string = String::new();
    out_string.push_str(&school);
    out_string.push_str(" ");
    out_string.push_str(&event);
    out_string.push_str("'s placement at ");
    out_string.push_str(&invy);
    out_string.push_str(" ");
    out_string.push_str(&year.to_string());
    out_string.push_str(" is: ");
    out_string.push_str(&x);
    query.print_fields();
    println!("{x}");
    poise::say_reply(ctx, out_string).await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits)]
pub async fn chat(ctx: Context<'_>, _command: Option<String>) -> Result<(), Error> {
    poise::say_reply(ctx, "chat it might be over :(").await?;
    Ok(())
}
