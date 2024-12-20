pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub _votes: Mutex<HashMap<String, u32>>,
}

use std::{collections::HashMap, sync::Mutex};

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
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct SciolyUser {
        pub username: String,
        pub default_email: String,
        pub team: char,
        pub events: Vec<String>,
    }

    impl Default for SciolyUser {
        fn default() -> Self {
            Self {
                username: String::from(""),
                default_email: String::from(""),
                team: 'z',
                events: Vec::new(),
            }
        }
    }

    pub fn get_user_data(file_path: &str) -> Vec<SciolyUser> {
        let data = std::fs::read_to_string(file_path).unwrap();
        let users: crate::utils::Thing = serde_json::from_str(&data).unwrap();
        users.users
    }

    pub fn write_user_data(
        file_path: &str,
        users: Vec<SciolyUser>,
    ) -> Result<(), crate::utils::Error> {
        std::fs::write(
            file_path,
            serde_json::to_string(&crate::utils::Thing { users })?,
        )?;
        Ok(())
    }

    pub fn find_user(username: &str) -> Option<String> {
        let users = get_user_data("userdata.json");
        for user in &users {
            println!("{:?}", user);
            if user.username == username {
                println!("found user; email: {:?}", &user.default_email);
                return Some(user.default_email.clone());
            }
        }
        None
    }
}

pub mod events {
    use rust_fuzzy_search::fuzzy_search_sorted;

    pub enum Division {
        B,
        C,
    }

    pub fn find_closest_event_name(in_event: String) -> Result<String, crate::utils::Error> {
        let event_list = [
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

        let sorted_vec = fuzzy_search_sorted(&in_event, &event_list);
        if &in_event == "widi" {
            Ok("Write It Do It".to_string())
        } else {
            //for (event, score) in &sorted_vec {
            //    println!("{:?} {:?}", event, score);
            //}
            Ok(sorted_vec[0].0.to_string())
        }
    }
}
