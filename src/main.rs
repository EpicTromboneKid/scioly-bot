#![warn(clippy::str_to_string)]

use poise::{
    send_reply,
    serenity_prelude::{self as serenity, http, CreateEmbedFooter, UserId},
    CreateReply, FrameworkError,
};
use rustls::crypto::{self};
use scioly_bot::{
    commands::{
        admin_only as register, ai, chat, help, moderation_tools, pc_handler as progress_checks,
        resources, test_handler, user,
    },
    secrets,
    utils::{Data, Error, MODEL},
};
pub const BRAINROT_WORDS: [&str; 27] = [
    "skibidi",
    "rizz",
    "gyatt",
    "gyat",
    "sigma",
    "npc",
    "cap",
    "pov",
    "sus",
    "finsta",
    "fyp",
    "simp",
    "yeet",
    "bussin",
    "flex",
    "ghosting",
    "clout",
    "mid",
    "slaps",
    "ohio",
    "sussy",
    "fanum",
    "gronk",
    "livvy",
    "maxxing",
    "mog",
    "galvanized square",
];

use std::{sync::Arc, time::Duration};

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
            let _ = send_reply(
                ctx,
                poise::CreateReply::default().ephemeral(true).embed(
                    serenity::CreateEmbed::default()
                        .title(format!(
                            "There was an error in command {}: {}",
                            ctx.command().name,
                            error
                        ))
                        .color(serenity::Color::RED)
                        .footer(CreateEmbedFooter::new(
                            "if this keeps occurring, please let epictrombonekid know 💀"
                                .to_owned(),
                        )),
                ),
            )
            .await;
        }
        FrameworkError::CommandPanic {
            ref payload, ctx, ..
        } => {
            if payload.is_none() {
                println!("there was an error :(");
            }
            println!(
                "the input was {payload:?}, and the command was {}, error {error}",
                ctx.command().name
            );
            let embed = serenity::CreateEmbed::default()
                .color(serenity::Colour::DARK_RED)
                .footer(CreateEmbedFooter::new("there seems to be an error :("));
            let fake_reply = embed.title(payload.clone().expect("not an input??").to_string());
            let reply = CreateReply::default().embed(fake_reply).ephemeral(true);
            let _ = send_reply(ctx, reply).await;
        }

        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value

    let model = ai::initialize_model().await.unwrap();

    let _ = MODEL.set(model);

    let _ = crypto::aws_lc_rs::default_provider().install_default();
    let mut x: std::collections::HashSet<UserId> = std::collections::HashSet::new();
    x.insert(poise::serenity_prelude::UserId::new(742791701986541599));

    let options = poise::FrameworkOptions {
        // commands go here lol
        commands: vec![
            test_handler::test(),
            chat::chat(),
            help::help(),
            resources::resources(),
            register::register_commands(),
            resources::set_defaults(),
            resources::set_server_defaults(),
            progress_checks::spcr(),
            user::add(),
            progress_checks::remind(),
            moderation_tools::ban(),
            moderation_tools::kick(),
            ai::ai(),
            progress_checks::pc(),
        ],
        // commands go above this lol
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("sob"),
                poise::Prefix::Literal("SciOlyBot"),
                poise::Prefix::Literal("sciolybot"),
            ],
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: move |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        owners: x,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();
    let token = secrets::discord_api_key();
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}

async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message
                .mention_roles
                .contains(&serenity::RoleId::new(1324572846327988224))
            {
                println!("Someone mentioned the role!");
                new_message
                    .react(ctx, serenity::ReactionType::Unicode("👍".to_owned()))
                    .await?;
                new_message
                    .react(ctx, serenity::ReactionType::Unicode("👎".to_owned()))
                    .await?;
            }

            //FUNNY STUFFS
            //if new_message.author.id == serenity::UserId::new(742791701986541599) {
            //    new_message
            //        .react(ctx, serenity::ReactionType::Unicode("🧐".to_owned()))
            //        .await?;
            //} else if new_message.author.id == serenity::UserId::new(711342969835356231) {
            //    new_message
            //        .react(ctx, serenity::ReactionType::Unicode("🐐".to_owned()))
            //        .await?;
            //} else if new_message.author.id == serenity::UserId::new(773799550237278209) {
            //    new_message
            //        .react(ctx, serenity::ReactionType::Unicode("🫘".to_owned()))
            //        .await?;
            //}
            //} else if new_message.author.id == serenity::UserId::new(737781806136295440) {
            //    new_message
            //        .reply_ping(ctx, "# PHYSICS MAIN!!!!!!!!!")
            //        .await?;
            //}

            if BRAINROT_WORDS
                .iter()
                .any(|word| new_message.content.to_lowercase().contains(word))
            {
                new_message.delete(ctx).await?;
                new_message
                    .reply_mention(ctx, "no brainrot words >:(")
                    .await?;
            }
        }
        _ => {
            println!("Got an event! {:?}", event.snake_case_name());
        }
    }
    Ok(())
}
