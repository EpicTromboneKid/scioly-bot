use crate::commands::google;
use crate::secrets::discord_api_key;
use crate::utils::{Context, Error, Perms};
use google_docs1::api::Document;

use google_docs1::hyper_rustls::HttpsConnector;
use google_docs1::hyper_util::client::legacy::connect::HttpConnector;
use google_drive3::api::Permission;
use poise::serenity_prelude::colours::branding::GREEN;

use poise::{
    serenity_prelude::{
        self as serenity, ButtonStyle, Color, CreateActionRow, CreateButton, CreateEmbed,
        CreateEmbedFooter,
    },
    CreateReply,
};

pub async fn send_start_embed(
    ctx: Context<'_>,
    event_list_ids: &Vec<(String, String)>,
    invoke_time: &String,
) -> Result<(), Error> {
    let mut actionrow = Vec::new();
    let invoke_footer = CreateEmbedFooter::new(format!(
        "Your invocation of this command was at {}.",
        invoke_time,
    ));

    let invoke_embed = CreateEmbed::default()
        .color(Color::PURPLE)
        .footer(invoke_footer)
        .title("Start a test!");

    for (event, start_button_id) in event_list_ids {
        let start_button = CreateButton::new(start_button_id)
            .label(format!("Start Test for {}", event))
            .emoji('🔬')
            .style(ButtonStyle::Success);
        actionrow.push(start_button);
    }

    let invoke_components = vec![CreateActionRow::Buttons(actionrow)];

    let invoke_reply = CreateReply::default()
        .embed(invoke_embed)
        .ephemeral(false)
        .components(invoke_components);

    let _ = ctx.send(invoke_reply).await?;

    Ok(())
}

pub async fn send_test_embed(
    ctx: Context<'_>,
    press: &serenity::ComponentInteraction,
    reqinfo: (&String, &char),
    finish_id: &String,
    email: &str,
    sciolyhubs: (
        &google_docs1::api::Docs<HttpsConnector<HttpConnector>>,
        &google_drive3::api::DriveHub<HttpsConnector<HttpConnector>>,
    ),
) -> Result<(String, Vec<(String, Permission)>), Error> {
    let (event, team) = reqinfo;
    let (sciolydocs, sciolydrive) = sciolyhubs;
    let req = Document {
        title: Some(format!(
            "{} Team {}, {}",
            event,
            team.to_uppercase(),
            chrono::Utc::now().date_naive()
        )),
        ..Default::default()
    };

    //println!("{:?}", result);

    let result = sciolydocs.documents().create(req).doit().await?;

    let file_id = result.1.document_id.expect("where is the doc id?");
    // insert link to answer doc here
    let doc_url: String = format!("https://docs.google.com/document/d/{}/edit", file_id);
    //println!("{doc_url}");

    // insert link to test here; must be input onto a sheet ig
    let test_url =
                "[Link to test](https://github.com/serenity-rs/poise/blob/current/examples/event_handler/main.rs)";

    let test_components = CreateActionRow::Buttons(vec![CreateButton::new(finish_id)
        .emoji('✅')
        .style(ButtonStyle::Danger)
        .label("Submit Test")]);

    let test_embed = CreateEmbed::default()
        .color(Color::BLUE)
        .title(format!("Answer Google Doc for {}", &event))
        .url(doc_url)
        .description(format!("This is the link to the test: {}", test_url));

    let builder = serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .embed(test_embed.clone())
            .components(vec![test_components.clone()]),
    );

    if press
        .create_response(ctx.serenity_context(), builder.clone())
        .await
        .is_err()
    {
        press
            .message
            .delete(poise::serenity_prelude::Http::new(discord_api_key()))
            .await?;
        ctx.send(
            CreateReply::default()
                .embed(test_embed)
                .components(vec![test_components]),
        )
        .await?;
    };
    Ok((
        file_id.clone(),
        google::gdrive::change_perms(
            sciolydrive,
            &file_id,
            Perms::Editor(),
            vec![email],
            (false, Permission::default()),
        )
        .await?,
    ))
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
        .style(ButtonStyle::Success)
        .label("Submit Test")
        .disabled(true)]);

    let finish_embed = CreateEmbed::default()
        .color(GREEN)
        .title(format!("Your {} test has been submitted!", event));

    let finish_builder = serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .embed(finish_embed.clone())
            .components(vec![finish_components.clone()]),
    );
    if press
        .create_response(ctx.serenity_context(), finish_builder.clone())
        .await
        .is_err()
    {
        press
            .message
            .delete(poise::serenity_prelude::Http::new(discord_api_key()))
            .await?;
        ctx.send(
            CreateReply::default()
                .embed(finish_embed)
                .components(vec![finish_components]),
        )
        .await?;
    }

    Ok(())
}
