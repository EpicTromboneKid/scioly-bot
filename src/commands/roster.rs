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
	let tab_array= tabs.1.sheets.unwrap();
    let _ = &ctx.say(format!("Tabs: 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}", tab_array[0].properties.as_ref().unwrap().title, tab_array[1].properties.as_ref().unwrap().title, tab_array[2].properties.as_ref().unwrap().title, tab_array[3].properties.as_ref().unwrap().title)).await?;
    Ok(())
}
