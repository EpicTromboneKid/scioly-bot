use crate::commands::google;
use crate::secrets::{self, discord_api_key};
use crate::utils::{Context, Error};
use google_docs1::api::{Document, Scope};
use poise::serenity_prelude::colours::branding::GREEN;
use poise::serenity_prelude::CacheHttp;
use poise::{
    serenity_prelude::{
        self as serenity, ButtonStyle, Color, CreateActionRow, CreateButton, CreateEmbed,
        CreateEmbedFooter,
    },
    CreateReply,
};

pub async fn send_start_embed(
    ctx: Context<'_>,
    event: &String,
    start_button_id: &String,
    invoke_time: &String,
) -> Result<(), Error> {
    let invoke_footer = CreateEmbedFooter::new(format!(
        "Your invocation of this command was at {}.",
        invoke_time,
    ));

    let invoke_embed = CreateEmbed::default()
        .color(Color::PURPLE)
        .footer(invoke_footer)
        .title(event);

    let start_button = CreateButton::new(start_button_id)
        .label("Start Test")
        .emoji('🔬')
        .style(ButtonStyle::Success);

    let invoke_components = vec![CreateActionRow::Buttons(vec![start_button])];

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
    event: &String,
    finish_id: &String,
    team: &char,
) -> Result<(), Error> {
    println!("{}", secrets::servicefilename());

    let scioly_docs = google::gdocs::instantiate_hub(secrets::servicefilename())
        .await
        .expect("gdocs instantiation failed");

    let scioly_drive = google::gdrive::instantiate_hub(secrets::servicefilename())
        .await
        .expect("drive instantiation failed");

    let req = Document {
        title: Some(format!(
            "{} Team {}, {}",
            event,
            team.to_uppercase(),
            chrono::Utc::now().date_naive()
        )),
        ..Default::default()
    };

    let result = scioly_docs
        .documents()
        .create(req)
        .add_scope(Scope::Drive)
        .doit()
        .await?;

    //println!("{:?}", result);

    // Share the document with sciolybot@gmail.com
    let permission = google_drive3::api::Permission {
        role: Some("writer".to_string()),
        type_: Some("user".to_string()),
        email_address: Some("sciolybot@gmail.com".to_string()),
        ..Default::default()
    };

    scioly_drive
        .permissions()
        .create(
            permission,
            result
                .1
                .document_id
                .clone()
                .as_ref()
                .expect("where is the doc id?"),
        )
        .doit()
        .await?;

    // insert link to answer doc here
    let doc_url: String = format!(
        "https://docs.google.com/document/d/{}/edit",
        result.1.document_id.expect("where is the doc id?")
    );
    println!("{doc_url}");

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
    }
    Ok(())
}

pub async fn send_finish_embed(
    ctx: Context<'_>,
    press: &serenity::ComponentInteraction,
    event: &String,
    finish_id: &String,
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
            .embed(finish_embed)
            .components(vec![finish_components]),
    );

    press
        .create_response(ctx.serenity_context(), finish_builder)
        .await?;

    Ok(())
}
