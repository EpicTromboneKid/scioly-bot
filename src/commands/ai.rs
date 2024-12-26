use mistralrs::{
    IsqType, PagedAttentionMetaBuilder, RequestBuilder, TextMessageRole, TextModelBuilder,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn initialize_model() -> Result<crate::utils::SharedModel, crate::utils::Error> {
    let model = TextModelBuilder::new("microsoft/Phi-3.5-mini-instruct".to_string())
        .with_isq(IsqType::Q4_1)
        .with_logging()
        .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?
        .build()
        .await?;

    Ok(Arc::new(Mutex::new(model)))
}

#[poise::command(prefix_command, slash_command, rename = "ask")]
pub async fn ai(
    ctx: crate::utils::Context<'_>,
    #[rest] mut prompt: String,
) -> Result<(), crate::utils::Error> {
    ctx.defer_or_broadcast().await?;

    let model = crate::utils::MODEL.get().unwrap().lock().await;
    let messages = RequestBuilder::new()
        .add_message(
            TextMessageRole::System,
            "You are aiding high school students with general science questions.",
        )
        .add_message(TextMessageRole::User, &prompt);

    println!("Prompt: {}", prompt);
    println!("sending request");

    let response = model.send_chat_request(messages).await?;

    let output = match response.choices[0].message.content.as_ref().unwrap().len() > 1000 {
        true => response.choices[0]
            .message
            .content
            .as_ref()
            .unwrap()
            .get(0..1000)
            .unwrap(),
        false => response.choices[0].message.content.as_ref().unwrap(),
    };

    println!("Output: {}", output);

    ctx.say(output).await?;
    Ok(())
}
