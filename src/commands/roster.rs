use crate::commands::{google, pc};
use crate::secrets;
use crate::utils::{self, Context, Error};
use poise::serenity_prelude::{self as serenity, CreateMessage, MessageFlags, UserId};

use poise::CreateReply;
use server_handling::get_server;

use crate::utils::*;

#[poise::command(slash_command, prefix_command)]
pub async fn roster_sync(ctx: Context<'_>) -> Result<(), Error> {
    let scioly_sheets = google::gsheets::instantiate_hub(secrets::servicefilename()).await?;
    //let roster_file_id = get_server(&ctx.guild_id().unwrap().to_string())?.roster_file_id;
    let tabs = scioly_sheets
        .spreadsheets()
        .get("13fwNMRKeqMEafSMgg6VBokJhSNCNIjD6eHDosfZd1GA")
        .doit()
        .await?;
    println!("Tabs: {:?}", tabs.1.sheets);
    Ok(())
}
