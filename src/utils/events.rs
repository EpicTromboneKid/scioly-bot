use crate::utils::Error;
use rust_fuzzy_search::fuzzy_search_sorted;

static EVENT_LIST: [&str; 23] = [
    "Anatomy and Physiology",
    "Astronomy",
    "Boomilever",
    "Bungee Drop",
    "Chemistry Lab",
    "Circuit Lab",
    "Codebusters",
    "Designer Genes",
    "Disease Detectives",
    "Dynamic Planet",
    "Electric Vehicle",
    "Engineering CAD",
    "Entomology",
    "Experimental Design",
    "Forensics",
    "Helicopter",
    "Hovercraft",
    "Machines",
    "Materials Science",
    "Remote Sensing",
    "Robot Tour",
    "Rocks and Minerals",
    "Water Quality",
];

pub enum Division {
    B,
    C,
}

#[derive(Debug)]
pub enum Types {
    Build,
    Hybrid,
    Study,
}

impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Types::Build => write!(f, "Build"),
            Types::Hybrid => write!(f, "Hybrid"),
            Types::Study => write!(f, "Study"),
        }
    }
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

pub fn find_closest_event_name(in_event: String) -> Result<String, Error> {
    let events = fuzzy_search_sorted(&in_event, &EVENT_LIST);
    if &in_event == "widi" {
        Ok("Write It Do It".to_string())
    } else if &in_event == "r&m" {
        Ok("Rocks and Minerals".to_string())
    } else {
        Ok(events[0].0.to_string())
    }
}

pub fn match_event_type(event: &str) -> Types {
    match event {
        "Anatomy and Physiology" => Types::Study,
        "Astronomy" => Types::Study,
        "Boomilever" => Types::Build,
        "Bungee Drop" => Types::Build,
        "Chemistry Lab" => Types::Hybrid,
        "Circuit Lab" => Types::Hybrid,
        "Codebusters" => Types::Study,
        "Designer Genes" => Types::Study,
        "Disease Detectives" => Types::Study,
        "Dynamic Planet" => Types::Study,
        "Electric Vehicle" => Types::Build,
        "Engineering CAD" => Types::Hybrid,
        "Entomology" => Types::Study,
        "Experimental Design" => Types::Hybrid,
        "Forensics" => Types::Hybrid,
        "Helicopter" => Types::Build,
        "Hovercraft" => Types::Build,
        "Machines" => Types::Hybrid,
        "Materials Science" => Types::Hybrid,
        "Remote Sensing" => Types::Study,
        "Robot Tour" => Types::Build,
        "Rocks and Minerals" => Types::Study,
        "Water Quality" => Types::Hybrid,
        _ => Types::Study,
    }
}
