pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub _votes: Mutex<HashMap<String, u32>>,
}

use std::{collections::HashMap, sync::Mutex};

mod events {
    enum Events {
        AirTrajectory,
        AnatomyAndPhysiology,
        Astronomy,
        BungeeDrop,
        ChemistryLab,
        Codebusters,
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
        MicrobeMission,
        Optics,
        RobotTour,
        Tower,
        WindPower,
        WriteItDoIt,
    }
}
