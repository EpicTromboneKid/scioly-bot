pub type Error = Box<dyn std::error::Error + Send + Sync>;

mod input_fixing {
    use std::panic::panic_any;

    use crate::{parse_file::Input, Error};

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
            panic_any(format!("Event was not provided!"));
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
    use crate::input_fixing;
    use crate::Error;
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
                .search_input(&self.qyear.to_string())
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
                    && event_at_i.as_str().eq_ignore_ascii_case(&event)
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
