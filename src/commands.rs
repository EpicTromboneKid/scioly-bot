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
    #[description = "Year of Invitational"] year: Option<u32>,
    #[description = "Invitational, if regionals or states, specify nCA or sCA"] invy: Option<
        String,
    >,
    #[description = "School of interest"] school: Option<String>,
    #[description = "Event of interest (i.e. Chem Lab)"] event: Option<String>,
    #[description = "Division"] division: Option<char>,
) -> Result<(), Error> {
    let year = year.expect("No year given!");
    let invy = invy.expect("No invitational given!");
    let school = school.expect("No school given!");
    let event = event.expect("No event given!");
    let division = match division {
        Some(div) => div.to_string(),
        None => "c".to_string(),
    };
    let event_clone = event.clone();

    println!("{} {} {} {} {}", &year, &invy, &school, &event, &division);
    let query = parse_file::Query::build_query(
        year.clone(),
        invy.clone(),
        school.clone(),
        event.clone(),
        division.clone(),
    );
    let x = query.find_rank()?.to_string();
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
