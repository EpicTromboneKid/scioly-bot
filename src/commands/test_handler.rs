use crate::utils::{Context, Error};

pub mod testing {
    use crate::utils::{Context, Error};
    use poise::serenity_prelude::{self as serenity, Attachment, FullEvent, Message};
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
        pub fn downloader(attach_vec: Vec<Attachment>) -> Result<Vec<String>, Error> {
            let mut filename_list: Vec<String> = vec![];
            for attachment in attach_vec {
                let filetype = match attachment.content_type.clone() {
                    Some(x) => Ok(x),
                    None => Err("no filetype given"),
                };
                println!(
                    "name: {}, url: {}, content_type: {}",
                    attachment.filename, attachment.proxy_url, filetype?
                );
                filename_list.push(attachment.filename);
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
