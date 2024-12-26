use mistralrs::{
    IsqType, PagedAttentionMetaBuilder, Response, TextMessageRole, TextMessages, TextModelBuilder,
};

#[poise::command(prefix_command, slash_command, track_edits, rename = "ask")]
pub async fn ai(
    ctx: crate::utils::Context<'_>,
    #[rest] prompt: String,
) -> Result<(), crate::utils::Error> {
    ctx.defer_or_broadcast().await?;
    let model = TextModelBuilder::new("microsoft/Phi-3.5-mini-instruct".to_string())
        .with_isq(IsqType::Q4_1)
        .with_logging()
        .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?
        .build()
        .await?;

    println!("Prompt: {}", prompt);

    let messages = TextMessages::new().add_message(TextMessageRole::User, prompt);

    let mut stream = model.stream_chat_request(messages).await?;

    let mut output = String::new();

    while let Some(chunk) = stream.next().await {
        if let Response::Chunk(chunk) = chunk {
            output.push_str(&chunk.choices[0].delta.content);
            println!("{}", &chunk.choices[0].delta.content);
        } else {
            return Err("Error while stream parsing".into());
        }
    }
    ctx.say(&output).await?;
    Ok(())
}
