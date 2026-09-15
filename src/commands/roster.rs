use std::option::Option::None;
use poise::serenity_prelude::{self as serenity};

use crate::commands::google;
use crate::secrets;
use crate::utils::server_handling::get_server;
use crate::utils::user_handling::SciolyUser;
use crate::utils::{Context, Error};
use rust_fuzzy_search as fuzzy;

use crate::utils::*;

pub async fn roster_sync(ctx: Context<'_>, name: String, id: String, officer: bool, default_email: String) -> Result<SciolyUser, Error> {
    let scioly_sheets = google::gsheets::instantiate_hub(secrets::servicefilename()).await?;
    let roster_file_id = get_server(&ctx.guild_id().unwrap().to_string())?.roster_file_id;
    let tabs = scioly_sheets
        .spreadsheets()
        .get(&roster_file_id)
        .doit()
        .await?;
    let tab_array = tabs.1.sheets.expect("no tabs in ss");
    println!("{:?}", tab_array[0].data);
    println!("Tabs: 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}", tab_array[0].properties.as_ref().unwrap().title, tab_array[1].properties.as_ref().unwrap().title, tab_array[2].properties.as_ref().unwrap().title, tab_array[3].properties.as_ref().unwrap().title);

    let mut all_entries: Vec<(String, char, Vec<String>)> = vec![];

    for tab in tab_array {
        let tab_title = tab.properties.as_ref().unwrap().title.as_ref().expect("not team?").clone();
        let title = tab_title.trim();
		println!("{title}");

        let range_query = format!("{}!A4:H18", title);
        let sheet_data = scioly_sheets
            .spreadsheets()
            .values_batch_get(&roster_file_id)
            .add_ranges(&range_query)
            .doit()
            .await?;

		let trimmed_title = title.replace("Team", "");

		let team_char = if trimmed_title.to_lowercase().contains("alternates") {
            'd'
        } else if trimmed_title.to_lowercase().contains('a') {
            'A'
        } else if trimmed_title.to_lowercase().contains('b') {
            'B'
        } else if trimmed_title.to_lowercase().contains('c') {
            'C'
        } else {
            'z' 
        };

        if let Some(value_ranges) = sheet_data.1.value_ranges {
            if let Some(rows) = value_ranges[0].values.as_ref() {
                for row in rows {
                    if row.is_empty() {
                        continue;
                    }

                    let candidate_name = row[0].to_string().trim().replace('"', "");
                    if candidate_name.is_empty() {
                        continue;
                    }

                    let mut events_list = Vec::new();
                    for cell in row.iter().skip(2) {
                        let event_name = cell.to_string().trim().replace('"', "");
                        if !event_name.is_empty() {
                            events_list.push(event_name);
                        }
                    }

                    println!("name: {}, team: {}, events: {:?}", candidate_name, team_char, events_list);
                    all_entries.push((candidate_name, team_char, events_list));
                }
            }
        }
    }

    let threshold = 0.85f32; 
    let mut best_match: Option<(String, char, Vec<String>)> = std::option::Option::None;
    let mut highest_score = threshold;

    let search_query = name.trim().to_lowercase();

    for (candidate_name, team_char, events_list) in all_entries {
        let candidate_lower = candidate_name.to_lowercase();
        let score = fuzzy::fuzzy_compare(&search_query, &candidate_lower);
        
        if score > highest_score {
            highest_score = score;
            best_match = Some((candidate_name, team_char, events_list));
        }
    }

    match best_match {
        Some((found_name, team_char, events_list)) => {
            let reply = poise::CreateReply::default()
                .embed(
                    serenity::CreateEmbed::default()
                        .title("Roster Match Confirmation")
                        .description(format!("Is your name **{}**?", found_name))
                        .color(serenity::Colour::GOLD)
                )
                .components(vec![
                    serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new("yes")
                            .label("Yes")
                            .style(serenity::ButtonStyle::Success),
                        serenity::CreateButton::new("NEIN")
                            .label("No")
                            .style(serenity::ButtonStyle::Danger),
                    ])
                ]);

            let sent_message = ctx.send(reply).await?;

            let author_id = ctx.author().id;
            let interaction = match sent_message
                .message()
                .await
                .unwrap()
                .await_component_interaction(ctx)
                .author_id(author_id)
                .timeout(std::time::Duration::from_secs(30))
                .await 
            {
                Some(x) => x,
                None => {
                    let _ = sent_message
                        .edit(ctx, poise::CreateReply::default().content("⏳ Registration timed out.").components(vec![]))
                        .await;
                    return Err("Registration timed out.".into());
                }
            };

            if interaction.data.custom_id == "yes" {
                interaction
                    .create_response(
                        ctx.http(), 
                        serenity::CreateInteractionResponse::UpdateMessage(
                            serenity::CreateInteractionResponseMessage::default()
                                .content(format!("✅ Confirmed! Registering as **{}**...", found_name))
                                .embeds(vec![])
                                .components(vec![])
                        )
                    )
                    .await?;

                Ok(SciolyUser { 
                    userid: id, 
                    default_email, 
                    team: team_char, 
                    events: events_list, 
                    officer 
                })
            } else {
                interaction
                    .create_response(
                        ctx.http(), 
                        serenity::CreateInteractionResponse::UpdateMessage(
                            serenity::CreateInteractionResponseMessage::default()
                                .content("❌ Registration cancelled. Please check your spelling and try again.")
                                .embeds(vec![])
                                .components(vec![])
                        )
                    )
                    .await?;

                Err("Registration cancelled by user.".into())
            }
        }
        std::option::Option::None => {
            Err(format!("User '{}' was not found in the roster.", name).into())
        }
    }
}