use crate::utils::{self, Context, Error};

use poise::serenity_prelude::EditInteractionResponse;
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
        .title("PC Aborted")
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

pub async fn pc_event_handling(ctx: Context<'_>, event: &String) -> Result<(), Error> {
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

    let confirmation_embed = CreateEmbed::default()
        .title("Confirm that the Progress Check is correct:")
        .description(format!(
            "Event: {}, Duration: {}, Progress: {}, Improvements: {}",
            prog_check.event, prog_check.duration, prog_check.progress, prog_check.improvements
        ))
        .color(poise::serenity_prelude::Color::DARK_ORANGE);
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
        .await
    {
        let final_response = EditInteractionResponse::default()
            .embed(confirmation_embed.clone())
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
        let value = press.edit_response(ctx, final_response);
        value.await?;
    }
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
