use mistralrs::{RequestBuilder, Response, TextMessageRole, TextModelBuilder};
use poise::{CreateReply, ReplyHandle};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn initialize_model() -> Result<crate::utils::SharedModel, crate::utils::Error> {
    let model = TextModelBuilder::new("microsoft/Phi-3.5-mini-instruct".to_string())
        .with_logging()
        .with_isq(mistralrs::IsqType::Q6K)
        .with_dtype(mistralrs::ModelDType::F16)
        .build()
        .await?;
    println!("model after");

    Ok(Arc::new(Mutex::new(model)))
}

#[poise::command(prefix_command, slash_command, rename = "ask")]
pub async fn ai(
    ctx: crate::utils::Context<'_>,
    #[rest] prompt: String,
) -> Result<(), crate::utils::Error> {
    let channelid = ctx.channel_id();
    channelid.broadcast_typing(ctx).await?;
    let model = crate::utils::MODEL.get().unwrap().lock().await;
    let messages = RequestBuilder::new()
        .add_message(
            TextMessageRole::System,
            "Limit your responses to 500 characters. You are helping high school students with general science questions.",
        )
        .add_message(TextMessageRole::User, &prompt);

    println!("Prompt: {}", prompt);

    // THIS IS THE NON-STREAMING VERSION
    //let response = model.send_chat_request(messages).await?;
    //
    //ctx.say(response.choices[0].message.content.as_ref().unwrap())
    //    .await?;

    let mut stream = model.stream_chat_request(messages).await?;

    let mut output = String::new();
    let mut reply_handle: Option<ReplyHandle> = None;

    while let Some(chunk) = stream.next().await {
        channelid.broadcast_typing(ctx).await?;
        if let Response::Chunk(chunk) = chunk {
            if output.len() > 900 {
                output.push_str("REPLY TRUNCATED; TOO LARGE");
                break;
            }
            if let Some(ref replyhandle) = reply_handle {
                output.push_str(chunk.choices[0].delta.content.as_str());
                let builder = CreateReply::default().content(&output);
                replyhandle.edit(ctx, builder).await?;
                continue;
            }
            output.push_str(chunk.choices[0].delta.content.as_str());
            reply_handle = Some(ctx.say(&output).await?);
        }
    }

    output.push_str(
        "
       
*AI-generated content. Always make sure to verify with trusted sources.*",
    );
    if let Some(ref rh) = reply_handle {
        let builder = CreateReply::default().content(output);
        rh.edit(ctx, builder).await?;
    }
    Ok(())
}
