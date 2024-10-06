pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub _votes: Mutex<HashMap<String, u32>>,
}

use std::{collections::HashMap, sync::Mutex};

pub mod events {
    use rust_fuzzy_search::fuzzy_search_sorted;
    pub enum Events {
        AirTrajectory,
        AnatomyAndPhysiology,
        Astronomy,
        BungeeDrop,
        ChemistryLab,
        Codebusters,
        CrimeBusters,
        DiseaseDetectives,
        DynamicPlanet,
        Ecology,
        ElectricVehicle,
        Entomology,
        ExperimentalDesign,
        Forensics,
        Fossils,
        GeologicMapping,
        Helicopter,
        MaterialsScience,
        MetricMastery,
        MicrobeMission,
        MissionPossible,
        Optics,
        PotionsAndPoisons,
        ReachForTheStars,
        RoadScholar,
        RobotTour,
        Scrambler,
        Tower,
        WindPower,
        WriteItDoIt,
    }
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
            for (event, score) in &sorted_vec {
                println!("{:?} {:?}", event, score);
            }
            Ok(sorted_vec[0].0.to_string())
        }
    }
}
