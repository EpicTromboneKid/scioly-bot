use google_docs1::hyper_rustls::HttpsConnector;
use google_docs1::hyper_util::client::legacy::connect::HttpConnector;
use mistralrs::Model;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type SharedModel = Arc<Mutex<Model>>;
pub type SciolyHubs<'a> = (
    &'a google_docs1::api::Docs<HttpsConnector<HttpConnector>>,
    &'a google_drive3::api::DriveHub<HttpsConnector<HttpConnector>>,
    &'a google_sheets4::api::Sheets<HttpsConnector<HttpConnector>>,
);
pub static MODEL: OnceCell<SharedModel> = OnceCell::const_new();

#[derive(Debug)]
pub struct Data {}

use user_handling::SciolyUser;

pub enum Perms {
    Viewer(),
    Commenter(),
    Editor(),
    Owner(),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Thing {
    pub users: Vec<SciolyUser>,
}

pub mod user_handling {
    use crate::utils::{Context, Error};

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct SciolyUser {
        pub userid: String,
        pub default_email: String,
        pub team: char,
        pub events: Vec<String>,
        pub officer: bool,
    }

    impl Default for SciolyUser {
        fn default() -> Self {
            Self {
                userid: String::from(""),
                default_email: String::from(""),
                team: 'z',
                events: Vec::new(),
                officer: false,
            }
        }
    }

    pub fn get_user_data(file_path: &str) -> Result<Vec<SciolyUser>, Error> {
        let data = std::fs::read_to_string(file_path).unwrap();
        let users: crate::utils::Thing = serde_json::from_str(&data).unwrap();
        Ok(users.users)
    }

    pub fn write_user_data(file_path: &str, users: Vec<SciolyUser>) -> Result<(), Error> {
        std::fs::write(
            file_path,
            serde_json::to_string(&crate::utils::Thing { users })?,
        )?;
        Ok(())
    }

    pub fn find_user(userid: &str) -> Result<SciolyUser, Error> {
        let users = get_user_data("userdata.json")?;
        for user in users {
            if user.userid == userid {
                return Ok(user);
            }
        }
        Err("User not found".into())
    }

    pub fn get_event_partners(
        event: &String,
        userid: &str,
        team: &char,
    ) -> Result<Vec<SciolyUser>, Error> {
        let mut partners = Vec::new();
        let users = get_user_data("userdata.json")?;
        for user in users {
            if &user.team == team && user.events.contains(event) && user.userid != userid {
                partners.push(user);
            }
        }
        Ok(partners)
    }

    pub fn get_officers_emails() -> Result<Vec<String>, Error> {
        let mut emails = Vec::new();
        let users = get_user_data("userdata.json")?;
        for user in users {
            if user.officer {
                emails.push(user.default_email);
            }
        }
        Ok(emails)
    }
    pub fn get_event_id_list(ctx: Context<'_>) -> Result<Vec<(String, String)>, Error> {
        let mut event_id_list = Vec::new();
        let event_list = match find_user(&ctx.author().id.to_string()) {
            Ok(user) => user.events,
            Err(_) => std::panic::panic_any(
                "No events found; please register with `/set_defaults`!, or check your roles in this server.",
            ),
        };

        for event in event_list {
            let event_id = format!("{}{}", &ctx.id(), &event);
            event_id_list.push((event, event_id));
        }
        Ok(event_id_list)
    }
}

pub mod server_handling {
    use crate::utils::Error;
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Server {
        pub server_id: String,
        pub server_name: String,
        pub server_email: String,
        pub tests_file_id: String,
        pub pc_file_id: String,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct AllData {
        pub servers: Vec<Server>,
    }

    pub fn get_server_data(file_path: &str) -> Result<AllData, Error> {
        let data = std::fs::read_to_string(file_path).unwrap();
        let server: AllData = serde_json::from_str(&data).unwrap();
        Ok(server)
    }

    pub fn write_server_data(file_path: &str, to_write: AllData) -> Result<(), Error> {
        std::fs::write(file_path, serde_json::to_string(&to_write)?)?;
        Ok(())
    }

    pub fn get_server(server_id: &str) -> Result<Server, Error> {
        let servers = get_server_data("serverdata.json")?;
        for server in servers.servers {
            if server.server_id == server_id {
                return Ok(server);
            }
        }
        Err("Server not found".into())
    }
}

pub mod events {
    use rust_fuzzy_search::fuzzy_search_sorted;
    use strum::Display;

    static EVENT_LIST: [&str; 30] = [
        "Air Trajectory",
        "Anatomy and Physiology",
        "Astronomy",
        "Bungee Drop",
        "Chemistry Lab",
        "Codebusters",
        "Crime Busters",
        "Disease Detectives",
        "Dynamic Planet",
        "Ecology",
        "Electric Vehicle",
        "Entomology",
        "Experimental Design",
        "Forensics",
        "Fossils",
        "Geologic Mapping",
        "Helicopter",
        "Materials Science",
        "Metric Mastery",
        "Microbe Mission",
        "Mission Possible",
        "Optics",
        "Potions and Poisons",
        "Reach For The Stars",
        "Road Scholar",
        "Robot Tour",
        "Scrambler",
        "Tower",
        "Wind Power",
        "Write It Do It",
    ];

    pub enum Division {
        B,
        C,
    }

    #[derive(Debug, Display)]
    pub enum Types {
        Build,
        Hybrid,
        Study,
    }

    pub fn extract_events(event_vec: &Vec<String>) -> Vec<String> {
        let mut events = Vec::new();
        for role in event_vec {
            for event in &EVENT_LIST {
                if rust_fuzzy_search::fuzzy_compare(role, event) > 0.9 {
                    events.push(event.to_string());
                }
            }
        }
        events
    }

    pub fn find_closest_event_name(in_event: String) -> Result<String, crate::utils::Error> {
        let events = fuzzy_search_sorted(&in_event, &EVENT_LIST);
        if &in_event == "widi" {
            Ok("Write It Do It".to_string())
        } else {
            //for (event, score) in &sorted_vec {
            //    println!("{:?} {:?}", event, score);
            //}
            Ok(events[0].0.to_string())
        }
    }

    pub fn match_event_type(event: &str) -> Types {
        match event {
            "Air Trajectory" => Types::Build,
            "Anatomy and Physiology" => Types::Study,
            "Astronomy" => Types::Study,
            "Bungee Drop" => Types::Build,
            "Chemistry Lab" => Types::Hybrid,
            "Codebusters" => Types::Study,
            "Crime Busters" => Types::Study,
            "Disease Detectives" => Types::Study,
            "Dynamic Planet" => Types::Study,
            "Ecology" => Types::Study,
            "Electric Vehicle" => Types::Build,
            "Entomology" => Types::Study,
            "Experimental Design" => Types::Hybrid,
            "Forensics" => Types::Hybrid,
            "Fossils" => Types::Study,
            "Geologic Mapping" => Types::Study,
            "Helicopter" => Types::Build,
            "Materials Science" => Types::Hybrid,
            "Metric Mastery" => Types::Study,
            "Microbe Mission" => Types::Study,
            "Mission Possible" => Types::Build,
            "Optics" => Types::Hybrid,
            "Potions and Poisons" => Types::Study,
            "Reach For The Stars" => Types::Study,
            "Road Scholar" => Types::Study,
            "Robot Tour" => Types::Build,
            "Scrambler" => Types::Build,
            "Tower" => Types::Build,
            "Wind Power" => Types::Build,
            "Write It Do It" => Types::Hybrid,
            _ => Types::Study,
        }
    }
}
