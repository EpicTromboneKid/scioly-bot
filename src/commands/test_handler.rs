use crate::utils::{Context, Error};

pub mod testing {
    use crate::utils::Error;
    use poise::serenity_prelude::{self as serenity, Attachment, Context, FullEvent, Message};
    pub struct Test {
        year: u32,
        place: String,
        event: String,
        division: String,
        file: Vec<u8>,
        filetype: String,
        has_parts: bool,
        parts: u32,
        allotted_time: u32,
    }
    impl Test {
        pub fn start_test(&self) -> Result<(), Error> {
            todo!();
        }
        pub fn end_test(&self) -> Result<(), Error> {
            todo!();
        }
        pub async fn downloader(context: Context, message: Message) -> Result<Vec<String>, Error> {
            let mut filename_list: Vec<String> = vec![];
            for attachment in message.attachments {
                //
                filename_list.push(attachment.filename.to_owned());
                //
                let content = match attachment.download().await {
                    Ok(content) => content,
                    Err(_) => {
                        message
                            .channel_id
                            .say(&context, "Error downloading {attachment.filename:?}")
                            .await?;
                        continue;
                    }
                };
            }
            Ok(filename_list)
        }
        fn drive_uploader(&self) -> Result<(), Error> {
            todo!()
        }
    }
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("start", "end", "upload"),
    subcommand_required
)]
pub async fn test(_ctx: Context<'_>) -> Result<(), Error> {
    println!("ok no subcommand given but its ok");
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("start").await?;
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn end(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("end").await?;
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn upload(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("upload").await?;
    Ok(())
}
