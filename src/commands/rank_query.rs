use crate::utils::{Context, Error};
use chrono::{Datelike, Utc};
use std::{collections::HashMap, panic::panic_any};
/// Command to get the placement of an event at an invitational (can only be used as
/// a slash command).
///
/// `/rq aggie Lynbrook nCA Chem Lab` would find the placement at aggie 2024 for LHS Chem lab.
#[poise::command(track_edits, slash_command)]
// this is the rank query function, taking in 4 arguments: year, invy, school, event
pub async fn rq(
    ctx: Context<'_>,
    #[description = "Year of Invitational, defaults to the current year"] year: Option<u32>,
    #[description = "Invitational"] invy: Option<String>,
    #[description = "School of interest"] school: Option<String>,
    #[description = "State of Invitational"] state: Option<String>,
    #[description = "Event of interest (i.e. Chem Lab)"] event: Option<String>,
    #[description = "Division, defaults to Div. C"] division: Option<String>,
) -> Result<(), Error> {
    let arg_hash_map = HashMap::from([
        (0, "year"),
        (1, "invitational"),
        (2, "school"),
        (3, "state"),
        (4, "event"),
        (5, "division"),
    ]);

    let qyear = year.unwrap_or(Utc::now().year().try_into()?);
    let qinvy = invy
        .unwrap_or("-1".to_string())
        .to_string()
        .trim()
        .to_string();
    let qschool = school.unwrap_or("-1".to_string()).trim().to_string();
    let qstate = state.unwrap_or("-1".to_string()).trim().to_string();
    let qevent = event.unwrap_or("-1".to_string()).trim().to_string();
    let qdivision: String = division.unwrap_or("c".to_string()).to_string();
    let array = [&qyear.to_string(), &qinvy, &qschool, &qevent, &qdivision];
    let mut input = String::new();

    println!(
        "{} {} {} {} {}",
        &qyear, &qinvy, &qschool, &qevent, &qdivision
    );

    for (i, element) in array.iter().enumerate() {
        if element.contains("-1") {
            if let Some(arg) = arg_hash_map.get(&i) {
                println!("found an argument with -1 lol");
                input.push_str(arg);
                input.push(' ');
            }
        }
    }

    if !input.is_empty() {
        panic_any(format!("Provide the following arguments: {}", input));
    }

    let query = parse_file::Input::build_input(
        qyear.try_into()?,
        qinvy.clone().to_string(),
        qschool.clone(),
        qstate.clone(),
        qevent.clone(),
        qdivision.clone(),
    )?;

    let x = query.find_rank()?.to_string();
    let out_string = format!(
        "{} {}'s placement at {} {} is: {} :)",
        &qschool, &qevent, &qinvy, &qyear, &x
    );

    query.print_fields();
    println!("{x}");
    poise::say_reply(ctx, out_string).await?;
    Ok(())
}

mod input_fixing {
    use crate::commands::rank_query::parse_file::Input;
    use crate::utils::Error;
    use std::panic::panic_any;

    fn inv(invitational: String) -> Result<String, Error> {
        Ok(invitational)
    }
    fn state(state: String) -> Result<String, Error> {
        Ok(state)
    }

    fn event(event: String) -> Result<String, Error> {
        if event.to_lowercase().contains("chem") {
            Ok("Chemistry Lab".to_string())
        } else if event.parse() == Ok(-1) {
            panic_any("Event was not provided!");
        } else {
            Ok(event)
        }
    }

    fn school(school: String) -> Result<String, Error> {
        Ok(school)
    }

    pub fn fix_inputs(query: Input) -> Result<Input, Error> {
        let oinv = inv(query.qinv)?.trim().to_string();
        let oevent = event(query.qevent)?.trim().to_string();
        let oschool = school(query.qschool)?.trim().to_string();
        let ostate = state(query.qstate)?.trim().to_string();
        Ok(Input {
            qyear: query.qyear,
            qinv: oinv,
            qschool: oschool,
            qstate: ostate,
            qevent: oevent,
            qdiv: query.qdiv,
        })
    }
}

pub mod parse_file {
    use crate::commands::rank_query::input_fixing;
    use crate::commands::rank_query::Error;
    use poise::serenity_prelude::Error as serenError;
    use rust_search::SearchBuilder;
    use std::env::current_dir;
    use std::panic::*;
    use yaml_rust2::YamlLoader;

    use String;
    pub struct Input {
        pub qyear: i32,
        pub qinv: String,
        pub qschool: String,
        pub qstate: String,
        pub qevent: String,
        pub qdiv: String,
    }

    impl Input {
        pub fn build_input(
            year: i32,
            inv: String,
            school: String,
            state: String,
            event: String,
            div: String,
        ) -> Result<Input, Error> {
            let inq = Input {
                qyear: year,
                qinv: inv,
                qschool: school,
                qstate: state,
                qevent: event,
                qdiv: div,
            };

            let outq: Input = input_fixing::fix_inputs(inq)?;

            Ok(outq)
        }

        fn get_filepath(&self) -> Result<String, Error> {
            let current_dir = current_dir()?;
            let mut path = match current_dir.to_str() {
                Some(x) => x.to_string(),
                None => panic_any("path not found"),
            };

            println!("this is the path: {:?}", &path);
            path.push_str("/duosmium/data/results/");
            println!("this is the new path: {:?}", &path);
            let mut files: Vec<String> = SearchBuilder::default()
                .location(&path)
                .depth(1)
                .search_input(self.qyear.to_string())
                .build()
                .collect();
            files.remove(0);

            for fakefile in files {
                println!("this is the file! {:?}", &fakefile);
                let file: String = std::fs::read_to_string(&fakefile)?;
                let yaml_file = &YamlLoader::load_from_str(&file)?[0];
                if fakefile.contains(&self.qinv)
                    && yaml_file["Tournament"]["division"]
                        .as_str()
                        .expect("no division provided?")
                        .to_string()
                        .eq_ignore_ascii_case(&self.qdiv)
                {
                    println!("found file! {:?}", fakefile);
                    return Ok(file);
                }
            }
            Err(Into::into("DNE".to_string()))
        }

        fn find_school(&self) -> Result<(i64, yaml_rust2::Yaml), Error> {
            let school_number;
            let test_file = self.get_filepath()?;
            let docs = YamlLoader::load_from_str(&test_file).expect("Loading file didn't work");
            let doc = &docs[0];
            println!(
                "The first team in this file is: {:?}",
                &doc["Teams"][0]["school"].as_str().expect("oof")
            );
            let teams = doc["Teams"].clone().into_iter();
            println!("{:?}", &self.qschool);
            for i in teams {
                let school_at_i = &i["school"].clone().into_string().expect("didn't work.");

                if school_at_i
                    .as_str()
                    .to_lowercase()
                    .contains(&self.qschool.as_str().to_lowercase())
                {
                    school_number = i["number"].as_i64().expect("NaN");
                    println!("successfully found school_rank: {}", school_number);
                    return Ok((school_number, doc.clone()));
                } else {
                    println!("{:?}", &i["school"]);
                }
            }
            Err(Box::new(serenError::Other("couldn't find school")))
        }

        pub fn find_rank(&self) -> Result<i64, Error> {
            let (number, the_doc) = self.find_school()?;
            let event = &self.qevent;
            println!(
                "in find_rank now, the number, first team, and event is: {:?}, {:?}, {:?}",
                &number,
                &the_doc["Teams"][0]["school"]
                    .as_str()
                    .expect("could not find school in file :("),
                &event
            );
            let placings = the_doc["Placings"].clone().into_iter();
            for i in placings {
                let team_at_i = &i["team"].clone().into_i64().expect("1");
                let event_at_i = &i["event"].clone().into_string().expect("2");
                if number != -1
                    && team_at_i == &number
                    && event_at_i.as_str().eq_ignore_ascii_case(event)
                {
                    println!("{:?}", i["place"].clone().into_i64().expect("3"));
                    return Ok(i["place"].clone().into_i64().expect("3"));
                }
            }
            Err(Into::into("placing not found"))
        }

        pub fn print_fields(&self) {
            println!(
                "year: {}, invitational: {}, school: {}, event: {}",
                &self.qyear, &self.qinv, &self.qschool, &self.qevent
            );
        }
    }
}
