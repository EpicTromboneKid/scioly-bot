pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub _votes: Mutex<HashMap<String, u32>>,
}

use std::{collections::HashMap, sync::Mutex};

pub mod events {
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
}
