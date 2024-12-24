use crate::commands::google::{self, gsheets};
use crate::utils::{Context, Error, Perms};
use google_docs1::api::Document;

use google_docs1::hyper_rustls::HttpsConnector;
use google_docs1::hyper_util::client::legacy::connect::HttpConnector;
use google_drive3::api::Permission;
use poise::serenity_prelude::colours::branding::GREEN;

use poise::ReplyHandle;
use poise::{
    serenity_prelude::{
        self as serenity, ButtonStyle, Color, CreateActionRow, CreateButton, CreateEmbed,
        CreateEmbedFooter,
    },
    CreateReply,
};

pub async fn send_abort_embed(
    ctx: Context<'_>,
    press: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let abort_embed = CreateEmbed::default()
        .color(Color::RED)
        .title("Test Aborted")
        .description("Your test has been aborted.");

    let abort_components = vec![CreateActionRow::Buttons(vec![CreateButton::new(
        "whatthefrik",
    )
    .emoji('❌')
    .style(ButtonStyle::Danger)
    .label("Aborted")
    .disabled(true)])];

    let builder = serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .embed(abort_embed.clone())
            .components(abort_components),
    );

    press.create_response(ctx, builder).await?;

    Ok(())
}

pub async fn send_init_embed(
    ctx: Context<'_>,
    event_list_ids: &Vec<(String, String)>,
    invoke_time: &String,
    abort_id: &String,
) -> Result<(), Error> {
    let mut actionrow = Vec::new();
    let invoke_footer = CreateEmbedFooter::new(format!(
        "Your invocation of this command was at {}.",
        invoke_time,
    ));

    let invoke_embed = CreateEmbed::default()
        .color(Color::PURPLE)
        .description("Please select an event to start a test!")
        .footer(invoke_footer)
        .title("Start a test!");

    for (event, start_button_id) in event_list_ids {
        let start_button = CreateButton::new(start_button_id)
            .label(format!("Start Test for {}", event))
            .emoji('🔬')
            .style(ButtonStyle::Secondary);
        actionrow.push(start_button);
    }

    let abort_button = CreateButton::new(abort_id)
        .label("Abort")
        .emoji('❌')
        .style(ButtonStyle::Danger);
    actionrow.push(abort_button);

    let invoke_components = vec![CreateActionRow::Buttons(actionrow)];

    let invoke_reply = CreateReply::default()
        .embed(invoke_embed)
        .ephemeral(false)
        .components(invoke_components);

    let _ = ctx.send(invoke_reply).await?;

    Ok(())
}

pub async fn send_start_embed<'a>(
    ctx: Context<'a>,
    press: &serenity::ComponentInteraction,
    event: &String,
    event_id: &String,
    team: &char,
) -> Result<(ReplyHandle<'a>, Vec<String>), Error> {
    let invoke_embed = CreateEmbed::default()
        .color(Color::PURPLE)
        .title(format!("Start the {} test!", event));

    let start_button = CreateButton::new(event_id)
        .label("Start Test")
        .emoji('🔬')
        .style(ButtonStyle::Success);

    let invoke_components = CreateActionRow::Buttons(vec![start_button]);

    let builder = serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .embed(invoke_embed.clone())
            .components(vec![invoke_components.clone()]),
    );

    let partners =
        crate::utils::user_handling::get_event_partners(event, &ctx.author().id.to_string(), team)?;

    let partner_ids = partners
        .iter()
        .map(|partner| partner.userid.to_string())
        .collect::<Vec<String>>();
    let mut emails = partners
        .iter()
        .map(|partner| partner.default_email.clone())
        .collect::<Vec<String>>();

    emails.push(
        crate::utils::user_handling::get_user_data("userdata.json")?
            .iter()
            .find(|user| user.userid == ctx.author().id.to_string())
            .unwrap()
            .default_email
            .clone(),
    );

    let oof =
        match partner_ids.len() {
            1 => {
                ctx.say(format!(
                    "When you and your partner <@{}> is ready, please start the test <@{}>!",
                    partner_ids[0], &ctx.author().id
                ))
                .await?
            }
            2 => {
                ctx.say(format!(
            "When you and your partners <@{}> and <@{}> are ready, please start the test <@{}>!",
             partner_ids[0], partner_ids[1], &ctx.author().id
        ))
                .await?
            }
            _ => return Err(
                "You seem to have an incorrect amount of partners. Please contact an officer to fix this."
                    .into(),
            ),
        };

    press.create_response(ctx, builder).await?;

    Ok((oof, emails))
}

pub async fn send_test_embed(
    ctx: Context<'_>,
    press: &serenity::ComponentInteraction,
    reqinfo: (&String, &char),
    finish_id: &String,
    emails: &Vec<String>,
    sciolyhubs: (
        &google_docs1::api::Docs<HttpsConnector<HttpConnector>>,
        &google_drive3::api::DriveHub<HttpsConnector<HttpConnector>>,
        &google_sheets4::api::Sheets<HttpsConnector<HttpConnector>>,
    ),
    reply: &ReplyHandle<'_>,
) -> Result<(String, Vec<(String, Permission)>), Error> {
    reply.delete(ctx).await?;
    let (event, team) = reqinfo;
    let (sciolydocs, sciolydrive, sciolysheets) = sciolyhubs;
    let req = Document {
        title: Some(format!(
            "{} Team {}, {}",
            event,
            team.to_uppercase(),
            chrono::Utc::now().date_naive()
        )),
        ..Default::default()
    };

    let sheet_id = "1MutocwAPR2Fwzj8PC9rQP3QqYzfcb91-D3fNnzrJLeI";

    let sheets = sciolysheets
        .spreadsheets()
        .values_get(sheet_id, "'test-sheet'!B:C")
        .doit()
        .await?
        .1
        .values
        .unwrap();
    println!("{:?}", sheets);

    let result = sciolydocs.documents().create(req).doit().await?;

    let file_id = result.1.document_id.expect("where is the doc id?");
    // insert link to answer doc here
    let doc_url: String = format!("https://docs.google.com/document/d/{}/edit", file_id);
    //println!("{doc_url}");

    // insert link to test here; must be input onto a sheet ig
    let test_url = match gsheets::get_test_link(event, sheets) {
        Some(url) => url,
        None => {
            return Err("Test URL not found, let one of the officers know.".into());
        }
    };

    let test_components = CreateActionRow::Buttons(vec![CreateButton::new(finish_id)
        .emoji('✅')
        .style(ButtonStyle::Primary)
        .label("Submit Test")]);

    let test_embed = CreateEmbed::default()
        .color(Color::BLUE)
        .title(format!("Answer Google Doc for {}", &event))
        .url(doc_url)
        .description(format!(
            "This is the [link]({}) to the test",
            &test_url[1..test_url.len() - 1]
        ));

    let builder = serenity::CreateInteractionResponseFollowup::new()
        .embed(test_embed)
        .components(vec![test_components]);

    press.create_followup(ctx, builder).await?;
    let out = google::gdrive::change_perms(
        sciolydrive,
        &file_id,
        Perms::Editor(),
        emails,
        (false, &Permission::default()),
    )
    .await?;
    Ok((file_id.clone(), out))
}

pub async fn send_finish_embed(
    ctx: Context<'_>,
    press: &serenity::ComponentInteraction,
    event: &String,
    finish_id: &String,
    _scioly_drive: &google_drive3::api::DriveHub<HttpsConnector<HttpConnector>>,
    _file_id: &str,
) -> Result<(), Error> {
    let finish_components = CreateActionRow::Buttons(vec![CreateButton::new(finish_id)
        .emoji('✅')
        .style(ButtonStyle::Primary)
        .label("Submit Test")
        .disabled(true)]);

    let finish_embed = CreateEmbed::default()
        .color(GREEN)
        .title(format!("Your {} test has been submitted!", event));

    let finish_builder = serenity::CreateInteractionResponseFollowup::new()
        .embed(finish_embed)
        .components(vec![finish_components]);

    press
        .create_followup(ctx.serenity_context(), finish_builder)
        .await?;

    Ok(())
}
