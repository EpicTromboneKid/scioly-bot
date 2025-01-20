use crate::utils::{self, Context, Error};

use poise::serenity_prelude::{CreateInteractionResponseFollowup, EditInteractionResponse};
use poise::ReplyHandle;
use poise::{
    serenity_prelude::{
        self as serenity, ButtonStyle, Color, CreateActionRow, CreateButton, CreateEmbed,
        CreateEmbedFooter,
    },
    CreateReply,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ProgressCheck {
    event: String,
    duration: String,
    progress: String,
    improvements: String,
    timestamp: String,
    team: String,
}
impl ProgressCheck {
    pub fn event(&mut self, event: String) {
        self.event = event;
    }
    pub fn duration(&mut self, duration: String) {
        self.duration = duration;
    }
    pub fn progress(&mut self, progress: String) {
        self.progress = progress;
    }
    pub fn improvements(&mut self, improvements: String) {
        self.improvements = improvements;
    }
    pub fn timestamp(&mut self, timestamp: String) {
        self.timestamp = timestamp;
    }
    pub fn team(&mut self, team: char) {
        self.team = team.to_string();
    }
}

pub async fn pc_start_embed(
    ctx: Context<'_>,
    event_list_ids: &Vec<(String, String)>,
    abort_id: &String,
) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(ctx).await?;
    let mut buttons = Vec::new();

    for (event, event_id) in event_list_ids {
        let button = CreateButton::new(event_id)
            .label(event)
            .style(ButtonStyle::Primary)
            .emoji('📝');
        buttons.push(button);
    }

    buttons.push(
        CreateButton::new(abort_id)
            .label("Abort")
            .style(ButtonStyle::Danger)
            .emoji('❌'),
    );

    let embed = CreateEmbed::default()
        .title("Pick an event to start")
        .footer(CreateEmbedFooter::new("run !help for help!"));
    let reply = CreateReply::default()
        .embed(embed)
        .components(vec![CreateActionRow::Buttons(buttons)]);

    ctx.send(reply).await?;

    Ok(())
}
pub async fn pc_abort_embed(
    ctx: Context<'_>,
    press: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let abort_embed = CreateEmbed::default()
        .color(Color::RED)
        .title("Progress Check Aborted")
        .description("Your progress check has been aborted.");

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

pub async fn pc_event_handling(ctx: Context<'_>, event: &String) -> Result<ProgressCheck, Error> {
    ctx.channel_id().broadcast_typing(ctx).await?;
    let mut prog_check = ProgressCheck::default();
    prog_check.event(event.clone());
    let replyhandle = ctx.say("Starting progress check...").await?;

    prog_check.duration(
        send_questions(
            &ctx,
            &replyhandle,
            "How long did you meet with your partner for".to_string(),
            event,
        )
        .await?,
    );

    prog_check.progress(
        send_questions(
            &ctx,
            &replyhandle,
            "What progress did you make on".to_string(),
            event,
        )
        .await?,
    );

    prog_check.improvements(
        send_questions(
            &ctx,
            &replyhandle,
            "What improvements can you make for".to_string(),
            event,
        )
        .await?,
    );

    prog_check.timestamp(chrono::offset::Local::now().to_string());
    println!("{}", prog_check.timestamp);

    let confirmation_embed = CreateEmbed::default()
        .title(format!("Confirm that your Progress Check for {} is correct: ", event))
        .description(format!(
            "1. Duration: {}\n 2. Progress: {}\n 3. Improvements: {}\n",
            prog_check.duration, prog_check.progress, prog_check.improvements
        ))
        .color(poise::serenity_prelude::Color::DARK_ORANGE)
        .footer(CreateEmbedFooter::new(
            "If you would like to change any of the above answers, type the number of the question you would like to change after pressing ❌'.",
        ));

    let reply = CreateReply::default()
        .embed(confirmation_embed.clone())
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("whatthefriksubmitit")
                .emoji('✅')
                .style(ButtonStyle::Success)
                .label("Confirm"),
            CreateButton::new("whatthefrikiswrong")
                .emoji('❌')
                .style(ButtonStyle::Danger)
                .label("Nope"),
        ])]);

    ctx.send(reply).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .filter(move |press| press.data.custom_id.starts_with("whatthefrik"))
        .timeout(std::time::Duration::from_secs(100))
        .await
    {
        press.defer(ctx).await?;
        let final_embed = CreateEmbed::default()
            .title("Progress Check Submitted")
            .description("Your progress check has been submitted.")
            .color(poise::serenity_prelude::Color::DARK_GREEN);
        let final_response = serenity::EditInteractionResponse::default()
            .embed(final_embed)
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("whatthefriksubmitit")
                    .emoji('✅')
                    .style(ButtonStyle::Success)
                    .label("Confirm")
                    .disabled(true),
                CreateButton::new("whatthefrikiswrong")
                    .emoji('❌')
                    .style(ButtonStyle::Danger)
                    .label("Nope")
                    .disabled(true),
            ])]);

        if press.data.custom_id == "whatthefrikiswrong" {
            println!("time to fix xd");
            edit_answers(&ctx, &press, &replyhandle, &mut prog_check, event).await?;
            break;
        } else if press.data.custom_id == "whatthefriksubmitit" {
            println!("{:?}", press.data.custom_id);
            press.edit_response(ctx, final_response).await?;
            break;
        }
    }

    Ok(prog_check)
}

async fn edit_answers(
    ctx: &Context<'_>,
    press: &serenity::ComponentInteraction,
    replyhandle: &poise::ReplyHandle<'_>,
    prog_check: &mut ProgressCheck,
    event: &str,
) -> Result<(), Error> {
    let interaction_handler = async {
        while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
            .author_id(ctx.author().id)
            .filter(move |press| press.data.custom_id.starts_with("whatthefrik"))
            .timeout(std::time::Duration::from_secs(100))
            .await
        {
            let _ = press.defer(ctx).await;
            if press.data.custom_id == "whatthefriksubmitit" {
                let final_embed = CreateEmbed::default()
                    .title("Progress Check Submitted")
                    .description("Your progress check has been submitted.")
                    .color(poise::serenity_prelude::Color::DARK_GREEN);
                let final_response = serenity::EditInteractionResponse::default()
                    .embed(final_embed)
                    .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
                        "whatthefriksubmitit",
                    )
                    .emoji('✅')
                    .style(ButtonStyle::Success)
                    .label("Confirm")
                    .disabled(true)])]);
                let _ = press.edit_response(ctx, final_response).await;
                return;
            }
        }
    };

    let other_task = async {
        while let Some(messages) = ctx
            .channel_id()
            .await_reply(ctx)
            .author_id(ctx.author().id)
            .timeout(std::time::Duration::from_secs(60))
            .await
        {
            let answer = messages.content;
            println!("{}", answer);
            if answer == "1" {
                prog_check.duration(
                    send_questions(
                        ctx,
                        replyhandle,
                        "How long did you meet with your partner for".to_string(),
                        &event.to_string(),
                    )
                    .await
                    .unwrap(),
                );
                let _ = send_confirmation(ctx, press, prog_check).await;
            } else if answer == "2" {
                prog_check.progress(
                    send_questions(
                        ctx,
                        replyhandle,
                        "What progress did you make on".to_string(),
                        &event.to_string(),
                    )
                    .await
                    .unwrap(),
                );
                let _ = send_confirmation(ctx, press, prog_check).await;
            } else if answer == "3" {
                prog_check.improvements(
                    send_questions(
                        ctx,
                        replyhandle,
                        "What improvements can you make for".to_string(),
                        &event.to_string(),
                    )
                    .await
                    .unwrap(),
                );
                let _ = send_confirmation(ctx, press, prog_check).await;
            } else {
                let _ = ctx.say(
                "Invalid input. Please type the number of the question you would like to change.",
            )
            .await;
            }
        }
    };
    tokio::select!(
        _ = interaction_handler => {},
        _ = other_task => {}
    );
    Ok(())
}

async fn send_confirmation(
    ctx: &Context<'_>,
    press: &serenity::ComponentInteraction,
    prog_check: &ProgressCheck,
) -> Result<(), Error> {
    let confirmation_embed = CreateEmbed::default()
        .title(format!("Confirm that your Progress Check for {} is correct: ", prog_check.event))
        .description(format!(
            "1. Duration: {}\n 2. Progress: {}\n 3. Improvements: {}\n",
            prog_check.duration, prog_check.progress, prog_check.improvements
        ))
        .color(poise::serenity_prelude::Color::DARK_ORANGE)
        .footer(CreateEmbedFooter::new(
            "If you would like to change any of the above answers, type the number of the question you would like to change after pressing ❌'.",
        ));

    let reply = CreateReply::default()
        .embed(confirmation_embed.clone())
        .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
            "whatthefriksubmitit",
        )
        .emoji('✅')
        .style(ButtonStyle::Success)
        .label("Confirm")])]);

    ctx.send(reply).await?;

    Ok(())
}

async fn send_questions<'a>(
    ctx: &Context<'a>,
    replyhandle: &poise::ReplyHandle<'a>,
    question: String,
    event: &String,
) -> Result<String, Error> {
    replyhandle
        .message()
        .await?
        .reply(ctx, format!("{} {}?", &question, &event))
        .await?;
    let mut answer = String::new();
    if let Some(messages) = ctx
        .channel_id()
        .await_reply(ctx)
        .author_id(ctx.author().id)
        .timeout(std::time::Duration::from_secs(60))
        .await
    {
        answer = messages.content;
    } else {
        return Err(Error::from("No answer provided"));
    }

    Ok(answer)
}
