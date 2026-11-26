use crate::utils::{Context, Error, Thing};

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
    let data = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read user data from {}: {}", file_path, e))?;
    let users: Thing = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse user data JSON: {}", e))?;
    Ok(users.users)
}

pub fn write_user_data(file_path: &str, users: Vec<SciolyUser>) -> Result<(), Error> {
    std::fs::write(
        file_path,
        serde_json::to_string(&Thing { users })?,
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
        Err(_) => {
            return Err("No events found; please register with `/reg` first, or check your roles in this server.".into());
        }
    };

    for event in event_list {
        let event_id = format!("{}{}", &ctx.id(), &event);
        event_id_list.push((event, event_id));
    }
    Ok(event_id_list)
}
